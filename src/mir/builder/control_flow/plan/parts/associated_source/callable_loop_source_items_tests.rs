//! Item-level tests for `lower_loop_cond_source_item`: the located LoopCond
//! item dispatcher seals each issued recipe item against the co-sealed body
//! carrier and drives it through the neutral Parts machinery. Shared fixtures
//! live in `callable_loop_source_testkit`; provider/co-seal negatives live in
//! `callable_loop_source_tests`.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::callable_loop_source_testkit::{
    cataloged_body_source, drive_item, function_body_source, integer, located_body,
    real_core_method_ledger, real_core_method_ledger_with_placements, real_ledger, test_builder,
    variable,
};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::try_build_exit_only_block_recipe;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan};
use crate::mir::builder::control_flow::recipes::loop_cond_break_continue::LoopCondBreakContinueItem;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use crate::mir::builder::normal_callable_loop_source_port::CallableLoopSourceExpressionPortV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::{MirType, ValueId};

fn core_method_test_builder(
    ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
) -> (MirBuilder, ValueId) {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("t/1".to_owned());
    let receiver = builder.alloc_typed(MirType::Unknown);
    let parameter = builder.alloc_typed(MirType::String);
    builder
        .function_state
        .current_function
        .as_mut()
        .expect("test function")
        .params
        .extend([receiver, parameter]);
    let entry =
        PreparedCallableEntryValuesV1::instance_method(&builder, 1).expect("instance entry values");
    ledger
        .borrow_mut()
        .install_entry_values(&entry)
        .expect("entry install");
    (builder, parameter)
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
fn source_item_method_calls_consume_exact_core_method_rows() {
    let (ledger, body) = real_core_method_ledger(
        "function t(text) { loop(text.length() < 2) { local piece = text.substring(0, 1) } }",
    );
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let function_body = port
        .body(&body, &function_body_source())
        .expect("located function body");
    let loop_stmt = port
        .body_stmt(&function_body, 0)
        .expect("located loop statement");
    let condition = port
        .child_expr_from_stmt(&loop_stmt, ExprChildRoleV1::LoopCondition)
        .expect("located loop condition");
    let loop_body = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
    let body_stmt = port.body_stmt(&loop_body, 0).expect("located body item");
    let substring = port
        .child_expr_from_stmt(&body_stmt, ExprChildRoleV1::LocalInitializer(0))
        .expect("located substring initializer");
    let (mut builder, parameter) = core_method_test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);

    let (_, _, _, condition_effects) =
        PlanNormalizer::lower_compare_input(&port, condition, &mut builder, &BTreeMap::new())
            .expect("length lowers through source port");
    let length = condition_effects
        .iter()
        .find_map(|effect| match effect {
            CoreEffectPlan::MethodCall {
                dst: Some(dst),
                object,
                method,
                ..
            } if method == "length" => Some((*dst, *object)),
            _ => None,
        })
        .expect("length method effect");
    assert_eq!(length.1, parameter);
    assert_eq!(
        builder.function_state.type_ctx.get_type(length.0),
        Some(&MirType::Integer)
    );

    let (_, substring_effects) =
        PlanNormalizer::lower_value_input(&port, substring, &mut builder, &BTreeMap::new())
            .expect("substring lowers through source port");
    let substring = substring_effects
        .iter()
        .find_map(|effect| match effect {
            CoreEffectPlan::MethodCall {
                dst: Some(dst),
                object,
                method,
                args,
                ..
            } if method == "substring" => Some((*dst, *object, args.len())),
            _ => None,
        })
        .expect("substring method effect");
    assert_eq!(substring.1, parameter);
    assert_eq!(substring.2, 2);
    assert_eq!(
        builder.function_state.type_ctx.get_type(substring.0),
        Some(&MirType::String)
    );
}

#[test]
fn condition_position_substring_contracts_and_consumes_exact_row() {
    // `StringSubstring/2` at `Condition` is inside the bounded arm
    // vocabulary: the issued contract seals `placement == Condition` and the
    // same expression port consumes the exact row in the condition region.
    let (ledger, body, placements) = real_core_method_ledger_with_placements(
        "function t(text) { loop(text.substring(0, 1) == \" \" && text.length() < 2) { local i = 0 } }",
        2,
    );
    assert!(
        placements.values().all(|placement| *placement
            == crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition),
        "both armed calls sit in the condition: {placements:?}"
    );
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let function_body = port
        .body(&body, &function_body_source())
        .expect("located function body");
    let loop_stmt = port
        .body_stmt(&function_body, 0)
        .expect("located loop statement");
    let condition = port
        .child_expr_from_stmt(&loop_stmt, ExprChildRoleV1::LoopCondition)
        .expect("located loop condition");
    let equality = port
        .child_expr(&condition, ExprChildRoleV1::BinaryLeft)
        .expect("located equality arm");
    let substring = port
        .child_expr(&equality, ExprChildRoleV1::BinaryLeft)
        .expect("located substring call");
    let (mut builder, parameter) = core_method_test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);

    let (_, substring_effects) =
        PlanNormalizer::lower_value_input(&port, substring, &mut builder, &BTreeMap::new())
            .expect("condition substring lowers through source port");
    let substring = substring_effects
        .iter()
        .find_map(|effect| match effect {
            CoreEffectPlan::MethodCall {
                dst: Some(dst),
                object,
                method,
                args,
                ..
            } if method == "substring" => Some((*dst, *object, args.len())),
            _ => None,
        })
        .expect("substring method effect");
    assert_eq!(substring.1, parameter);
    assert_eq!(substring.2, 2);
    assert_eq!(
        builder.function_state.type_ctx.get_type(substring.0),
        Some(&MirType::String)
    );
}

