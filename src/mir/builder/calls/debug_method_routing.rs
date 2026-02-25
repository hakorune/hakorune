//! 🎯 箱理論: Debug/REPL/MIR method routing
//!
//! 責務: 開発専用・診断用のメソッド呼び出し処理
//! - trace_method_call_if_enabled: メソッド呼び出しトレース
//! - trace_receiver_if_enabled: レシーバートレース
//! - try_build_mir_debug_method_call: __mir__.log/__mir__.mark 処理
//! - try_build_repl_method_call: __repl.get/__repl.set 処理
//! - try_build_mir_debug_call: MIR デバッグ命令生成

use super::super::{EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, LiteralValue};

impl MirBuilder {
    /// Trace method call if NYASH_STATIC_CALL_TRACE=1
    pub(super) fn trace_method_call_if_enabled(&self, object: &ASTNode, method: &str) {
        if !crate::config::env::builder_static_call_trace() {
            return;
        }
        let kind = match object {
            ASTNode::Variable { .. } => "Variable",
            ASTNode::FieldAccess { .. } => "FieldAccess",
            ASTNode::This { .. } => "This",
            ASTNode::Me { .. } => "Me",
            _ => "Other",
        };
        crate::runtime::get_global_ring0().log.info(&format!("[builder] method-call object kind={} method={}", kind, method));
    }

    /// Try to build __mir__.log() or __mir__.mark() method call
    pub(super) fn try_build_mir_debug_method_call(
        &mut self,
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        let ASTNode::Variable { name: obj_name, .. } = object else {
            return Ok(None);
        };
        if obj_name != "__mir__" {
            return Ok(None);
        }
        self.try_build_mir_debug_call(method, arguments)
    }

    /// Phase 288.1: REPL session variable bridge
    /// Transform __repl.get/set → ExternCall("__repl", "get/set", args)
    pub(super) fn try_build_repl_method_call(
        &mut self,
        object: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        let ASTNode::Variable { name: obj_name, .. } = object else {
            return Ok(None);
        };
        if obj_name != "__repl" {
            return Ok(None);
        }

        // Only handle get/set methods
        if method != "get" && method != "set" {
            return Err(format!("__repl.{} is not supported. Only __repl.get and __repl.set are allowed.", method));
        }

        // Build argument values
        let arg_values = self.build_call_args(arguments)?;

        // Emit ExternCall instruction
        let dst = self.next_value_id();
        self.emit_extern_call_with_effects(
            "__repl",
            method,
            arg_values,
            Some(dst),
            EffectMask::PURE, // get/set are pure from MIR perspective
        )?;

        Ok(Some(dst))
    }

    /// Dev-only: __mir__.log / __mir__.mark → MirInstruction::Debug 列への変換
    ///
    /// 構文:
    ///   __mir__.log("label", v1, v2, ...)
    ///   __mir__.mark("label")  // label-only marker
    ///
    /// - 第一引数は String リテラル想定（それ以外はこのハンドラをスキップして通常の解決に回す）。
    /// - 戻り値は Void 定数の ValueId（式コンテキストでも型破綻しないようにするため）。
    pub(super) fn try_build_mir_debug_call(
        &mut self,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<ValueId>, String> {
        if method != "log" && method != "mark" {
            return Ok(None);
        }

        if arguments.is_empty() {
            return Err("__mir__.log/__mir__.mark requires at least a label argument".to_string());
        }

        // 第一引数は String リテラルのみ対応（それ以外は通常経路にフォールバック）
        let label = match &arguments[0] {
            ASTNode::Literal {
                value: LiteralValue::String(s),
                ..
            } => s.clone(),
            _ => {
                // ラベルがリテラルでない場合はこのハンドラをスキップし、通常の static box 解決に任せる
                return Ok(None);
            }
        };

        // 残りの引数を評価して ValueId を集める
        let mut values: Vec<ValueId> = Vec::new();
        if method == "log" {
            for (arg_idx, arg) in arguments[1..].iter().enumerate() {
                let v = self.build_expression(arg.clone())?;

                // Debug-only observation: check for undefined ValueId immediately after build
                if crate::config::env::joinir_dev::debug_enabled() {
                    if let Some(func) = self.scope_ctx.current_function.as_ref() {
                        let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);

                        if !def_blocks.contains_key(&v) {
                            // Found undefined ValueId - log AST type and span
                            crate::runtime::get_global_ring0().log.error(&format!("[call/arg_build:undefined_value] fn={} bb={:?} arg_idx={} v=%{} ast={} span={:?} next={}",
                                func.signature.name,
                                self.current_block,
                                arg_idx,
                                v.0,
                                arg.node_type(),
                                arg.span(),
                                func.next_value_id
                            ));
                        }
                    }
                }

                values.push(v);
            }
        }

        // 式コンテキスト用の戻り値（呼び出し元では通常使われない）
        let void_value = crate::mir::builder::emission::constant::emit_void(self)?;

        // RDN-0: DebugLog retire。label/value は Debug 列へ正規化する。
        if method == "mark" || values.is_empty() {
            self.emit_instruction(MirInstruction::Debug {
                value: void_value,
                message: label,
            })?;
            return Ok(Some(void_value));
        }

        for (idx, value) in values.iter().copied().enumerate() {
            let message = if values.len() <= 1 {
                label.clone()
            } else {
                format!("{}[{}]", label, idx)
            };
            self.emit_instruction(MirInstruction::Debug { value, message })?;
        }

        Ok(Some(void_value))
    }

    /// Debug trace for receiver (if enabled)
    pub(super) fn trace_receiver_if_enabled(&self, object: &ASTNode, object_value: ValueId) {
        if crate::config::env::builder_debug_param_receiver() {
            if let ASTNode::Variable { name, .. } = object {
                let trace = crate::mir::builder::control_flow::joinir::trace::trace();
                trace.stderr_if(
                    &format!(
                        "[DEBUG/param-recv] build_method_call receiver '{}' → ValueId({})",
                        name, object_value.0
                    ),
                    true,
                );
                if let Some(origin) = self.type_ctx.value_origin_newbox.get(&object_value) {
                    trace.stderr_if(&format!("[DEBUG/param-recv]   origin: {}", origin), true);
                }
                if let Some(&mapped_id) = self.variable_ctx.variable_map.get(name) {
                    trace.stderr_if(
                        &format!(
                            "[DEBUG/param-recv]   variable_map['{}'] = ValueId({})",
                            name, mapped_id.0
                        ),
                        true,
                    );
                    if mapped_id != object_value {
                        trace.stderr_if(
                            "[DEBUG/param-recv]   ⚠️ MISMATCH! build_expression returned different ValueId!",
                            true,
                        );
                    }
                } else {
                    trace.stderr_if(
                        &format!(
                            "[DEBUG/param-recv]   ⚠️ '{}' NOT FOUND in variable_map!",
                            name
                        ),
                        true,
                    );
                }
                trace.stderr_if(
                    &format!("[DEBUG/param-recv]   current_block: {:?}", self.current_block),
                    true,
                );
            }
        }
    }
}
