//! Tests for coreloop_v1 pattern composers.

use super::coreloop_v1::{
    try_compose_core_loop_v1_pattern2_break, try_compose_core_loop_v1_pattern3_ifphi,
    try_compose_core_loop_v1_pattern5_infinite_early_exit,
};
use super::coreloop_single_entry::{
    try_compose_scan_with_init_unified, try_compose_split_scan_unified,
};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    CleanupFacts, CleanupKindFacts, ExitKindFacts, ExitMapFacts, LoopFeatureFacts,
    ValueJoinFacts,
};
use crate::mir::builder::control_flow::plan::facts::loop_types::{
    LoopFacts, ScanWithInitFacts,
};
use crate::mir::builder::control_flow::plan::facts::pattern3_ifphi_facts::Pattern3IfPhiFacts;
use crate::mir::builder::control_flow::plan::facts::pattern5_infinite_early_exit_facts::Pattern5InfiniteEarlyExitFacts;
use crate::mir::builder::control_flow::plan::facts::pattern2_break_types::Pattern2BreakFacts;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    ConditionShape, SplitScanShape, StepShape,
};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
    SkeletonFacts, SkeletonKind,
};
use crate::mir::builder::control_flow::plan::domain::LoopTrueEarlyExitKind;
use crate::mir::builder::control_flow::plan::LoopBreakStepPlacement;
use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeSet;

fn v(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn lit_int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

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
        split_scan: Some(crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts {
            s_var: "s".to_string(),
            sep_var: "sep".to_string(),
            result_var: "result".to_string(),
            i_var: "i".to_string(),
            start_var: "start".to_string(),
            shape: SplitScanShape::Minimal,
        }),
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
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
    let ctx =
        LoopRouteContext::new(&condition, &[], "coreloop_v1_split_scan", false, false);
    let composed =
        try_compose_split_scan_unified(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(matches!(composed, Some(crate::mir::builder::control_flow::plan::CorePlan::Loop(_))));
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
        split_scan: Some(crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts {
            s_var: "s".to_string(),
            sep_var: "sep".to_string(),
            result_var: "result".to_string(),
            i_var: "i".to_string(),
            start_var: "start".to_string(),
            shape: SplitScanShape::Minimal,
        }),
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_split_scan_no_join".to_string());
    let ctx =
        LoopRouteContext::new(&condition, &[], "coreloop_v1_split_scan_no_join", false, false);
    // Without value_join, unified helper uses v0 gate (coreloop_base_gate) which rejects
    let composed =
        try_compose_split_scan_unified(&mut builder, &canonical, &ctx).expect("Ok");
    // v0 path: returns None because coreloop_base_gate fails for Loop without pattern facts
    assert!(
        composed.is_none(),
        "split_scan without value_join requires coreloop_base_gate"
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
        split_scan: Some(crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts {
            s_var: "s".to_string(),
            sep_var: "sep".to_string(),
            result_var: "result".to_string(),
            i_var: "i".to_string(),
            start_var: "start".to_string(),
            shape: SplitScanShape::Minimal,
        }),
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_split_scan_exit".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_split_scan_exit",
        false,
        false,
    );
    let composed =
        try_compose_split_scan_unified(&mut builder, &canonical, &ctx).expect("Ok");
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
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
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
    let composed =
        try_compose_scan_with_init_unified(&mut builder, &canonical, &ctx)
            .expect("Ok");
    assert!(
        composed.is_none(),
        "scan_with_init with value_join_needed is rejected"
    );
}

#[test]
fn coreloop_v1_composes_pattern2_with_value_join() {
    let loop_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(lit_int(3)),
        span: Span::unknown(),
    };
    let break_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(v("i")),
        right: Box::new(lit_int(1)),
        span: Span::unknown(),
    };
    let carrier_update_in_body = ASTNode::BinaryOp {
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
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: Some(Pattern2BreakFacts {
            loop_var: "i".to_string(),
            carrier_var: "sum".to_string(),
            loop_condition: loop_condition.clone(),
            break_condition,
            carrier_update_in_break: None,
            carrier_update_in_body,
            loop_increment,
            step_placement: LoopBreakStepPlacement::Last,
        }),
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_pattern2".to_string());
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
    let ctx =
        LoopRouteContext::new(&loop_condition, &[], "coreloop_v1_pattern2", false, false);
    let composed =
        try_compose_core_loop_v1_pattern2_break(&mut builder, &canonical, &ctx)
            .expect("Ok");
    assert!(matches!(composed, Some(crate::mir::builder::control_flow::plan::CorePlan::Loop(_))));
}

