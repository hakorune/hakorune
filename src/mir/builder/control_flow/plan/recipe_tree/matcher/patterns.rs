use super::super::{
    build_accum_const_loop_recipe, build_array_join_recipe, build_bool_predicate_scan_recipe,
    build_char_map_recipe, build_loop_break_recipe, build_loop_simple_while_recipe,
    build_loop_true_early_exit_recipe, build_scan_with_init_recipe, build_split_scan_recipe,
    AccumConstLoopRecipe, ArrayJoinRecipe, BoolPredicateScanRecipe, CharMapRecipe, LoopBreakRecipe,
    LoopSimpleWhileRecipe, LoopTrueEarlyExitRecipe, ScanWithInitRecipe, SplitScanRecipe,
};
use super::utils::*;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::if_phi_join_builder::{
    build_if_phi_join_recipe, IfPhiJoinRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::loop_continue_only_builder::{
    build_loop_continue_only_recipe, LoopContinueOnlyRecipe,
};

/// Recipe-first verification for loop-break.
pub fn verify_loop_break_recipe(
    loop_break_facts: &crate::mir::builder::control_flow::plan::facts::LoopBreakFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    // Create dummy loop_stmt (structure verification only)
    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(loop_break_facts.loop_condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    // Create CondBlockView from actual conditions
    let loop_cond_view = CondBlockView::from_expr(&loop_break_facts.loop_condition);
    let break_cond_view = CondBlockView::from_expr(&loop_break_facts.break_condition);

    // Build Recipe
    let recipe = build_loop_break_recipe(
        &loop_stmt,
        loop_cond_view,
        break_cond_view,
        loop_break_facts,
    );

    let Some(LoopBreakRecipe { arena, root }) = recipe else {
        // Recipe not buildable = contract violation in planner_required
        return Err(Freeze::contract(
            "LoopBreak recipe missing (planner_required)",
        ));
    };

    // Verify Recipe structure
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "loop_break_recipe",
    )
    .map_err(|e| Freeze::contract("LoopBreak recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_break status=ok");
    }
    Ok(())
}

pub fn verify_generic_loop_v1_recipe(
    generic_loop: &crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopV1Facts,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let Some(recipe) = generic_loop.body_exit_allowed.as_ref() else {
        return Err(Freeze::unsupported(
            "generic_loop_v1: cannot build recipe for body",
        ));
    };

    check_block_contract(
        &recipe.arena,
        &recipe.block,
        BlockContractKind::ExitAllowed,
        "generic_loop_v1",
    )
    .map(|_| ())
    .map_err(|e| Freeze::contract("[generic_loop_v1] recipe verification failed").with_hint(&e))
}

/// Recipe-first verification for if-phi-join.
pub fn verify_if_phi_join_recipe(
    if_phi_join_facts: &crate::mir::builder::control_flow::facts::IfPhiJoinFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(if_phi_join_facts.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&if_phi_join_facts.condition);
    let if_cond_view = CondBlockView::from_expr(&if_phi_join_facts.if_condition);

    let recipe =
        build_if_phi_join_recipe(&loop_stmt, loop_cond_view, if_cond_view, if_phi_join_facts);

    let Some(IfPhiJoinRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "IfPhiJoin recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "if_phi_join_recipe",
    )
    .map_err(|e| Freeze::contract("IfPhiJoin recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=if_phi_join status=ok");
    }
    Ok(())
}

/// Recipe-first verification for loop-continue-only.
pub fn verify_loop_continue_only_recipe(
    continue_only_facts: &crate::mir::builder::control_flow::plan::facts::LoopContinueOnlyFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(continue_only_facts.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&continue_only_facts.condition);
    let continue_cond_view = CondBlockView::from_expr(&continue_only_facts.continue_condition);

    let recipe = build_loop_continue_only_recipe(
        &loop_stmt,
        loop_cond_view,
        continue_cond_view,
        continue_only_facts,
    );

    let Some(LoopContinueOnlyRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "LoopContinueOnly recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "loop_continue_only_recipe",
    )
    .map_err(|e| Freeze::contract("LoopContinueOnly recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_continue_only status=ok");
    }
    Ok(())
}

/// Recipe-first verification for loop-true-early-exit.
pub fn verify_loop_true_early_exit_recipe(
    early_exit_facts: &crate::mir::builder::control_flow::plan::facts::LoopTrueEarlyExitFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    // LoopTrueEarlyExit is loop(true), so condition is always true
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span,
        }),
        body: vec![],
        span: dummy_span,
    };

    let exit_cond_view = CondBlockView::from_expr(&early_exit_facts.exit_condition);

    let recipe = build_loop_true_early_exit_recipe(&loop_stmt, exit_cond_view, early_exit_facts);

    let Some(LoopTrueEarlyExitRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "LoopTrueEarlyExit recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "loop_true_early_exit_recipe",
    )
    .map_err(|e| Freeze::contract("LoopTrueEarlyExit recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_true_early_exit status=ok");
    }
    Ok(())
}

