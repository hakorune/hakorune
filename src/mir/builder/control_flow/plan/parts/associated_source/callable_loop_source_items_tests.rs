//! Item-level tests for `lower_loop_cond_source_item`: the located LoopCond
//! item dispatcher seals each issued recipe item against the co-sealed body
//! carrier and drives it through the neutral Parts machinery. Shared fixtures
//! live in `callable_loop_source_testkit`; provider/co-seal negatives live in
//! `callable_loop_source_tests`.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::callable_loop_source_items::lower_loop_cond_source_item;
use super::callable_loop_source_testkit::{
    function_body_source, integer, real_ledger, test_builder, variable,
};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_only_block_recipe;
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceBodyInputV1, CallableLoopSourceExpressionPortV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::BodyChildRoleV1;
use crate::mir::ValueId;

/// Drive one issued item through the located dispatcher under the pinned
/// default JoinIR mode, sharing `builder`/`bindings` across calls exactly
/// like the future sibling's per-item loop. The pin acquires the
/// non-reentrant process state lock; callers must not nest another env scope.
fn drive_item(
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    body: &CallableLoopSourceBodyInputV1<'_>,
    builder: &mut MirBuilder,
    bindings: &mut BTreeMap<String, ValueId>,
    item_index: usize,
    item: &LoopCondBreakContinueItem,
) -> Result<Vec<LoweredRecipe>, String> {
    let port = CallableLoopSourceExpressionPortV1::new(ledger);
    let mut carrier_updates = BTreeMap::new();
    let empty = BTreeMap::new();
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_DEFAULT_MODE, || {
        lower_loop_cond_source_item(
            port,
            body,
            item_index,
            item,
            builder,
            bindings,
            &empty,
            &empty,
            &empty,
            &mut carrier_updates,
            "callable-loop-parts/test",
        )
    })
}

fn located_body<'a>(
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
    body: &'a [ASTNode],
) -> CallableLoopSourceBodyInputV1<'a> {
    let port = CallableLoopSourceExpressionPortV1::new(ledger);
    port.body(body, &function_body_source())
        .expect("located body")
}

fn return_stmt(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: crate::ast::Span::unknown(),
    }
}

#[test]
fn source_item_lowers_a_stmt_item_through_the_located_body() {
    let (ledger, body) = real_ledger("function t() { local i = 0 }");
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let plans = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::Stmt(StmtRef::new(0)),
    )
    .expect("direct stmt lowers");
    assert!(!plans.is_empty());
    assert!(bindings.contains_key("i"));
}

#[test]
fn source_item_lowers_a_return_exit_leaf() {
    let (ledger, body) = real_ledger("function t() { return 7 }");
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let plans = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::ExitLeaf {
            kind: ExitKind::Return,
            stmt: StmtRef::new(0),
        },
    )
    .expect("return exit leaf lowers");
    assert!(plans
        .iter()
        .any(|plan| matches!(plan, CorePlan::Exit(CoreExitPlan::Return(Some(_))))));
}

#[test]
fn source_item_lowers_program_block_if_through_the_exit_allowed_singleton() {
    // `if i == 0 { break } else { i = i + 1 }` derives a non-degenerate
    // `IfV2{ExitAllowed{ThenOnlyExit}}` singleton recipe; the else branch
    // falls through with the carrier update. `break` only resolves inside a
    // loop body, so the fixture hosts the if inside `loop(true)` and the
    // LoopCond body carrier is the located `LoopBody` child, exactly like
    // production.
    let (ledger, body) = real_ledger(
        "function t() { loop(true) { local i = 0; if i == 0 { break } else { i = i + 1 } } }",
    );
    let port = CallableLoopSourceExpressionPortV1::new(&ledger);
    let function_carrier = port
        .body(&body, &function_body_source())
        .expect("located function body");
    let loop_stmt = port
        .body_stmt(&function_carrier, 0)
        .expect("located loop stmt");
    let carrier = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::Stmt(StmtRef::new(0)),
    )
    .expect("local seed lowers");
    let plans = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        1,
        &LoopCondBreakContinueItem::ProgramBlock {
            stmt: StmtRef::new(1),
            stmt_only: None,
        },
    )
    .expect("program block lowers");
    assert!(
        plans.iter().any(|plan| matches!(plan, CorePlan::If(_))),
        "expected an if plan, got {plans:?}"
    );
    assert!(bindings.contains_key("i"));
}

#[test]
fn source_item_lowers_general_if_through_the_issued_no_exit_recipe() {
    // `GeneralIf` carries no `StmtRef`; the item ordinal locates the issued
    // `from_ref(if_stmt)` no-exit recipe's single statement.
    let (ledger, body) = real_ledger(
        "function t() { local i = 0; local j = 0; if i == 0 { j = 1 } else { j = 2 } }",
    );
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    for index in 0..2 {
        drive_item(
            &ledger,
            &carrier,
            &mut builder,
            &mut bindings,
            index,
            &LoopCondBreakContinueItem::Stmt(StmtRef::new(index)),
        )
        .expect("local seed lowers");
    }
    let recipe = try_build_no_exit_block_recipe(std::slice::from_ref(&body[2]), true)
        .expect("no-exit recipe");
    let plans = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        2,
        &LoopCondBreakContinueItem::GeneralIf(recipe),
    )
    .expect("general if lowers");
    assert!(
        plans.iter().any(|plan| matches!(plan, CorePlan::If(_))),
        "expected an if plan, got {plans:?}"
    );
    assert!(bindings.contains_key("j"));
}