#[test]
fn coreloop_v1_rejects_pattern2_with_cleanup() {
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
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: Some(Pattern2BreakFacts {
            loop_var: "i".to_string(),
            carrier_var: "sum".to_string(),
            loop_condition: condition.clone(),
            break_condition: condition.clone(),
            carrier_update_in_break: None,
            carrier_update_in_body: lit_int(0),
            loop_increment: lit_int(0),
            step_placement: LoopBreakStepPlacement::Last,
        }),
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_pattern2_cleanup".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_pattern2_cleanup",
        false,
        false,
    );
    let composed =
        try_compose_core_loop_v1_pattern2_break(&mut builder, &canonical, &ctx)
            .expect("Ok");
    assert!(composed.is_none());
}

#[test]
fn coreloop_v1_composes_pattern5_with_value_join() {
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
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: Some(Pattern5InfiniteEarlyExitFacts {
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
        pattern6_nested_minimal: None,
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_pattern5".to_string());
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
    let ctx =
        LoopRouteContext::new(&condition, &[], "coreloop_v1_pattern5", false, false);
    let composed =
        try_compose_core_loop_v1_pattern5_infinite_early_exit(
            &mut builder, &canonical, &ctx,
        )
        .expect("Ok");
    assert!(matches!(composed, Some(crate::mir::builder::control_flow::plan::CorePlan::Loop(_))));
}

#[test]
fn coreloop_v1_rejects_pattern5_with_cleanup() {
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
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: None,
        pattern4_continue: None,
        pattern5_infinite_early_exit: Some(Pattern5InfiniteEarlyExitFacts {
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
        pattern6_nested_minimal: None,
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_pattern5_cleanup".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_pattern5_cleanup",
        false,
        false,
    );
    let composed =
        try_compose_core_loop_v1_pattern5_infinite_early_exit(
            &mut builder, &canonical, &ctx,
        )
        .expect("Ok");
    assert!(composed.is_none());
}

#[test]
fn coreloop_v1_composes_pattern3_with_value_join() {
    let condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(v("i")),
        right: Box::new(lit_int(3)),
        span: Span::unknown(),
    };
    let if_condition = ASTNode::BinaryOp {
        operator: BinaryOperator::Greater,
        left: Box::new(v("i")),
        right: Box::new(lit_int(0)),
        span: Span::unknown(),
    };
    let then_update = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(v("sum")),
        right: Box::new(lit_int(1)),
        span: Span::unknown(),
    };
    let else_update = ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(v("sum")),
        right: Box::new(lit_int(0)),
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
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: Some(Pattern3IfPhiFacts {
            loop_var: "i".to_string(),
            carrier_var: "sum".to_string(),
            condition: condition.clone(),
            if_condition,
            then_update,
            else_update,
            loop_increment,
        }),
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_pattern3".to_string());
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
    let ctx =
        LoopRouteContext::new(&condition, &[], "coreloop_v1_pattern3", false, false);
    let composed =
        try_compose_core_loop_v1_pattern3_ifphi(&mut builder, &canonical, &ctx)
            .expect("Ok");
    assert!(matches!(composed, Some(crate::mir::builder::control_flow::plan::CorePlan::Loop(_))));
}

#[test]
fn coreloop_v1_rejects_pattern3_with_cleanup() {
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
        pattern1_simplewhile: None,
        pattern1_char_map: None,
        pattern1_array_join: None,
        pattern_is_integer: None,

        pattern_starts_with: None,


        pattern_int_to_str: None,


        pattern_escape_map: None,


        pattern_split_lines: None,



        pattern_skip_ws: None,
        generic_loop_v0: None,
        generic_loop_v1: None,
        pattern3_ifphi: Some(Pattern3IfPhiFacts {
            loop_var: "i".to_string(),
            carrier_var: "sum".to_string(),
            condition: condition.clone(),
            if_condition: condition.clone(),
            then_update: lit_int(0),
            else_update: lit_int(0),
            loop_increment: lit_int(0),
        }),
        pattern4_continue: None,
        pattern5_infinite_early_exit: None,
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
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
        pattern6_nested_minimal: None,
    };
    let canonical = canonicalize_loop_facts(facts);
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("coreloop_v1_pattern3_cleanup".to_string());
    let ctx = LoopRouteContext::new(
        &condition,
        &[],
        "coreloop_v1_pattern3_cleanup",
        false,
        false,
    );
    let composed =
        try_compose_core_loop_v1_pattern3_ifphi(&mut builder, &canonical, &ctx)
            .expect("Ok");
    assert!(composed.is_none());
}