#[test]
fn body_position_length_stays_unarmed() {
    // `StringLen/0` remains `Condition`-only: a body-position `length` call
    // issues no contract row and arms nothing.
    let (_, _, placements) = real_core_method_ledger_with_placements(
        "function t(text) { loop(text.length() < 2) { local n = text.length() } }",
        1,
    );
    assert_eq!(placements.len(), 1);
    assert!(placements.values().all(|placement| *placement
        == crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition));
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
fn source_item_consumes_me_receiver_site_for_method_call() {
    // `me.bump(7)` inside the loop body: the located Stmt arm must consume
    // the registered `Receiver` site through the source port. The test
    // builder never binds `me` in `variable_map`, so a name-based resolve
    // would fail with `me.method without bound receiver`; an emitted
    // MethodCall proves the site-consumption path ran.
    let (ledger, body) =
        real_ledger("function t() { local i = 0; loop(i < 2) { me.bump(7); i = i + 1 } }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let function_carrier = port
        .body(&body, &cataloged_body_source())
        .expect("located function body");
    let loop_stmt = port
        .body_stmt(&function_carrier, 1)
        .expect("located loop stmt");
    let carrier = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
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
    .expect("me.bump lowers through the consumed receiver site");
    assert!(
        plans.iter().any(|plan| matches!(
            plan,
            CorePlan::Effect(CoreEffectPlan::MethodCall { dst: None, .. })
        )),
        "expected a method call plan, got {plans:?}"
    );
}

#[test]
fn source_item_consumes_me_receiver_site_for_value_call() {
    // Value position: `local x = me.size()` routes through the MethodCall
    // value arm; the same receiver-site consumption must fire and emit a
    // `MethodCall` with a result `dst`.
    let (ledger, body) =
        real_ledger("function t() { local i = 0; loop(i < 2) { local x = me.size(); i = i + 1 } }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let function_carrier = port
        .body(&body, &cataloged_body_source())
        .expect("located function body");
    let loop_stmt = port
        .body_stmt(&function_carrier, 1)
        .expect("located loop stmt");
    let carrier = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
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
    .expect("local x = me.size() lowers through the consumed receiver site");
    assert!(
        plans.iter().any(|plan| matches!(
            plan,
            CorePlan::Effect(CoreEffectPlan::MethodCall { dst: Some(_), .. })
        )),
        "expected a result method call plan, got {plans:?}"
    );
    assert!(bindings.contains_key("x"));
}

#[test]
fn source_item_rejects_duplicate_me_receiver_site_consumption() {
    // Driving the same `me.bump(7)` item twice attempts to consume the same
    // `Receiver` site twice; the ledger's exactly-once contract must surface
    // the named reject, never a silent re-read.
    let (ledger, body) =
        real_ledger("function t() { local i = 0; loop(i < 2) { me.bump(7); i = i + 1 } }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let function_carrier = port
        .body(&body, &cataloged_body_source())
        .expect("located function body");
    let loop_stmt = port
        .body_stmt(&function_carrier, 1)
        .expect("located loop stmt");
    let carrier = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
    let mut builder = test_builder(&ledger);
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut bindings = BTreeMap::new();
    let item = LoopCondBreakContinueItem::Stmt(StmtRef::new(0));
    drive_item(&ledger, &carrier, &mut builder, &mut bindings, 0, &item)
        .expect("first me.bump lowers");
    let error = drive_item(&ledger, &carrier, &mut builder, &mut bindings, 0, &item)
        .expect_err("second consumption of the same receiver site rejects");
    assert!(error.contains("duplicate-variable-consumption"), "{error}");
}

#[test]
fn source_item_consumes_this_receiver_site_for_method_call() {
    // `this` in a plain callable resolves to the receiver binding too, so
    // its `Receiver` site is registered and consumed through the port the
    // same way as `me` — a `MethodCall` plan with the materialized receiver
    // is the pin.
    let (ledger, body) =
        real_ledger("function t() { local i = 0; loop(i < 2) { this.bump(7); i = i + 1 } }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let function_carrier = port
        .body(&body, &cataloged_body_source())
        .expect("located function body");
    let loop_stmt = port
        .body_stmt(&function_carrier, 1)
        .expect("located loop stmt");
    let carrier = port
        .child_body_from_stmt(&loop_stmt, BodyChildRoleV1::LoopBody)
        .expect("located loop body");
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
    .expect("this.bump lowers through the consumed receiver site");
    assert!(
        plans.iter().any(|plan| matches!(
            plan,
            CorePlan::Effect(CoreEffectPlan::MethodCall { dst: None, .. })
        )),
        "expected a method call plan, got {plans:?}"
    );
}

#[test]
fn port_declines_receiver_value_for_non_receiver_expression() {
    // The `exact_source_receiver_value` hook only intercepts `Me`/`This`
    // expressions; any other expression returns `Ok(None)` so the existing
    // receiver resolution path keeps owning it.
    let (ledger, _body) = real_ledger("function t() { local i = 0 }");
    let port = CallableLoopSourceExpressionPortV1::new(&ledger, None);
    let node = variable("x");
    let input = port.synthetic_expr(&node);
    let result = port
        .exact_source_receiver_value(&input)
        .expect("non-receiver expression declines cleanly");
    assert!(result.is_none());
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
