//! Conditional-update item tests for `lower_loop_cond_source_item`: the
//! located LoopCond dispatcher seals `ConditionalUpdateIf` recipes against
//! the co-sealed carriers (else parity keyed on body-or-exit presence,
//! `CondBlockView` match, branch body/exit shape) and reuses the existing
//! port-aware conditional-update/Select owner. Shared fixtures live in
//! `callable_loop_source_testkit`.

use std::collections::BTreeMap;

use super::callable_loop_source_testkit::{
    drive_item, function_body_source, integer, located_body, real_ledger, test_builder, variable,
};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::recipe_tree::ExitKind;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan};
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::resolved_semantics::BodyChildRoleV1;

#[test]
fn source_item_lowers_conditional_update_if_through_located_branches() {
    // `if i == 0 { j = 1 }` (no else, no exit) is the json
    // `if end < 0 { end = n }` shape: the located port drives the
    // conditional-update/Select owner, not the wildcard reject.
    let (ledger, body) = real_ledger(
        "function t() { local i = 0; local j = 0; if i == 0 { j = 1 } }",
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
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[2]
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let item = LoopCondBreakContinueItem::ConditionalUpdateIf {
        if_stmt: StmtRef::new(2),
        cond_view: CondBlockView::from_expr(condition),
        then_body: Some(
            try_build_no_exit_block_recipe(then_body, true).expect("then no-exit recipe"),
        ),
        then_exit: None,
        else_body: None,
        else_exit: None,
    };
    let plans = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 2, &item)
        .expect("conditional update lowers");
    assert!(
        plans
            .iter()
            .any(|plan| matches!(plan, CorePlan::Effect(CoreEffectPlan::Select { .. }))),
        "expected a select plan, got {plans:?}"
    );
    assert!(bindings.contains_key("j"));
}

#[test]
fn source_item_lowers_conditional_update_if_with_tail_break() {
    // `if i == 0 { j = 1; break }` is the exit-bearing sibling shape: the
    // recipe records `then_exit = Break{1}` and the facade attaches the
    // (empty) break phi args through the same owner.
    let (ledger, body) = real_ledger(
        "function t() { loop(true) { local i = 0; local j = 0; if i == 0 { j = 1; break } } }",
    );
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
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
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = port.stmt_syntax(
        &port
            .body_stmt(&carrier, 2)
            .expect("located if statement"),
    )
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let ASTNode::Break { .. } = then_body.last().expect("tail break") else {
        panic!("fixture tail break")
    };
    let item = LoopCondBreakContinueItem::ConditionalUpdateIf {
        if_stmt: StmtRef::new(2),
        cond_view: CondBlockView::from_expr(condition),
        then_body: Some(
            try_build_no_exit_block_recipe(&then_body[..then_body.len() - 1], true)
                .expect("then no-exit recipe"),
        ),
        then_exit: Some(ExitKind::Break { depth: 1 }),
        else_body: None,
        else_exit: None,
    };
    let plans = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 2, &item)
        .expect("exit-bearing conditional update lowers");
    assert!(
        plans.iter().any(|plan| matches!(
            plan,
            CorePlan::Effect(CoreEffectPlan::ExitIf {
                exit: CoreExitPlan::BreakWithPhiArgs { depth: 1, .. },
                ..
            })
        )),
        "expected a break exit plan, got {plans:?}"
    );
    assert!(bindings.contains_key("j"));
}

#[test]
fn source_item_lowers_conditional_update_if_with_tail_continue() {
    // `if i == 0 { j = 1; continue }` carries a tail continue after the
    // update: the recipe records `then_exit = Continue{1}` and the owner
    // emits the continue exit plan through the same facade.
    let (ledger, body) = real_ledger(
        "function t() { loop(true) { local i = 0; local j = 0; if i == 0 { j = 1; continue } } }",
    );
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
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
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = port.stmt_syntax(&port.body_stmt(&carrier, 2).expect("located if statement"))
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let ASTNode::Continue { .. } = then_body.last().expect("tail continue") else {
        panic!("fixture tail continue")
    };
    let item = LoopCondBreakContinueItem::ConditionalUpdateIf {
        if_stmt: StmtRef::new(2),
        cond_view: CondBlockView::from_expr(condition),
        then_body: Some(
            try_build_no_exit_block_recipe(&then_body[..then_body.len() - 1], true)
                .expect("then no-exit recipe"),
        ),
        then_exit: Some(ExitKind::Continue { depth: 1 }),
        else_body: None,
        else_exit: None,
    };
    let plans = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 2, &item)
        .expect("continue-bearing conditional update lowers");
    assert!(
        plans.iter().any(|plan| matches!(
            plan,
            CorePlan::Effect(CoreEffectPlan::ExitIf {
                exit: CoreExitPlan::ContinueWithPhiArgs { depth: 1, .. }
                    | CoreExitPlan::Continue(1),
                ..
            })
        )),
        "expected a continue exit plan, got {plans:?}"
    );
    assert!(bindings.contains_key("j"));
}

