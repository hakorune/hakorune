//! Driver-level tests for the located callable-loop lowering hooks: recipe
//! items execute through the neutral block driver against a real
//! parse+resolve ledger. Provider/co-seal projection coverage lives in
//! `callable_loop_source_tests`; shared fixtures live in
//! `callable_loop_source_testkit`.

use std::collections::BTreeMap;

use super::callable_loop_source::CallableLoopSourcePartsBlockV1;
use super::callable_loop_source_lowering::CallableLoopSourcePartsLoweringHooksV1;
use super::callable_loop_source_testkit::{
    assign, drive_block, drive_recipe, function_body_source, integer, less_than, project,
    real_ledger, span, test_builder, variable,
};
use super::dispatch::{PartsAssociatedBlockModeV1, PartsAssociatedLoweringHooksV1};
use super::PartsAssociatedRecipeItemV1;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_allowed_block_recipe;
use crate::mir::builder::control_flow::plan::recipe_tree::{IfContractKind, IfMode, RecipeItem};
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceExprInputV1, CallableLoopSourceExpressionPortV1,
};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::resolved_semantics::BodyChildRoleV1;

#[test]
fn driver_lowers_no_exit_stmt_block_through_the_source_port() {
    let (ledger, body) = real_ledger("function t() { local tmp = 0; tmp = tmp + 1 }");
    let recipe = try_build_no_exit_block_recipe(&body, true).expect("no-exit recipe");
    let (plans, bindings) = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::NoExit,
    )
    .expect("no-exit block lowers");
    assert!(!plans.is_empty());
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_lowers_join_if_through_the_source_port() {
    let (ledger, body) =
        real_ledger("function t() { local tmp = 0; if tmp == 0 { tmp = 1 } else { tmp = 2 } }");
    let recipe = try_build_no_exit_block_recipe(&body, true).expect("no-exit recipe");
    assert!(matches!(
        recipe.block.items[1],
        RecipeItem::IfV2 {
            contract: IfContractKind::Join,
            ..
        }
    ));
    let (plans, bindings) = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::NoExit,
    )
    .expect("join if lowers");
    assert!(!plans.is_empty());
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_lowers_exit_if_under_exit_allowed_mode() {
    // `break` only resolves inside a loop body, so the fixture hosts the
    // exit-if inside `loop(true)` and the carrier is projected through the
    // located `LoopBody` child exactly like production does.
    let (ledger, body) =
        real_ledger("function t() { loop(true) { local tmp = 0; if tmp == 0 { break } } }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let function_carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let loop_stmt = port
        .body_stmt(&function_carrier, 0)
        .expect("located loop stmt");
    let carrier = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
    let ASTNode::Loop {
        body: loop_body, ..
    } = &body[0]
    else {
        panic!("fixture loop")
    };
    let recipe = try_build_exit_allowed_block_recipe(loop_body, true).expect("exit-allowed recipe");
    assert!(matches!(
        recipe.block.items[1],
        RecipeItem::IfV2 {
            contract: IfContractKind::ExitOnly {
                mode: IfMode::ExitIf
            },
            ..
        }
    ));
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("co-sealed block");
    let (plans, bindings) = drive_block(&ledger, &block, PartsAssociatedBlockModeV1::ExitAllowed)
        .expect("exit-if lowers");
    assert!(!plans.is_empty());
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_lowers_an_opaque_if_container_via_the_singleton_recipe() {
    let (ledger, body) = real_ledger("function t() { local tmp = 0; if tmp == 0 { tmp = 1 } }");
    // A join-bearing if with no else degrades to an opaque `Stmt` item under
    // the exit-allowed builder; the hooks re-derive the singleton container
    // recipe exactly like the raw return-prelude arm.
    let recipe = try_build_exit_allowed_block_recipe(&body, true).expect("exit-allowed recipe");
    assert!(matches!(recipe.block.items[1], RecipeItem::Stmt(_)));
    let (_, bindings) = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::ExitAllowed,
    )
    .expect("opaque if container lowers through the singleton recipe");
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_lowers_loop_v0_through_the_shared_core() {
    // The nested `loop` is a `LoopV0` recipe item; the source hook feeds the
    // shared `loop_v0` core with a ported header condition and a located body
    // block instead of the raw `CondBlockView`/verified pair.
    let (ledger, body) =
        real_ledger("function t() { local tmp = 0; loop(tmp < 3) { tmp = tmp + 1 } }");
    let recipe = try_build_no_exit_block_recipe(&body, true).expect("no-exit recipe");
    assert!(matches!(recipe.block.items[1], RecipeItem::LoopV0 { .. }));
    let (plans, bindings) = drive_recipe(
        &ledger,
        &body,
        &recipe.arena,
        &recipe.block,
        PartsAssociatedBlockModeV1::NoExit,
    )
    .expect("LoopV0 lowers through the shared core");
    assert_eq!(plans.len(), 2);
    assert!(
        matches!(plans[1], CorePlan::Loop(_)),
        "expected a CorePlan::Loop, got {:?}",
        plans[1]
    );
    assert!(bindings.contains_key("tmp"));
}

#[test]
fn driver_rejects_a_loop_v0_block_expr_condition_before_effects() {
    // A `CondBlockView` prelude has no ported lowering yet, so a BlockExpr
    // loop condition is a named reject before any Builder effect.
    let (ledger, body) =
        real_ledger("function t() { local tmp = 0; loop(tmp < 3) { tmp = tmp + 1 } }");
    let recipe = try_build_no_exit_block_recipe(&body, true).expect("no-exit recipe");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let carrier = port
        .body(&body, &function_body_source())
        .expect("located body");
    let block =
        CallableLoopSourcePartsBlockV1::located_body(&recipe.arena, &recipe.block, carrier, &port)
            .expect("co-sealed block");
    let source = super::callable_loop_source::CallableLoopSourcePartsAssociatedSourceV1::for_block(
        &block, port,
    );
    let item = project(&source, &block, 1).expect("loop item");
    let PartsAssociatedRecipeItemV1::RawLoopV0 { mut loop_input } = item else {
        panic!("expected RawLoopV0")
    };
    let prelude_cond = ASTNode::BlockExpr {
        prelude_stmts: vec![assign("tmp", integer(1))],
        tail_expr: Box::new(less_than(variable("tmp"), integer(3))),
        span: span(),
    };
    loop_input.condition = CallableLoopSourceExprInputV1::Synthetic(&prelude_cond);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let mut carrier_updates = BTreeMap::new();
    let empty = BTreeMap::new();
    let mut hooks = CallableLoopSourcePartsLoweringHooksV1 {
        builder: &mut builder,
        current_bindings: &mut bindings,
        carrier_phis: &empty,
        carrier_step_phis: &empty,
        break_phi_dsts: &empty,
        carrier_updates: &mut carrier_updates,
        error_prefix: "callable-loop-parts/test",
    };
    let error = hooks
        .lower_raw_loop_v0(port, loop_input)
        .expect_err("BlockExpr condition is a named reject");
    assert!(error.contains("loop-v0-cond-prelude-unlocated"), "{error}");
}
