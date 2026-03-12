//! Method call handlers for MIR builder
//!
//! This module contains specialized handlers for different types of method calls,
//! following the Single Responsibility Principle.

use crate::ast::ASTNode;
use crate::mir::builder::builder_calls::CallTarget;
use crate::mir::builder::calls::function_lowering;
use crate::mir::builder::{MirBuilder, ValueId};
use crate::mir::{MirInstruction, MirType, TypeOpKind};

/// Me-call 専用のポリシー箱。
///
/// - 責務:
///   - me.method(...) を「インスタンス呼び出し」か「static メソッド呼び出し」か判定する。
///   - static box 文脈で実体のない receiver を生まないように、静的メソッド降下にフォールバックする。
struct MeCallPolicyBox;

impl MeCallPolicyBox {
    fn resolve_me_call(
        builder: &mut MirBuilder,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        // Instance box: prefer enclosing box method (lowered function) if存在
        let enclosing_cls: Option<String> = builder
            .scope_ctx
            .current_function
            .as_ref()
            .and_then(|f| f.signature.name.split('.').next().map(|s| s.to_string()));

        if let Some(cls) = enclosing_cls.as_ref() {
            let arg_values = builder.build_call_args(arguments)?;
            let arity = arg_values.len();
            let fname = function_lowering::generate_method_function_name(cls, method, arity);
            if let Some(ref module) = builder.current_module {
                if let Some(func) = module.functions.get(&fname) {
                    // Decide whether this lowered function expects an implicit receiver.
                    // Instance methods: params[0] is Box(box_name)
                    // Static methods:   params[0] is non-Box or params.is_empty()
                    let params = &func.signature.params;
                    let is_instance_method =
                        !params.is_empty() && matches!(params[0], MirType::Box(_));

                    // Expected argument count from signature (including receiver for instance)
                    let expected_params = params.len();
                    let provided_static = arg_values.len();
                    let provided_instance = arg_values.len() + 1;

                    // Build call_args based on method kind
                    let call_args: Vec<ValueId> = if is_instance_method {
                        // Instance method: prepend 'me' receiver
                        if expected_params != provided_instance {
                            if crate::config::env::builder_me_call_arity_strict() {
                                return Err(format!(
                                    "[me-call] arity mismatch (instance): {}: declared {} params, got {} args(+me)",
                                    fname, expected_params, provided_instance
                                ));
                            } else if crate::config::env::builder_static_call_trace() {
                                crate::runtime::get_global_ring0().log.warn(&format!(
                                    "[me-call] arity mismatch (instance): {}: declared {} params, got {} args(+me)",
                                    fname, expected_params, provided_instance
                                ));
                            }
                        }
                        let me_id = super::stmts::variable_stmt::build_me_expression(builder)?;
                        let mut v = Vec::with_capacity(provided_instance);
                        v.push(me_id);
                        v.extend(arg_values.into_iter());
                        v
                    } else {
                        // Static method: no receiver
                        if expected_params != provided_static {
                            if crate::config::env::builder_me_call_arity_strict() {
                                return Err(format!(
                                    "[me-call] arity mismatch (static): {}: declared {} params, got {} args",
                                    fname, expected_params, provided_static
                                ));
                            } else if crate::config::env::builder_static_call_trace() {
                                crate::runtime::get_global_ring0().log.warn(&format!(
                                    "[me-call] arity mismatch (static): {}: declared {} params, got {} args",
                                    fname, expected_params, provided_static
                                ));
                            }
                        }
                        arg_values
                    };

                    let dst = builder.next_value_id();
                    // Emit as unified global call to lowered function
                    builder.emit_unified_call(
                        Some(dst),
                        CallTarget::Global(fname.clone()),
                        call_args,
                    )?;
                    builder.annotate_call_result_from_func_name(dst, &fname);
                    return Ok(Some(dst));
                }
            }

            // Fallback 1: if `me` is bound, keep instance semantics.
            // This avoids silently turning `me.method(...)` into a static call.
            if let Ok(me_id) = super::stmts::variable_stmt::build_me_expression(builder) {
                let dst =
                    builder.handle_standard_method_call(me_id, method.to_string(), arguments)?;
                return Ok(Some(dst));
            }

            // Fallback 2: static helper context (no bound `me`) keeps legacy static lowering.
            // This path is mainly for static-box helper code where receiver is intentionally absent.
            let static_dst = builder.handle_static_method_call(cls, method, arguments)?;
            return Ok(Some(static_dst));
        }

        Ok(None)
    }
}

