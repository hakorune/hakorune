use super::utils::*;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::planner::Freeze;

/// Recipe-first verification for loop_collect_using_entries_v0.
pub fn verify_loop_collect_using_entries_v0_recipe(
    collect_using: &crate::mir::builder::control_flow::facts::loop_collect_using_entries_v0::LoopCollectUsingEntriesV0Facts,
) -> Result<(), Freeze> {
    verify_no_exit_block_recipe(
        &collect_using.recipe.body_no_exit,
        "loop_collect_using_entries_v0_body_no_exit",
    )?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:collect_using_entries] verified OK"));
    }
    Ok(())
}

/// Recipe-first verification for loop_bundle_resolver_v0.
pub fn verify_loop_bundle_resolver_v0_recipe(
    bundle_resolver: &crate::mir::builder::control_flow::facts::loop_bundle_resolver_v0::LoopBundleResolverV0Facts,
) -> Result<(), Freeze> {
    verify_exit_allowed_block_recipe(
        &bundle_resolver.recipe.body_exit_allowed,
        "loop_bundle_resolver_v0_body_exit_allowed",
    )?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:bundle_resolver] verified OK"));
    }
    Ok(())
}

/// Recipe-first verification for loop_true_break_continue.
pub fn verify_loop_true_break_continue_recipe(
    loop_true: &crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::LoopTrueBreakContinueFacts,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::loop_cond::true_break_continue::LoopTrueBreakContinueLowering;
    use crate::mir::builder::control_flow::plan::loop_true_break_continue::recipe::{
        ElseItem, LoopTrueItem,
    };

    match &loop_true.lowering {
        LoopTrueBreakContinueLowering::ExitAllowed(body_exit_allowed) => {
            verify_exit_allowed_block_recipe(
                body_exit_allowed,
                "loop_true_break_continue_body_exit_allowed",
            )?;
        }
        LoopTrueBreakContinueLowering::RecipeOnly => {
            let body_len = loop_true.recipe.body.len();
            for item in &loop_true.recipe.items {
                match item {
                    LoopTrueItem::Stmt(r)
                    | LoopTrueItem::ProgramGeneralBlock(r)
                    | LoopTrueItem::ExitIf(r)
                    | LoopTrueItem::NestedLoopDepth1(r)
                    | LoopTrueItem::GeneralIf(r)
                    | LoopTrueItem::TailReturn(r) => {
                        verify_stmt_ref_in_bounds(*r, body_len, "loop_true_break_continue")?;
                    }
                    LoopTrueItem::IfTailExitPair(pair) => {
                        verify_stmt_pair_in_bounds(*pair, body_len, "loop_true_break_continue")?;
                    }
                    LoopTrueItem::GeneralIfElseExit {
                        if_ref,
                        else_recipe,
                    } => {
                        verify_stmt_ref_in_bounds(*if_ref, body_len, "loop_true_break_continue")?;
                        let else_len = else_recipe.else_body.len();
                        for else_item in &else_recipe.items {
                            match else_item {
                                ElseItem::ExitIf(r) | ElseItem::PreludeStmt(r) => {
                                    verify_stmt_ref_in_bounds(
                                        *r,
                                        else_len,
                                        "loop_true_break_continue_else",
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("[recipe:loop_true] verified OK"));
    }
    Ok(())
}

/// Recipe-first verification for loop_cond_break_continue.
pub fn verify_loop_cond_break_continue_recipe(
    facts: &crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakContinueFacts,
) -> Result<(), Freeze> {
    use crate::mir::policies::BodyLoweringPolicy;

    if matches!(
        facts.body_lowering_policy,
        BodyLoweringPolicy::ExitAllowed { .. }
    ) {
        let Some(body_exit_allowed) = facts.body_exit_allowed.as_ref() else {
            return Err(Freeze::contract(
                "loop_cond_break_continue: body_exit_allowed missing (planner_required)",
            ));
        };
        verify_exit_allowed_block_recipe(
            body_exit_allowed,
            "loop_cond_break_continue_body_exit_allowed",
        )?;
    }

    verify_loop_cond_break_continue_recipe_items(&facts.recipe, "loop_cond_break_continue_body")?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:loop_cond_break_continue] verified OK"));
    }
    Ok(())
}

/// Recipe-first verification for loop_cond_continue_only.
pub fn verify_loop_cond_continue_only_recipe(
    facts: &crate::mir::builder::control_flow::facts::loop_cond_continue_only::LoopCondContinueOnlyFacts,
) -> Result<(), Freeze> {
    verify_continue_only_recipe(&facts.recipe, "loop_cond_continue_only_body")?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:loop_cond_continue_only] verified OK"));
    }
    Ok(())
}

/// Recipe-first verification for loop_cond_continue_with_return.
pub fn verify_loop_cond_continue_with_return_recipe(
    facts: &crate::mir::builder::control_flow::facts::loop_cond_continue_with_return::LoopCondContinueWithReturnFacts,
) -> Result<(), Freeze> {
    verify_continue_with_return_recipe(&facts.recipe, "loop_cond_continue_with_return_body")?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[recipe:loop_cond_continue_with_return] verified OK"
        ));
    }
    Ok(())
}

/// Recipe-first verification for loop_cond_return_in_body.
pub fn verify_loop_cond_return_in_body_recipe(
    facts: &crate::mir::builder::control_flow::facts::loop_cond_return_in_body::LoopCondReturnInBodyFacts,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_recipe::LoopCondReturnInBodyItem;

    let body_len = facts.recipe.body.len();
    for item in &facts.recipe.items {
        match item {
            LoopCondReturnInBodyItem::Stmt(stmt) => {
                verify_stmt_ref_in_bounds(*stmt, body_len, "loop_cond_return_in_body")?;
            }
        }
    }

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug(&format!("[recipe:loop_cond_return_in_body] verified OK"));
    }
    Ok(())
}
