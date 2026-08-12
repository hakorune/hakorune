//! 🎯 箱理論: Function/method skeleton creation
//!
//! 責務:
//! - Function/method skeleton creation with entry blocks
//! - Region observer setup for function scopes
//! - Parameter allocation and initialization
//!
//! このモジュールは関数の「骨格」を作成する責務のみを持つ。
//! 本体lowering や finalize は別モジュールで処理される。

use super::function_lowering;
use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::function::MirParamDecl;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::{FunctionSignature, MirType};

impl MirBuilder {
    /// 🎯 箱理論: Step 2 - 関数スケルトン作成
    pub(in crate::mir::builder) fn create_function_skeleton(
        &mut self,
        func_name: String,
        params: &[String],
        body: &[ASTNode],
    ) -> Result<(), String> {
        let signature =
            function_lowering::prepare_static_method_signature(func_name.clone(), params, body);
        self.install_function_skeleton(signature, Some("create_function_skeleton"))
    }

    /// Create a canonical skeleton from an already verified physical header.
    ///
    /// This entry deliberately does not inspect an AST body.  Return shape and
    /// parameter representation are supplied by the upstream contract/plan;
    /// the legacy body-aware entry remains available only to compatibility
    /// lowering.
    pub(in crate::mir::builder) fn create_resolved_function_skeleton(
        &mut self,
        func_name: String,
        param_decls: &[MirParamDecl],
        declared_return_type_name: Option<&str>,
        effects: crate::mir::EffectMask,
    ) -> Result<(), String> {
        let signature = FunctionSignature {
            name: func_name,
            params: param_decls
                .iter()
                .map(|decl| {
                    decl.declared_type_name
                        .as_deref()
                        .map(crate::mir::builder::builder_metadata::source_type_name_to_mir)
                        .unwrap_or(MirType::Unknown)
                })
                .collect(),
            return_type: declared_return_type_name
                .map(crate::mir::builder::builder_metadata::source_type_name_to_mir)
                .unwrap_or(MirType::Void),
            effects,
        };
        self.install_function_skeleton(signature, Some("create_resolved_function_skeleton"))
    }

    fn install_function_skeleton(
        &mut self,
        signature: FunctionSignature,
        trace_name: Option<&str>,
    ) -> Result<(), String> {
        let func_name = signature.name.clone();
        let entry = self.next_block_id();
        let function = self.new_function_with_metadata(signature, entry);

        if let Some(trace_name) = trace_name {
            let trace = crate::mir::builder::control_flow::joinir::trace::trace();
            trace.emit_if(
                "debug",
                trace_name,
                &format!("Creating function: {}", func_name),
                trace.is_enabled(),
            );
            trace.emit_if(
                "debug",
                trace_name,
                &format!("Entry block: {:?}", entry),
                trace.is_enabled(),
            );
        }

        // Phase 136 Step 3/7: Use scope_ctx as SSOT
        self.function_state.current_function = Some(function);
        self.function_state.current_block = Some(entry);
        // Phase 29bq+: reset sealing session for new function
        self.function_state.frag_emit_session.reset();
        // 新しい関数スコープ用の SlotRegistry を準備するよ（観測専用）
        self.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
        self.ensure_block_exists(entry)?;

        // Region 観測レイヤ: static 関数用の FunctionRegion を積むよ。
        crate::mir::region::observer::observe_function_region(self);

        Ok(())
    }

    /// 🎯 箱理論: Step 2b - 関数スケルトン作成（instance method版）
    pub(in crate::mir::builder) fn create_method_skeleton(
        &mut self,
        func_name: String,
        box_name: &str,
        params: &[String],
        body: &[ASTNode],
    ) -> Result<(), String> {
        let signature =
            function_lowering::prepare_method_signature(func_name, box_name, params, body);
        self.install_function_skeleton(signature, None)
    }
}
