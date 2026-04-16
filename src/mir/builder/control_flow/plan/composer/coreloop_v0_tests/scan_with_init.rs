use super::v;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    ExitKindFacts, ExitMapFacts, ExitUsageFacts, LoopFeatureFacts, ValueJoinFacts,
};
use crate::mir::builder::control_flow::plan::facts::loop_types::{LoopFacts, ScanWithInitFacts};
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    ConditionShape, LengthMethod, StepShape,
};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{SkeletonFacts, SkeletonKind};
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeSet;

use super::super::coreloop_single_entry::try_compose_scan_with_init_unified;
use super::super::coreloop_v0::try_compose_core_loop_v0;

#[test]
fn coreloop_v0_composes_scan_with_init_subset() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(v("s")),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let mut kinds_present = BTreeSet::new();
    kinds_present.insert(ExitKindFacts::Return);
    let features = LoopFeatureFacts {
        nested_loop: false,
        exit_usage: ExitUsageFacts {
            has_break: false,
            has_continue: false,
            has_return: true,
            has_unwind: false,
        },
        exit_map: Some(ExitMapFacts { kinds_present }),
        value_join: None,
        cleanup: None,
    };
    let facts = LoopFacts {
        condition_shape: ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "s".to_string(),
            method: LengthMethod::Length,
        },
        step_shape: StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        },
        skeleton: SkeletonFacts {
            kind: SkeletonKind::Loop,
            ..Default::default()
        },
        features,
        scan_with_init: Some(ScanWithInitFacts {
            loop_var: "i".to_string(),
            haystack: "s".to_string(),
            needle: "ch".to_string(),
            step_lit: 1,
            dynamic_needle: false,
        }),
        split_scan: None,
        loop_simple_while: None,
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
    builder.enter_function_for_test("coreloop_v0_scan_with_init".to_string());
    let i_init = builder.alloc_typed(MirType::Integer);
    let s_val = builder.alloc_typed(MirType::String);
    let ch_val = builder.alloc_typed(MirType::String);
    builder
        .variable_ctx
        .variable_map
        .insert("i".to_string(), i_init);
    builder
        .variable_ctx
        .variable_map
        .insert("s".to_string(), s_val);
    builder
        .variable_ctx
        .variable_map
        .insert("ch".to_string(), ch_val);
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v0_scan", false, false);
    let composed = try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(matches!(composed, Some(CorePlan::Loop(_))));
}

#[test]
fn coreloop_v0_rejects_scan_with_init_when_shapes_mismatch() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let mut kinds_present = BTreeSet::new();
    kinds_present.insert(ExitKindFacts::Return);
    let features = LoopFeatureFacts {
        nested_loop: false,
        exit_usage: ExitUsageFacts {
            has_break: false,
            has_continue: false,
            has_return: true,
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
        scan_with_init: Some(ScanWithInitFacts {
            loop_var: "i".to_string(),
            haystack: "s".to_string(),
            needle: "ch".to_string(),
            step_lit: 1,
            dynamic_needle: false,
        }),
        split_scan: None,
        loop_simple_while: None,
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
    builder.enter_function_for_test("coreloop_v0_scan_mismatch".to_string());
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v0_scan", false, false);
    let err = try_compose_core_loop_v0(&mut builder, &canonical, &ctx)
        .expect_err("scan_with_init mismatch should fail fast");
    assert!(
        err.contains("carrier_init_missing"),
        "scan_with_init mismatch should report missing carrier init"
    );
}

#[test]
fn coreloop_v0_rejects_scan_with_init_value_join_direct() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(ASTNode::MethodCall {
            object: Box::new(v("s")),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    };
    let mut kinds_present = BTreeSet::new();
    kinds_present.insert(ExitKindFacts::Return);
    let features = LoopFeatureFacts {
        nested_loop: false,
        exit_usage: ExitUsageFacts {
            has_break: false,
            has_continue: false,
            has_return: true,
            has_unwind: false,
        },
        exit_map: Some(ExitMapFacts { kinds_present }),
        value_join: Some(ValueJoinFacts { needed: true }),
        cleanup: None,
    };
    let facts = LoopFacts {
        condition_shape: ConditionShape::VarLessLength {
            idx_var: "i".to_string(),
            haystack_var: "s".to_string(),
            method: LengthMethod::Length,
        },
        step_shape: StepShape::AssignAddConst {
            var: "i".to_string(),
            k: 1,
        },
        skeleton: SkeletonFacts {
            kind: SkeletonKind::Loop,
            ..Default::default()
        },
        features,
        scan_with_init: Some(ScanWithInitFacts {
            loop_var: "i".to_string(),
            haystack: "s".to_string(),
            needle: "ch".to_string(),
            step_lit: 1,
            dynamic_needle: false,
        }),
        split_scan: None,
        loop_simple_while: None,
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
    builder.enter_function_for_test("coreloop_v0_scan_with_init_join".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v0_scan_with_init_join",
        false,
        false,
    );
    let composed = try_compose_scan_with_init_unified(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(
        composed.is_none(),
        "scan_with_init rejects value_join_needed"
    );
}