#[test]
fn source_item_lowers_exit_if_tree_through_issued_branch_recipes() {
    // The issued ExitOnly branch recipes stay the packaging authority; the
    // dispatcher only seals them against the located branch carriers and
    // reuses the shared exit-if state core.
    let (ledger, body) =
        real_ledger("function t() { local i = 0; if i == 0 { return 0 } else { return 1 } }");
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::Stmt(StmtRef::new(0)),
    )
    .expect("local seed lowers");
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[1]
    else {
        panic!("fixture if")
    };
    let item = LoopCondBreakContinueItem::ExitIfTree {
        if_stmt: StmtRef::new(1),
        cond_view: CondBlockView::from_expr(condition),
        mode: IfMode::ExitAll,
        then_body: try_build_exit_only_block_recipe(then_body, true)
            .expect("then exit-only recipe"),
        else_body: Some(
            try_build_exit_only_block_recipe(else_body.as_ref().expect("fixture else"), true)
                .expect("else exit-only recipe"),
        ),
    };
    let plans = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 1, &item)
        .expect("exit-if tree lowers");
    assert!(
        plans.iter().any(|plan| matches!(plan, CorePlan::If(_))),
        "expected an if plan, got {plans:?}"
    );
}

#[test]
fn source_item_lowers_exit_if_tree_exit_if_mode_without_else() {
    // The acceptance tuple's `ExitIfTree` is `mode: ExitIf` (:83-86): an if
    // with no else whose then branch exits on all paths. The then-only
    // ExitOnly recipe seals against the located `IfThen` child.
    let (ledger, body) = real_ledger("function t() { local i = 0; if i == 0 { return 0 } }");
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::Stmt(StmtRef::new(0)),
    )
    .expect("local seed lowers");
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[1]
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let item = LoopCondBreakContinueItem::ExitIfTree {
        if_stmt: StmtRef::new(1),
        cond_view: CondBlockView::from_expr(condition),
        mode: IfMode::ExitIf,
        then_body: try_build_exit_only_block_recipe(then_body, true)
            .expect("then exit-only recipe"),
        else_body: None,
    };
    let plans = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 1, &item)
        .expect("exit-if tree lowers");
    assert!(
        plans.iter().any(|plan| matches!(plan, CorePlan::If(_))),
        "expected an if plan, got {plans:?}"
    );
}

#[test]
fn source_item_rejects_exit_if_tree_else_parity_drift() {
    // The issued `else_body` recipe claims an else branch that the located if
    // statement does not carry; the parity check rejects before any Builder
    // effect.
    let (ledger, body) = real_ledger("function t() { local i = 0; if i == 0 { return 0 } }");
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[1]
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let forged_else = try_build_exit_only_block_recipe(&[return_stmt(integer(1))], true)
        .expect("forged else recipe");
    let item = LoopCondBreakContinueItem::ExitIfTree {
        if_stmt: StmtRef::new(1),
        cond_view: CondBlockView::from_expr(condition),
        mode: IfMode::ExitAll,
        then_body: try_build_exit_only_block_recipe(then_body, true)
            .expect("then exit-only recipe"),
        else_body: Some(forged_else),
    };
    let error = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 1, &item)
        .expect_err("else parity drift is a named reject");
    assert!(error.contains("RecipeBodyMismatch"), "{error}");
}

#[test]
fn source_item_rejects_general_if_index_drift() {
    // The issued no-exit recipe body is the verbatim `from_ref(if_stmt)`
    // clone; pointing the item ordinal at a different located statement must
    // fail the `singleton` seal.
    let (ledger, body) = real_ledger(
        "function t() { local i = 0; local j = 0; if i == 0 { j = 1 } else { j = 2 } }",
    );
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let recipe = try_build_no_exit_block_recipe(std::slice::from_ref(&body[2]), true)
        .expect("no-exit recipe");
    let error = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::GeneralIf(recipe),
    )
    .expect_err("index drift is a named reject");
    assert!(error.contains("RecipeBodyMismatch"), "{error}");
}

#[test]
fn source_item_rejects_program_block_stmt_only_as_unlocated() {
    // A StmtOnly recipe body is the flattened container list; it cannot
    // align 1:1 with a located carrier, so the arm stays a named reject.
    let (ledger, body) =
        real_ledger("function t() { local i = 0; local j = 0; if i == 0 { j = 1 } }");
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let stmt_only =
        try_build_stmt_only_block_recipe(std::slice::from_ref(&body[2])).expect("stmt-only recipe");
    let error = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        2,
        &LoopCondBreakContinueItem::ProgramBlock {
            stmt: StmtRef::new(2),
            stmt_only: Some(stmt_only),
        },
    )
    .expect_err("stmt_only program block is a named reject");
    assert!(
        error.contains("program-block-stmt-only-unlocated"),
        "{error}"
    );
}

#[test]
fn source_item_rejects_unsupported_variants_as_named_terminals() {
    let (ledger, body) = real_ledger("function t() { local i = 0 }");
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let error = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::TailBreak { block: None },
    )
    .expect_err("unsupported variant is a named reject");
    assert!(error.contains("loop-cond-item-unsupported"), "{error}");
    let error = drive_item(
        &ledger,
        &carrier,
        &mut builder,
        &mut bindings,
        0,
        &LoopCondBreakContinueItem::NestedLoopDepth1 {
            loop_stmt: StmtRef::new(0),
            nested: crate::mir::builder::control_flow::recipes::loop_cond_break_continue::NestedLoopDepth1Recipe {
                cond_view: CondBlockView::from_expr(&variable("i")),
                body: None,
            },
        },
    )
    .expect_err("nested loop variant is a named reject");
    assert!(error.contains("loop-cond-item-unsupported"), "{error}");
}
