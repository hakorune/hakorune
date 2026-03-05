use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use super::super::{
    build_loop_break_recipe, LoopBreakRecipe,
    build_if_phi_join_recipe, IfPhiJoinRecipe,
    build_loop_continue_only_recipe, LoopContinueOnlyRecipe,
    build_loop_true_early_exit_recipe, LoopTrueEarlyExitRecipe,
    build_loop_simple_while_recipe, LoopSimpleWhileRecipe,
    build_char_map_recipe, CharMapRecipe,
    build_array_join_recipe, ArrayJoinRecipe,
    build_scan_with_init_recipe, ScanWithInitRecipe,
    build_split_scan_recipe, SplitScanRecipe,
    build_bool_predicate_scan_recipe, BoolPredicateScanRecipe,
    build_accum_const_loop_recipe, AccumConstLoopRecipe,
};
use super::utils::*;

/// Recipe-first verification for Pattern2Break.
pub fn verify_pattern2_break_recipe(
    pattern2: &crate::mir::builder::control_flow::plan::facts::pattern2_break_types::Pattern2BreakFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    // Create dummy loop_stmt (structure verification only)
    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern2.loop_condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    // Create CondBlockView from actual conditions
    let loop_cond_view = CondBlockView::from_expr(&pattern2.loop_condition);
    let break_cond_view = CondBlockView::from_expr(&pattern2.break_condition);

    // Build Recipe
    let recipe = build_loop_break_recipe(
        &loop_stmt,
        loop_cond_view,
        break_cond_view,
        pattern2,
    );

    let Some(LoopBreakRecipe { arena, root }) = recipe else {
        // Recipe not buildable = contract violation in planner_required
        return Err(Freeze::contract(
            "Pattern2Break recipe missing (planner_required)",
        ));
    };

    // Verify Recipe structure
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "pattern2_break_recipe",
    ).map_err(|e| {
        Freeze::contract("Pattern2Break recipe verification failed")
            .with_hint(&e)
    })?;

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
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;

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
    .map_err(|e| {
        Freeze::contract("[generic_loop_v1] recipe verification failed").with_hint(&e)
    })
}

/// Recipe-first verification for Pattern3IfPhi.
pub fn verify_pattern3_ifphi_recipe(
    pattern3: &crate::mir::builder::control_flow::plan::facts::pattern3_ifphi_facts::Pattern3IfPhiFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern3.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&pattern3.condition);
    let if_cond_view = CondBlockView::from_expr(&pattern3.if_condition);

    let recipe = build_if_phi_join_recipe(
        &loop_stmt,
        loop_cond_view,
        if_cond_view,
        pattern3,
    );

    let Some(IfPhiJoinRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern3IfPhi recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "pattern3_ifphi_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern3IfPhi recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=if_phi_join status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern4Continue.
pub fn verify_pattern4_continue_recipe(
    pattern4: &crate::mir::builder::control_flow::plan::facts::pattern4_continue_facts::Pattern4ContinueFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern4.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&pattern4.condition);
    let continue_cond_view = CondBlockView::from_expr(&pattern4.continue_condition);

    let recipe = build_loop_continue_only_recipe(
        &loop_stmt,
        loop_cond_view,
        continue_cond_view,
        pattern4,
    );

    let Some(LoopContinueOnlyRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern4Continue recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "pattern4_continue_recipe",
    )
    .map_err(|e| Freeze::contract("Pattern4Continue recipe verification failed").with_hint(&e))?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_continue_only status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern5InfiniteEarlyExit.
pub fn verify_pattern5_infinite_early_exit_recipe(
    pattern5: &crate::mir::builder::control_flow::plan::facts::pattern5_infinite_early_exit_facts::Pattern5InfiniteEarlyExitFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    // Pattern5 is loop(true), so condition is always true
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: dummy_span,
        }),
        body: vec![],
        span: dummy_span,
    };

    let exit_cond_view = CondBlockView::from_expr(&pattern5.exit_condition);

    let recipe = build_loop_true_early_exit_recipe(&loop_stmt, exit_cond_view, pattern5);

    let Some(LoopTrueEarlyExitRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern5InfiniteEarlyExit recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "pattern5_infinite_early_exit_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern5InfiniteEarlyExit recipe verification failed").with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_true_early_exit status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern1SimpleWhile.
