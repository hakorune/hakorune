use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    ExitKindFacts, ExitMapFacts, LoopFeatureFacts, ValueJoinFacts,
};
use crate::mir::builder::control_flow::plan::facts::loop_types::{
    LoopFacts, ScanWithInitFacts, SplitScanFacts,
};
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    ConditionShape, SplitScanShape, StepShape,
};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{SkeletonFacts, SkeletonKind};
use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeSet;

use super::super::coreloop_single_entry::{
    try_compose_scan_with_init_unified, try_compose_split_scan_unified,
};

#[test]
fn coreloop_v1_composes_split_scan_with_value_join() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
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
        split_scan: Some(SplitScanFacts {
            s_var: "s".to_string(),
            sep_var: "sep".to_string(),
            result_var: "result".to_string(),
            i_var: "i".to_string(),
            start_var: "start".to_string(),
            shape: SplitScanShape::Minimal,
        }),
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
    builder.enter_function_for_test("coreloop_v1_split_scan".to_string());
    let s_val = builder.alloc_typed(MirType::String);
    let sep_val = builder.alloc_typed(MirType::String);
    let result_val = builder.alloc_typed(MirType::Array(Box::new(MirType::String)));
    let i_val = builder.alloc_typed(MirType::Integer);
    let start_val = builder.alloc_typed(MirType::Integer);
    builder
        .variable_ctx
        .variable_map
        .insert("s".to_string(), s_val);
    builder
        .variable_ctx
        .variable_map
        .insert("sep".to_string(), sep_val);
    builder
        .variable_ctx
        .variable_map
        .insert("result".to_string(), result_val);
    builder
        .variable_ctx
        .variable_map
        .insert("i".to_string(), i_val);
    builder
        .variable_ctx
        .variable_map
        .insert("start".to_string(), start_val);
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v1_split_scan", false, false);
    let composed = try_compose_split_scan_unified(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(matches!(
        composed,
        Some(crate::mir::builder::control_flow::plan::CorePlan::Loop(_))
    ));
}

#[test]
fn coreloop_v1_rejects_split_scan_without_value_join() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
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
        split_scan: Some(SplitScanFacts {
            s_var: "s".to_string(),
            sep_var: "sep".to_string(),
            result_var: "result".to_string(),
            i_var: "i".to_string(),
            start_var: "start".to_string(),
            shape: SplitScanShape::Minimal,
        }),
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
    builder.enter_function_for_test("coreloop_v1_split_scan_no_join".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_split_scan_no_join",
        false,
        false,
    );
    // Without value_join, the unified helper currently fails the lower with a contract error.
    let err = try_compose_split_scan_unified(&mut builder, &canonical, &ctx)
        .expect_err("split_scan without value_join should fail fast");
    assert!(
        err.contains("carrier_init_missing"),
        "split_scan without value_join should report the missing carrier init gate"
    );
}

#[test]
fn coreloop_v1_rejects_split_scan_with_disallowed_exitmap() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let mut kinds_present = BTreeSet::new();
    kinds_present.insert(ExitKindFacts::Break);
    let features = LoopFeatureFacts {
        exit_map: Some(ExitMapFacts { kinds_present }),
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
        split_scan: Some(SplitScanFacts {
            s_var: "s".to_string(),
            sep_var: "sep".to_string(),
            result_var: "result".to_string(),
            i_var: "i".to_string(),
            start_var: "start".to_string(),
            shape: SplitScanShape::Minimal,
        }),
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
    builder.enter_function_for_test("coreloop_v1_split_scan_exit".to_string());
    let ctx = LoopRouteContext::new(&condition, &[], "coreloop_v1_split_scan_exit", false, false);
    let composed = try_compose_split_scan_unified(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
}

#[test]
fn unified_scan_with_init_rejects_value_join() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
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
    builder.enter_function_for_test("coreloop_v1_scan_with_init_join".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_scan_with_init_join",
        false,
        false,
    );
    let composed = try_compose_scan_with_init_unified(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(
        composed.is_none(),
        "scan_with_init with value_join_needed is rejected"
    );
}