/// Recipe-first verification for loop-simple-while.
pub fn verify_loop_simple_while_recipe(
    simple_while_facts: &crate::mir::builder::control_flow::plan::facts::LoopSimpleWhileFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(simple_while_facts.condition.clone()),
        body: vec![simple_while_facts.loop_increment.clone()],
        span: dummy_span,
    };

    let cond_view = CondBlockView::from_expr(&simple_while_facts.condition);

    // Get body from facts (loop body context)
    let body = &[simple_while_facts.loop_increment.clone()];

    let recipe = build_loop_simple_while_recipe(&loop_stmt, cond_view, body);

    let Some(LoopSimpleWhileRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "LoopSimpleWhile recipe missing (planner_required)",
        ));
    };

    // Note: Root block contains LoopV0, so NoExit is appropriate (not StmtOnly).
    // The nested body_block inside LoopV0 is already set to StmtOnly contract.
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "loop_simple_while_recipe",
    )
    .map_err(|e| Freeze::contract("LoopSimpleWhile recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_simple_while status=ok");
    }
    Ok(())
}

/// Recipe-first verification for loop-char-map.
pub fn verify_loop_char_map_recipe(
    char_map_facts: &crate::mir::builder::control_flow::plan::facts::LoopCharMapFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(char_map_facts.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let cond_view = CondBlockView::from_expr(&char_map_facts.condition);
    crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_value(
        &char_map_facts.cond_profile,
    );

    let recipe = build_char_map_recipe(&loop_stmt, cond_view, char_map_facts);

    let Some(CharMapRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "LoopCharMap recipe missing (planner_required)",
        ));
    };

    // Note: Root block contains LoopV0, so NoExit is appropriate (not StmtOnly).
    // The nested body_block inside LoopV0 is already set to StmtOnly contract.
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "loop_char_map_recipe",
    )
    .map_err(|e| Freeze::contract("LoopCharMap recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_char_map status=ok");
    }
    Ok(())
}

/// Recipe-first verification for loop-array-join.
pub fn verify_loop_array_join_recipe(
    array_join_facts: &crate::mir::builder::control_flow::plan::facts::LoopArrayJoinFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(array_join_facts.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&array_join_facts.condition);
    crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_value(
        &array_join_facts.cond_profile,
    );
    let if_cond_view = CondBlockView::from_expr(&array_join_facts.if_condition);
    let recipe =
        build_array_join_recipe(&loop_stmt, loop_cond_view, if_cond_view, array_join_facts);

    let Some(ArrayJoinRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "LoopArrayJoin recipe missing (planner_required)",
        ));
    };

    // Root contains LoopV0 with NoExit body (IfV2 + Stmt + Stmt)
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "loop_array_join_recipe",
    )
    .map_err(|e| Freeze::contract("LoopArrayJoin recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_array_join status=ok");
    }
    Ok(())
}

/// Recipe-first verification for scan-with-init.
pub fn verify_scan_with_init_recipe(
    scan_with_init_facts: &crate::mir::builder::control_flow::plan::facts::loop_types::ScanWithInitFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_condition = build_scan_with_init_loop_condition(scan_with_init_facts);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(loop_condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&loop_condition);
    let recipe = build_scan_with_init_recipe(&loop_stmt, loop_cond_view, scan_with_init_facts);

    let Some(ScanWithInitRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "ScanWithInit recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "scan_with_init_recipe",
    )
    .map_err(|e| Freeze::contract("ScanWithInit recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=scan_with_init status=ok");
    }
    Ok(())
}

/// Recipe-first verification for split-scan.
pub fn verify_split_scan_recipe(
    split_scan_facts: &crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_condition = build_split_scan_loop_condition(split_scan_facts);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(loop_condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&loop_condition);
    let recipe = build_split_scan_recipe(&loop_stmt, loop_cond_view, split_scan_facts);

    let Some(SplitScanRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "SplitScan recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "split_scan_recipe",
    )
    .map_err(|e| Freeze::contract("SplitScan recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=split_scan status=ok");
    }
    Ok(())
}

/// Recipe-first verification for bool-predicate-scan.
pub fn verify_bool_predicate_scan_recipe(
    bool_scan_facts: &crate::mir::builder::control_flow::plan::facts::BoolPredicateScanFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(bool_scan_facts.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&bool_scan_facts.condition);
    crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_value(
        &bool_scan_facts.cond_profile,
    );
    let recipe = build_bool_predicate_scan_recipe(&loop_stmt, loop_cond_view, bool_scan_facts);

    let Some(BoolPredicateScanRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "BoolPredicateScan recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "bool_predicate_scan_recipe",
    )
    .map_err(|e| Freeze::contract("BoolPredicateScan recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=bool_predicate_scan status=ok");
    }
    Ok(())
}

/// Recipe-first verification for accum-const-loop.
pub fn verify_accum_const_loop_recipe(
    accum_const_facts: &crate::mir::builder::control_flow::plan::facts::AccumConstLoopFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(accum_const_facts.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&accum_const_facts.condition);
    crate::mir::builder::control_flow::verify::verifier::debug_observe_cond_profile_value(
        &accum_const_facts.cond_profile,
    );
    let recipe = build_accum_const_loop_recipe(&loop_stmt, loop_cond_view, accum_const_facts);

    let Some(AccumConstLoopRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "AccumConstLoop recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "accum_const_loop_recipe",
    )
    .map_err(|e| Freeze::contract("AccumConstLoop recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=accum_const_loop status=ok");
    }
    Ok(())
}
