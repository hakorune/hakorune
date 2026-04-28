use super::{lit_int, v};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::domain::LoopTrueEarlyExitKind;
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    CleanupFacts, CleanupKindFacts, LoopFeatureFacts, ValueJoinFacts,
};
use crate::mir::builder::control_flow::plan::facts::loop_true_early_exit_facts::LoopTrueEarlyExitFacts;
use crate::mir::builder::control_flow::plan::facts::loop_types::LoopFacts;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{ConditionShape, StepShape};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{SkeletonFacts, SkeletonKind};
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeSet;

use super::super::coreloop_v1::try_compose_core_loop_v1_loop_true_early_exit;

#[test]
fn coreloop_v1_composes_loop_true_early_exit_with_value_join() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let exit_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(v("i")),
        right: Box::new(lit_int(2)),
        span: Span::unknown(),
    };
    let carrier_update = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(v("sum")),
        right: Box::new(lit_int(1)),
        span: Span::unknown(),
    };
    let loop_increment = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(v("i")),
        right: Box::new(lit_int(1)),
        span: Span::unknown(),
    };

    let features = LoopFeatureFacts {
        value_join: Some(ValueJoinFacts { needed: true }),
        ..LoopFeatureFacts::default()
    };
    let facts = LoopFacts {
        condition_shape: ConditionShape::Unknown,
        step_shape: StepShape::Unknown,
        skeleton: SkeletonFacts {
            kind: SkeletonKind::Loop,
            ..Default::default()
        },
        features,
        scan_with_init: None,
        split_scan: None,
        loop_simple_while: None,
        loop_char_map: None,
        loop_array_join: None,
        string_is_integer: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        if_phi_join: None,
        loop_continue_only: None,
        loop_true_early_exit: Some(LoopTrueEarlyExitFacts {
            loop_var: "i".to_string(),
            exit_kind: LoopTrueEarlyExitKind::Break,
            exit_condition,
            exit_value: None,
            carrier_var: Some("sum".to_string()),
            carrier_update: Some(carrier_update),
            loop_increment,
        }),
        loop_true_break_continue: None,
        loop_cond_break_continue: None,
        loop_cond_continue_only: None,
        loop_cond_continue_with_return: None,
        loop_cond_return_in_body: None,
        loop_scan_v0: None,
        loop_scan_methods_block_v0: None,
        loop_scan_methods_v0: None,
        loop_scan_phi_vars_v0: None,
        loop_bundle_resolver_v0: None,
        loop_collect_using_entries_v0: None,
        nested_loop_minimal: None,
        bool_predicate_scan: None,
        accum_const_loop: None,
        loop_break: None,
        loop_break_body_local: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_loop_true_early_exit".to_string());
    let i_val = builder.alloc_typed(MirType::Integer);
    let sum_val = builder.alloc_typed(MirType::Integer);
    builder
        .variable_ctx
        .variable_map
        .insert("i".to_string(), i_val);
    builder
        .variable_ctx
        .variable_map
        .insert("sum".to_string(), sum_val);
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_loop_true_early_exit",
        false,
        false,
    );
    let composed =
        try_compose_core_loop_v1_loop_true_early_exit(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(matches!(
        composed,
        Some(crate::mir::builder::control_flow::plan::CorePlan::Loop(_))
    ));
}

#[test]
fn coreloop_v1_rejects_loop_true_early_exit_with_cleanup() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let mut kinds_present = BTreeSet::new();
    kinds_present.insert(CleanupKindFacts::Return);
    let features = LoopFeatureFacts {
        cleanup: Some(CleanupFacts { kinds_present }),
        value_join: Some(ValueJoinFacts { needed: true }),
        ..LoopFeatureFacts::default()
    };
    let facts = LoopFacts {
        condition_shape: ConditionShape::Unknown,
        step_shape: StepShape::Unknown,
        skeleton: SkeletonFacts {
            kind: SkeletonKind::Loop,
            ..Default::default()
        },
        features,
        scan_with_init: None,
        split_scan: None,
        loop_simple_while: None,
        loop_char_map: None,
        loop_array_join: None,
        string_is_integer: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        if_phi_join: None,
        loop_continue_only: None,
        loop_true_early_exit: Some(LoopTrueEarlyExitFacts {
            loop_var: "i".to_string(),
            exit_kind: LoopTrueEarlyExitKind::Break,
            exit_condition: condition.clone(),
            exit_value: None,
            carrier_var: Some("sum".to_string()),
            carrier_update: Some(lit_int(0)),
            loop_increment: lit_int(0),
        }),
        loop_true_break_continue: None,
        loop_cond_break_continue: None,
        loop_cond_continue_only: None,
        loop_cond_continue_with_return: None,
        loop_cond_return_in_body: None,
        loop_scan_v0: None,
        loop_scan_methods_block_v0: None,
        loop_scan_methods_v0: None,
        loop_scan_phi_vars_v0: None,
        loop_bundle_resolver_v0: None,
        loop_collect_using_entries_v0: None,
        nested_loop_minimal: None,
        bool_predicate_scan: None,
        accum_const_loop: None,
        loop_break: None,
        loop_break_body_local: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_loop_true_early_exit_cleanup".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_loop_true_early_exit_cleanup",
        false,
        false,
    );
    let composed =
        try_compose_core_loop_v1_loop_true_early_exit(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
}
