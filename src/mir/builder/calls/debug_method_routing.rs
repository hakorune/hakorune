//! 🎯 箱理論: Debug/REPL/MIR method routing
//!
//! 責務: 開発専用・診断用のメソッド呼び出し処理
//! - trace_method_call_if_enabled: メソッド呼び出しトレース
//! - trace_receiver_if_enabled: レシーバートレース
//! Reserved-route admission is owned by `mir::policies`. This module only
//! emits already-selected MIR debug and REPL operations.

use super::super::{EffectMask, MirBuilder, MirInstruction, ValueId};
use crate::ast::ASTNode;
use crate::mir::policies::source_method_reserved_route::{MirDebugMethodV1, ReplIntrinsicMethodV1};

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
        crate::runtime::get_global_ring0().log.info(&format!(
            "[builder] method-call object kind={} method={}",
            kind, method
        ));
    }

    /// Emit one already-selected REPL intrinsic.
    pub(super) fn build_selected_repl_method_call(
        &mut self,
        method: ReplIntrinsicMethodV1,
        arg_values: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        // Emit ExternCall instruction
        let dst = self.next_value_id();
        self.emit_extern_call_with_effects(
            "__repl",
            method.spelling(),
            arg_values,
            Some(dst),
            EffectMask::PURE, // get/set are pure from MIR perspective
        )?;

        Ok(dst)
    }

    /// Dev-only: __mir__.log / __mir__.mark → MirInstruction::Debug 列への変換
    ///
    /// 構文:
    ///   __mir__.log("label", v1, v2, ...)
    ///   __mir__.mark("label")  // label-only marker
    ///
    /// - 第一引数は String リテラル想定（それ以外はこのハンドラをスキップして通常の解決に回す）。
    /// - 戻り値は Void 定数の ValueId（式コンテキストでも型破綻しないようにするため）。
    pub(super) fn build_selected_mir_debug_call(
        &mut self,
        method: MirDebugMethodV1,
        label: &str,
        values: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        // 式コンテキスト用の戻り値（呼び出し元では通常使われない）
        let void_value = crate::mir::builder::emission::constant::emit_void(self)?;

        // RDN-0: DebugLog retire。label/value は Debug 列へ正規化する。
        if method == MirDebugMethodV1::Mark || values.is_empty() {
            self.emit_instruction(MirInstruction::Debug {
                value: void_value,
                message: label.into(),
            })?;
            return Ok(void_value);
        }

        for (idx, value) in values.iter().copied().enumerate() {
            let message = if values.len() <= 1 {
                label.into()
            } else {
                format!("{}[{}]", label, idx)
            };
            self.emit_instruction(MirInstruction::Debug { value, message })?;
        }

        Ok(void_value)
    }

    pub(super) fn observe_selected_mir_debug_argument(
        &self,
        syntax: &ASTNode,
        argument_index: usize,
        value: ValueId,
    ) {
        if !crate::config::env::joinir_dev::debug_enabled() {
            return;
        }
        let Some(function) = self.scope_ctx.current_function.as_ref() else {
            return;
        };
        let def_blocks = crate::mir::verification::utils::compute_def_blocks(function);
        if def_blocks.contains_key(&value) {
            return;
        }
        crate::runtime::get_global_ring0().log.error(&format!(
            "[call/arg_build:undefined_value] fn={} bb={:?} arg_idx={} v=%{} ast={} span={:?} next={}",
            function.signature.name,
            self.current_block,
            argument_index,
            value.0,
            syntax.node_type(),
            syntax.span(),
            function.next_value_id
        ));
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
                    &format!(
                        "[DEBUG/param-recv]   current_block: {:?}",
                        self.current_block
                    ),
                    true,
                );
            }
        }
    }
}
