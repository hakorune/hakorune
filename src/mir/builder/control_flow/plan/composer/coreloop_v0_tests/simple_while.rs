use super::{base_facts, lit_int, v};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    ExitKindFacts, ExitMapFacts, ExitUsageFacts, LoopFeatureFacts, ValueJoinFacts,
};
use crate::mir::builder::control_flow::plan::facts::loop_simple_while_facts::LoopSimpleWhileFacts;
use crate::mir::builder::control_flow::plan::facts::loop_types::LoopFacts;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{ConditionShape, StepShape};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{SkeletonFacts, SkeletonKind};
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeSet;

use super::super::coreloop_v0::try_compose_core_loop_v0;

#[test]
fn coreloop_v0_returns_none_for_non_loop_skeleton() {
    let facts = base_facts(SkeletonKind::If2, LoopFeatureFacts::default());
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v0_non_loop".to_string());
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v0_non_loop", false, false);
    let composed = try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
}

#[test]
fn coreloop_v0_returns_none_when_value_join_needed() {
    let features = LoopFeatureFacts {
        value_join: Some(ValueJoinFacts { needed: true }),
        ..LoopFeatureFacts::default()
    };
    let facts = base_facts(SkeletonKind::Loop, features);
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v0_value_join".to_string());
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v0_value_join", false, false);
    let composed = try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
}

#[test]
fn coreloop_v0_composes_simple_while_route() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(lit_int(3)),
        span: Span::unknown(),
    };
    let loop_increment = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(v("i")),
        right: Box::new(lit_int(1)),
        span: Span::unknown(),
    };
    let facts = LoopFacts {
        condition_shape: ConditionShape::Unknown,
        step_shape: StepShape::Unknown,
        skeleton: SkeletonFacts {
            kind: SkeletonKind::Loop,
            ..Default::default()
        },
        features: LoopFeatureFacts::default(),
        scan_with_init: None,
        split_scan: None,
        loop_simple_while: Some(LoopSimpleWhileFacts {
            loop_var: "i".to_string(),
            condition: condition.clone(),
            loop_increment: loop_increment.clone(),
        }),
        loop_char_map: None,
        loop_array_join: None,
        string_is_integer: None,
        starts_with: None,
        int_to_str: None,
        escape_map: None,
        split_lines: None,
        skip_whitespace: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        if_phi_join: None,
        loop_continue_only: None,
        loop_true_early_exit: None,
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
        bool_predicate_scan: None,
        accum_const_loop: None,
        loop_break: None,
        loop_break_body_local: None,
        nested_loop_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v0_simple_while".to_string());
    let init = builder.alloc_typed(MirType::Integer);
    builder
        .variable_ctx
        .variable_map
        .insert("i".to_string(), init);
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v0_simple_while", false, false);
    let composed = try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(matches!(composed, Some(CorePlan::Loop(_))));
}

#[test]
fn coreloop_v0_returns_none_when_exitmap_present() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(lit_int(2)),
        span: Span::unknown(),
    };
    let loop_increment = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(v("i")),
        right: Box::new(lit_int(1)),
        span: Span::unknown(),
    };
    let mut kinds_present = BTreeSet::new();
    kinds_present.insert(ExitKindFacts::Break);
    let features = LoopFeatureFacts {
        nested_loop: false,
        exit_usage: ExitUsageFacts {
            has_break: true,
            has_continue: false,
            has_return: false,
            has_unwind: false,
        },
        exit_map: Some(ExitMapFacts { kinds_present }),
        value_join: None,
        cleanup: None,
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
        loop_simple_while: Some(LoopSimpleWhileFacts {
            loop_var: "i".to_string(),
            condition: condition.clone(),
            loop_increment: loop_increment.clone(),
        }),
        loop_char_map: None,
        loop_array_join: None,
        string_is_integer: None,
        starts_with: None,
        int_to_str: None,
        escape_map: None,
        split_lines: None,
        skip_whitespace: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        if_phi_join: None,
        loop_continue_only: None,
        loop_true_early_exit: None,
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
        bool_predicate_scan: None,
        accum_const_loop: None,
        loop_break: None,
        loop_break_body_local: None,
        nested_loop_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v0_exitmap".to_string());
    let init = builder.alloc_typed(MirType::Integer);
    builder
        .variable_ctx
        .variable_map
        .insert("i".to_string(), init);
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v0_exitmap", false, false);
    let composed = try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
}