#[test]
fn source_item_lowers_conditional_update_if_with_exit_only_else() {
    // `if i == 0 { j = 1 } else { break }`: the AST else exists but its only
    // statement is a tail exit, so the recipe records `else_body = None` and
    // `else_exit = Break{1}`. Else parity must key on
    // `else_body.is_some() || else_exit.is_some()`, not `else_body` alone.
    let (ledger, body) = real_ledger(
        "function t() { loop(true) { local i = 0; local j = 0; if i == 0 { j = 1 } else { break } } }",
    );
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
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
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = port.stmt_syntax(&port.body_stmt(&carrier, 2).expect("located if statement"))
    else {
        panic!("fixture if")
    };
    let ast_else = else_body.as_ref().expect("fixture else");
    let ASTNode::Break { .. } = ast_else.last().expect("else tail break") else {
        panic!("fixture else tail break")
    };
    let item = LoopCondBreakContinueItem::ConditionalUpdateIf {
        if_stmt: StmtRef::new(2),
        cond_view: CondBlockView::from_expr(condition),
        then_body: Some(
            try_build_no_exit_block_recipe(then_body, true).expect("then no-exit recipe"),
        ),
        then_exit: None,
        else_body: None,
        else_exit: Some(ExitKind::Break { depth: 1 }),
    };
    let plans = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 2, &item)
        .expect("exit-only else conditional update lowers");
    assert!(
        plans.iter().any(|plan| matches!(
            plan,
            CorePlan::Effect(CoreEffectPlan::ExitIf { .. })
        )),
        "expected an exit plan, got {plans:?}"
    );
    assert!(bindings.contains_key("j"));
}

#[test]
fn source_item_rejects_conditional_update_if_else_parity_drift() {
    // Recipe claims an else branch the located `if` does not carry.
    let (ledger, body) = real_ledger(
        "function t() { local i = 0; local j = 0; if i == 0 { j = 1 } }",
    );
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[2]
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let forged_else = try_build_no_exit_block_recipe(
        &[ASTNode::Assignment {
            target: Box::new(variable("j")),
            value: Box::new(integer(2)),
            span: crate::ast::Span::unknown(),
        }],
        true,
    )
    .expect("forged else recipe");
    let item = LoopCondBreakContinueItem::ConditionalUpdateIf {
        if_stmt: StmtRef::new(2),
        cond_view: CondBlockView::from_expr(condition),
        then_body: Some(
            try_build_no_exit_block_recipe(then_body, true).expect("then no-exit recipe"),
        ),
        then_exit: None,
        else_body: Some(forged_else),
        else_exit: None,
    };
    let error = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 2, &item)
        .expect_err("else parity drift is a named reject");
    assert!(error.contains("RecipeBodyMismatch"), "{error}");
}

#[test]
fn source_item_rejects_conditional_update_if_exit_parity_drift() {
    // Recipe records a tail break the located then branch does not carry.
    let (ledger, body) = real_ledger(
        "function t() { local i = 0; local j = 0; if i == 0 { j = 1 } }",
    );
    let carrier = located_body(&ledger, &body);
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[2]
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let item = LoopCondBreakContinueItem::ConditionalUpdateIf {
        if_stmt: StmtRef::new(2),
        cond_view: CondBlockView::from_expr(condition),
        then_body: Some(
            try_build_no_exit_block_recipe(then_body, true).expect("then no-exit recipe"),
        ),
        then_exit: Some(ExitKind::Break { depth: 1 }),
        else_body: None,
        else_exit: None,
    };
    let error = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 2, &item)
        .expect_err("exit parity drift is a named reject");
    assert!(error.contains("RecipeBodyMismatch"), "{error}");
}

#[test]
fn source_item_rejects_conditional_update_if_unsupported_shape() {
    // `if i == 0 { j = 1; j = 2 }` seals fine against the located branch but
    // the conditional-update owner declines the duplicate target update; the
    // arm must surface the named contract terminal, not a fallback.
    let (ledger, body) = real_ledger(
        "function t() { local i = 0; local j = 0; if i == 0 { j = 1; j = 2 } }",
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
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[2]
    else {
        panic!("fixture if")
    };
    assert!(else_body.is_none());
    let item = LoopCondBreakContinueItem::ConditionalUpdateIf {
        if_stmt: StmtRef::new(2),
        cond_view: CondBlockView::from_expr(condition),
        then_body: Some(
            try_build_no_exit_block_recipe(then_body, true).expect("then no-exit recipe"),
        ),
        then_exit: None,
        else_body: None,
        else_exit: None,
    };
    let error = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 2, &item)
        .expect_err("unsupported conditional update shape is a named reject");
    assert!(
        error.contains("loop-cond-item-conditional-update-unsupported"),
        "{error}"
    );
}
