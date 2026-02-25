use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::common::balanced_depth_scan_emitter::{
    BalancedDepthScanEmitter, BalancedDepthScanRecipe,
};
use crate::mir::join_ir::lowering::common::body_local_derived_slot_emitter::{
    BodyLocalDerivedSlotEmitter, BodyLocalDerivedSlotRecipe,
};
use crate::mir::join_ir::lowering::common::condition_only_emitter::ConditionOnlyRecipe;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::lowering::debug_output_box::DebugOutputBox;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::JoinInst;
use crate::mir::ValueId;

pub(crate) fn emit_body_local_inits<F>(
    env: &ConditionEnv,
    body_ast: &[ASTNode],
    body_local_env: Option<&mut LoopBodyLocalEnv>,
    condition_only_recipe: Option<&ConditionOnlyRecipe>,
    body_local_derived_slot_recipe: Option<&BodyLocalDerivedSlotRecipe>,
    balanced_depth_scan_recipe: Option<&BalancedDepthScanRecipe>,
    current_static_box_name: Option<String>,
    alloc_local_fn: &mut F,
    body_init_block: &mut Vec<JoinInst>,
    dev_log: &DebugOutputBox,
) -> Result<(), String>
where
    F: FnMut() -> ValueId,
{
    let Some(body_env) = body_local_env else {
        return Ok(());
    };

    use crate::mir::join_ir::lowering::loop_body_local_init::LoopBodyLocalInitLowerer;

    let mut init_lowerer = LoopBodyLocalInitLowerer::new(
        env,
        body_init_block,
        Box::new(&mut *alloc_local_fn),
        current_static_box_name.clone(),
    );

    init_lowerer.lower_inits_for_loop(body_ast, body_env)?;

    dev_log.log_if_enabled(|| {
        format!(
            "Phase 191/246-EX: Lowered {} body-local init expressions (scheduled block before break)",
            body_env.len()
        )
    });

    drop(init_lowerer);

    if let Some(recipe) = condition_only_recipe {
        use crate::mir::join_ir::lowering::common::condition_only_emitter::ConditionOnlyEmitter;
        let mut temp_env = env.clone();

        let condition_value = ConditionOnlyEmitter::emit_condition_only_recalc(
            recipe,
            body_env,
            &mut temp_env,
            alloc_local_fn,
            body_init_block,
        )?;

        body_env.insert(recipe.name.clone(), condition_value);

        dev_log.log_if_enabled(|| {
            format!(
                "Phase 93 P0: Recalculated ConditionOnly variable '{}' → {:?} (registered in body_local_env)",
                recipe.name, condition_value
            )
        });
    }

    if let Some(recipe) = body_local_derived_slot_recipe {
        BodyLocalDerivedSlotEmitter::emit(
            recipe,
            alloc_local_fn,
            env,
            body_env,
            body_init_block,
            current_static_box_name.as_deref(),
        )?;
    }

    if let Some(recipe) = balanced_depth_scan_recipe {
        BalancedDepthScanEmitter::emit_derived(
            recipe,
            body_env,
            env,
            alloc_local_fn,
            body_init_block,
        )?;
    }

    Ok(())
}
