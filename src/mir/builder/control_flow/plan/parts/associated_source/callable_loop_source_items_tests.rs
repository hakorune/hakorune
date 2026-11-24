//! Shared checks for the located callable expression port and resolver sources
//! item dispatcher seals each issued recipe item against the co-sealed body
//! carrier and drives it through the neutral Parts machinery. Shared fixtures
//! live in `callable_loop_source_testkit`; provider/co-seal negatives live in
//! `callable_loop_source_tests`.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use super::callable_loop_source_testkit::{
    cataloged_body_source, function_body_source, integer, located_body,
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
    // `StringLen/0` at body position requires receiver text evidence: a
    // parameter receiver carries no initializer, so it issues no contract
    // row and stays unarmed — only the condition-position row remains.
    let (_, _, placements) = real_core_method_ledger_with_placements(
        "function t(text) { loop(text.length() < 2) { local n = text.length() } }",
        1,
    );
    assert_eq!(placements.len(), 1);
    assert!(placements.values().all(|placement| *placement
        == crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition));
}

#[test]
fn body_position_length_with_text_evidence_arms() {
    // `chunk_data`-shape: the receiver's sole initializer is a
    // `TextToCaller`-minting `substring` call, so `piece.length()` at body
    // position arms beside the substring row.
    let (_, _, placements) = real_core_method_ledger_with_placements(
        "function t() { local text = \"abcd\" local i = 0 loop(i < 2) { local piece = text.substring(0, 1) local n = piece.length() } }",
        2,
    );
    assert_eq!(placements.len(), 2);
    assert!(placements.values().all(|placement| *placement
        == crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body));
}

#[test]
fn body_position_length_without_text_evidence_stays_unarmed() {
    // A non-text local initializer (`local s = 0`) fails the evidence gate:
    // `s.length()` at body position issues nothing and stays unarmed.
    let (_, _, placements) = real_core_method_ledger_with_placements(
        "function t(text) { local s = 0 loop(s < 2) { local n = s.length() } }",
        0,
    );
    assert!(placements.is_empty());
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