impl MirBuilder {
    /// Handle static method calls: BoxName.method(args)
    pub(super) fn handle_static_method_call(
        &mut self,
        box_name: &str,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<ValueId, String> {
        if crate::config::env::joinir_dev::debug_enabled() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[handle_static_method_call] ENTRY: box_name={} method={}",
                box_name, method
            ));
        }

        // Build argument values
        let arg_values = self.build_call_args(arguments)?;

        // Compose lowered function name: BoxName.method/N
        let func_name = format!("{}.{}/{}", box_name, method, arg_values.len());
        let dst = self.next_value_id();

        if crate::config::env::builder_static_call_trace() {
            crate::runtime::get_global_ring0()
                .log
                .info(&format!("[builder] static-call {}", func_name));
        }

        // Emit unified global call to the static-lowered function (module-local)
        self.emit_unified_call(Some(dst), CallTarget::Global(func_name), arg_values)?;
        Ok(dst)
    }

    /// Handle TypeOp method calls: value.is("Type") and value.as("Type")
    pub(super) fn handle_typeop_method(
        &mut self,
        object_value: ValueId,
        method: &str,
        type_name: &str,
    ) -> Result<ValueId, String> {
        let mir_ty = Self::parse_type_name_to_mir(type_name);
        let dst = self.next_value_id();
        let op = if method == "is" {
            TypeOpKind::Check
        } else {
            TypeOpKind::Cast
        };

        self.emit_instruction(MirInstruction::TypeOp {
            dst,
            op,
            value: object_value,
            ty: mir_ty,
        })?;

        Ok(dst)
    }

    /// Check if this is a TypeOp method call
    #[allow(dead_code)]
    pub(super) fn is_typeop_method(method: &str, arguments: &[ASTNode]) -> Option<String> {
        if (method == "is" || method == "as") && arguments.len() == 1 {
            Self::extract_string_literal(&arguments[0])
        } else {
            None
        }
    }

    /// Handle me.method() calls within static box context
    pub(super) fn handle_me_method_call(
        &mut self,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        MeCallPolicyBox::resolve_me_call(self, method, arguments)
    }

    /// Handle standard Box/Plugin method calls (fallback)
    pub(super) fn handle_standard_method_call(
        &mut self,
        object_value: ValueId,
        method: String,
        arguments: &[ASTNode],
    ) -> Result<ValueId, String> {
        if crate::config::env::joinir_dev::debug_enabled() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[handle_standard_method_call] ENTRY: method={} object=%{}",
                method, object_value.0
            ));
        }

        // Phase 285A0.1: WeakRef.weak_to_strong() → WeakRef(Load)
        // SSOT: docs/reference/language/lifecycle.md:179 - weak_to_strong() returns Box | null
        if method == "weak_to_strong" && arguments.is_empty() {
            return self.emit_weak_load(object_value);
        }

        // Phase 285A0.1: upgrade() is deprecated - Fail-Fast
        if method == "upgrade" && arguments.is_empty() {
            return Err("WeakRef uses weak_to_strong(), not upgrade()".to_string());
        }

        // Build argument values
        let arg_values = self.build_call_args(arguments)?;

        // Receiver class hintは emit_unified_call 側で起源/型から判断する（重複回避）
        // 統一経路: emit_unified_call に委譲（RouterPolicy と rewrite::* で安定化）
        let dst = self.next_value_id();
        self.emit_unified_call(
            Some(dst),
            CallTarget::Method {
                box_type: None,
                method,
                receiver: object_value,
            },
            arg_values,
        )?;
        Ok(dst)
    }
}
