//! 🎯 箱理論: Call命令発行専用モジュール
//!
//! 責務: MIR Call命令の発行のみ
//! - emit_unified_call: 統一Call発行（Phase 3対応）
//! - emit_legacy_call: レガシーCall発行（既存互換）
//! - emit_global_call/emit_method_call/emit_constructor_call: 便利ラッパー

use super::super::{EffectMask, MirBuilder, MirInstruction, ValueId};
use super::CallTarget;
use crate::mir::definitions::call_unified::Callee;

impl MirBuilder {
    /// Unified call emission - delegates to UnifiedCallEmitterBox
    /// 箱理論: 統一Call発行ロジックを unified_emitter.rs に集約
    pub fn emit_unified_call(
        &mut self,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        super::unified_emitter::UnifiedCallEmitterBox::emit_unified_call(self, dst, target, args)
    }

    /// Legacy call fallback - preserves existing behavior
    pub fn emit_legacy_call(
        &mut self,
        dst: Option<ValueId>,
        target: CallTarget,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        match target {
            CallTarget::Method {
                receiver,
                method,
                box_type: _,
            } => {
                // LEGACY PATH (after unified migration):
                // Instance→Function rewrite is centralized in unified call path.
                // Legacy path no longer functionizes; always use Box/Plugin call here.
                // CRITICAL FIX: Prevent bouncing back to emit_unified_call
                // Set flag to prevent emit_box_or_plugin_call from calling emit_unified_call
                let prev_flag = self.in_unified_boxcall_fallback;
                self.in_unified_boxcall_fallback = true;
                let result =
                    self.emit_box_or_plugin_call(dst, receiver, method, None, args, EffectMask::IO);
                self.in_unified_boxcall_fallback = prev_flag;
                result
            }
            CallTarget::Constructor(box_type) => {
                // Use existing NewBox
                let dst = dst.ok_or("Constructor must have destination")?;
                self.emit_instruction(MirInstruction::NewBox {
                    dst,
                    box_type,
                    args,
                })
            }
            CallTarget::Extern(name) => {
                // Use existing ExternCall
                let mut args = args;
                crate::mir::builder::ssa::local::finalize_args(self, &mut args)?;
                let parts: Vec<&str> = name.splitn(2, '.').collect();
                let (iface, method) = if parts.len() == 2 {
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    ("nyash".to_string(), name)
                };

                self.emit_extern_call_with_effects(&iface, &method, args, dst, EffectMask::IO)
            }
            CallTarget::Global(name) => {
                super::unified_emitter::UnifiedCallEmitterBox::emit_global_unified(
                    self, dst, name, args,
                )
            }
            CallTarget::Value(func_val) => {
                super::unified_emitter::UnifiedCallEmitterBox::emit_value_unified(
                    self, dst, func_val, args,
                )
            }
            CallTarget::Closure {
                params,
                captures,
                me_capture,
            } => {
                let dst = dst.ok_or("Closure creation must have destination")?;
                self.emit_instruction(MirInstruction::NewClosure {
                    dst,
                    params,
                    body_id: None,
                    body: vec![], // Empty body for now
                    captures,
                    me: me_capture,
                })
            }
        }
    }

    // Phase 2 Migration: Convenience methods that use emit_unified_call

    /// Emit a global function call (print, panic, etc.)
    pub fn emit_global_call(
        &mut self,
        dst: Option<ValueId>,
        name: String,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        self.emit_unified_call(dst, CallTarget::Global(name), args)
    }

    /// Emit a method call (box.method)
    pub fn emit_method_call(
        &mut self,
        dst: Option<ValueId>,
        receiver: ValueId,
        method: String,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        self.emit_unified_call(
            dst,
            CallTarget::Method {
                box_type: None, // Auto-infer
                method,
                receiver,
            },
            args,
        )
    }

    /// Emit a constructor call (new BoxType)
    pub fn emit_constructor_call(
        &mut self,
        dst: ValueId,
        box_type: String,
        args: Vec<ValueId>,
    ) -> Result<(), String> {
        self.emit_unified_call(Some(dst), CallTarget::Constructor(box_type), args)
    }

    // ========================================
    // Private helper methods (small functions)
    // ========================================

    /// Try fallback handlers for global functions (delegates to CallMaterializerBox)
    #[allow(dead_code)]
    pub(super) fn try_global_fallback_handlers(
        &mut self,
        dst: Option<ValueId>,
        name: &str,
        args: &[ValueId],
    ) -> Result<Option<()>, String> {
        super::materializer::CallMaterializerBox::try_global_fallback_handlers(
            self, dst, name, args,
        )
    }

    /// Ensure receiver is materialized in Callee::Method (delegates to CallMaterializerBox)
    #[allow(dead_code)]
    pub(super) fn materialize_receiver_in_callee(
        &mut self,
        callee: Callee,
    ) -> Result<Callee, String> {
        super::materializer::CallMaterializerBox::materialize_receiver_in_callee(self, callee)
    }

    // ✅ 箱化完了:
    // - emit_unified_call_impl → UnifiedCallEmitterBox::emit_unified_call_impl (unified_emitter.rs)
    // - emit_global_unified → UnifiedCallEmitterBox::emit_global_unified (unified_emitter.rs)
    // - emit_value_unified → UnifiedCallEmitterBox::emit_value_unified (unified_emitter.rs)
    // - apply_static_runtime_guard → CalleeGuardBox::apply_static_runtime_guard (guard.rs)
}
