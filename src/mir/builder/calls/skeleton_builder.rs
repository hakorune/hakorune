//! 🎯 箱理論: Function/method skeleton creation
//!
//! 責務:
//! - Function/method skeleton creation with entry blocks
//! - Region observer setup for function scopes
//! - Parameter allocation and initialization
//!
//! このモジュールは関数の「骨格」を作成する責務のみを持つ。
//! 本体lowering や finalize は別モジュールで処理される。

use super::context_lifecycle::LoweringContext;
use super::function_lowering;
use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;

impl MirBuilder {
    /// 🎯 箱理論: Step 2 - 関数スケルトン作成
    pub(super) fn create_function_skeleton(
        &mut self,
        func_name: String,
        params: &[String],
        body: &[ASTNode],
        ctx: &mut LoweringContext,
    ) -> Result<(), String> {
        let signature =
            function_lowering::prepare_static_method_signature(func_name.clone(), params, body);
        let entry = self.next_block_id();
        let function = self.new_function_with_metadata(signature, entry);

        // Phase 136 Step 3/7: Save from scope_ctx (SSOT)
        ctx.saved_function = self.scope_ctx.current_function.take();
        ctx.saved_block = self.current_block.take();

        let trace = crate::mir::builder::control_flow::joinir::trace::trace();
        trace.emit_if(
            "debug",
            "create_function_skeleton",
            &format!("Creating function: {}", func_name),
            trace.is_enabled(),
        );
        trace.emit_if(
            "debug",
            "create_function_skeleton",
            &format!("Entry block: {:?}", entry),
            trace.is_enabled(),
        );

        // Phase 136 Step 3/7: Use scope_ctx as SSOT
        self.scope_ctx.current_function = Some(function);
        self.current_block = Some(entry);
        // Phase 29bq+: reset sealing session for new function
        self.frag_emit_session.reset();
        // 新しい関数スコープ用の SlotRegistry を準備するよ（観測専用）
        self.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
        self.ensure_block_exists(entry)?;

        // Region 観測レイヤ: static 関数用の FunctionRegion を積むよ。
        crate::mir::region::observer::observe_function_region(self);

        Ok(())
    }

    /// 🎯 箱理論: Step 2b - 関数スケルトン作成（instance method版）
    pub(super) fn create_method_skeleton(
        &mut self,
        func_name: String,
        box_name: &str,
        params: &[String],
        body: &[ASTNode],
        ctx: &mut LoweringContext,
    ) -> Result<(), String> {
        let signature =
            function_lowering::prepare_method_signature(func_name, box_name, params, body);
        let entry = self.next_block_id();
        let function = self.new_function_with_metadata(signature, entry);

        // Phase 136 Step 3/7: Save from scope_ctx (SSOT)
        ctx.saved_function = self.scope_ctx.current_function.take();
        ctx.saved_block = self.current_block.take();

        // Phase 136 Step 3/7: Use scope_ctx as SSOT
        self.scope_ctx.current_function = Some(function);
        self.current_block = Some(entry);
        // instance method 用の関数スコープ SlotRegistry もここで用意するよ。
        self.comp_ctx.current_slot_registry = Some(FunctionSlotRegistry::new());
        self.ensure_block_exists(entry)?;

        // Region 観測レイヤ: instance method 用の FunctionRegion も積んでおくよ。
        crate::mir::region::observer::observe_function_region(self);

        Ok(())
    }
}