pub fn verify_pattern1_simple_while_recipe(
    pattern1: &crate::mir::builder::control_flow::plan::facts::pattern1_simplewhile_facts::Pattern1SimpleWhileFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern1.condition.clone()),
        body: vec![pattern1.loop_increment.clone()],
        span: dummy_span,
    };

    let cond_view = CondBlockView::from_expr(&pattern1.condition);

    // Get body from facts (loop body context)
    let body = &[pattern1.loop_increment.clone()];

    let recipe = build_loop_simple_while_recipe(&loop_stmt, cond_view, body);

    let Some(LoopSimpleWhileRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern1SimpleWhile recipe missing (planner_required)",
        ));
    };

    // Note: Root block contains LoopV0, so NoExit is appropriate (not StmtOnly).
    // The nested body_block inside LoopV0 is already set to StmtOnly contract.
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "pattern1_simple_while_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern1SimpleWhile recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_simple_while status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern1CharMap.
pub fn verify_pattern1_char_map_recipe(
    pattern1_cm: &crate::mir::builder::control_flow::plan::facts::pattern1_char_map_facts::Pattern1CharMapFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern1_cm.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let cond_view = CondBlockView::from_expr(&pattern1_cm.condition);
    crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_value(
        &pattern1_cm.cond_profile,
    );

    let recipe = build_char_map_recipe(&loop_stmt, cond_view, pattern1_cm);

    let Some(CharMapRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern1CharMap recipe missing (planner_required)",
        ));
    };

    // Note: Root block contains LoopV0, so NoExit is appropriate (not StmtOnly).
    // The nested body_block inside LoopV0 is already set to StmtOnly contract.
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "pattern1_char_map_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern1CharMap recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_char_map status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern1ArrayJoin.
pub fn verify_pattern1_array_join_recipe(
    pattern1_aj: &crate::mir::builder::control_flow::plan::facts::pattern1_array_join_facts::Pattern1ArrayJoinFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern1_aj.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&pattern1_aj.condition);
    crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_value(
        &pattern1_aj.cond_profile,
    );
    let if_cond_view = CondBlockView::from_expr(&pattern1_aj.if_condition);
    let recipe = build_array_join_recipe(
        &loop_stmt,
        loop_cond_view,
        if_cond_view,
        pattern1_aj,
    );

    let Some(ArrayJoinRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern1ArrayJoin recipe missing (planner_required)",
        ));
    };

    // Root contains LoopV0 with NoExit body (IfV2 + Stmt + Stmt)
    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "pattern1_array_join_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern1ArrayJoin recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=loop_array_join status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern6 ScanWithInit.
pub fn verify_pattern6_scan_with_init_recipe(
    pattern6: &crate::mir::builder::control_flow::plan::facts::loop_types::ScanWithInitFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_condition = build_scan_with_init_loop_condition(pattern6);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(loop_condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&loop_condition);
    let recipe = build_scan_with_init_recipe(&loop_stmt, loop_cond_view, pattern6);

    let Some(ScanWithInitRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern6ScanWithInit recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "pattern6_scan_with_init_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern6ScanWithInit recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=scan_with_init status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern7 SplitScan.
pub fn verify_pattern7_split_scan_recipe(
    pattern7: &crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_condition = build_split_scan_loop_condition(pattern7);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(loop_condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&loop_condition);
    let recipe = build_split_scan_recipe(&loop_stmt, loop_cond_view, pattern7);

    let Some(SplitScanRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern7SplitScan recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "pattern7_split_scan_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern7SplitScan recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=split_scan status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern8 BoolPredicateScan.
pub fn verify_pattern8_bool_predicate_scan_recipe(
    pattern8: &crate::mir::builder::control_flow::plan::facts::pattern8_bool_predicate_scan_facts::Pattern8BoolPredicateScanFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern8.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&pattern8.condition);
    crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_value(
        &pattern8.cond_profile,
    );
    let recipe =
        build_bool_predicate_scan_recipe(&loop_stmt, loop_cond_view, pattern8);

    let Some(BoolPredicateScanRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern8BoolPredicateScan recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::ExitAllowed,
        "pattern8_bool_predicate_scan_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern8BoolPredicateScan recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=bool_predicate_scan status=ok");
    }
    Ok(())
}

/// Recipe-first verification for Pattern9 AccumConstLoop.
pub fn verify_pattern9_accum_const_loop_recipe(
    pattern9: &crate::mir::builder::control_flow::plan::facts::pattern9_accum_const_loop_facts::Pattern9AccumConstLoopFacts,
) -> Result<(), Freeze> {
    use crate::ast::{ASTNode, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;

    let dummy_span = Span::new(0, 0, 0, 0);
    let loop_stmt = ASTNode::Loop {
        condition: Box::new(pattern9.condition.clone()),
        body: vec![],
        span: dummy_span,
    };

    let loop_cond_view = CondBlockView::from_expr(&pattern9.condition);
    crate::mir::builder::control_flow::plan::verifier::debug_observe_cond_profile_value(
        &pattern9.cond_profile,
    );
    let recipe = build_accum_const_loop_recipe(&loop_stmt, loop_cond_view, pattern9);

    let Some(AccumConstLoopRecipe { arena, root }) = recipe else {
        return Err(Freeze::contract(
            "Pattern9AccumConstLoop recipe missing (planner_required)",
        ));
    };

    check_block_contract(
        &arena,
        &root,
        BlockContractKind::NoExit,
        "pattern9_accum_const_loop_recipe",
    )
    .map_err(|e| {
        Freeze::contract("Pattern9AccumConstLoop recipe verification failed")
            .with_hint(&e)
    })?;

    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:verify] route=accum_const_loop status=ok");
    }
    Ok(())
}
