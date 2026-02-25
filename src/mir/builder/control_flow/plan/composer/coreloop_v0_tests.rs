//! Tests for CoreLoopComposer v0.

use super::coreloop_single_entry::{
    try_compose_scan_with_init_unified, try_compose_split_scan_unified,
};
use super::coreloop_v0::try_compose_core_loop_v0;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    ExitKindFacts, ExitMapFacts, ExitUsageFacts, LoopFeatureFacts, ValueJoinFacts,
};
use crate::mir::builder::control_flow::plan::facts::loop_types::LoopFacts;
use crate::mir::builder::control_flow::plan::facts::loop_types::ScanWithInitFacts;
use crate::mir::builder::control_flow::plan::facts::pattern1_simplewhile_facts::Pattern1SimpleWhileFacts;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
    ConditionShape, LengthMethod, SplitScanShape, StepShape,
};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
    SkeletonFacts, SkeletonKind,
};
use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopPatternContext;
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

fn base_facts(
    skeleton_kind: SkeletonKind,
    features: LoopFeatureFacts,
) -> LoopFacts {
    LoopFacts {
        condition_shape: ConditionShape::Unknown,
        step_shape: StepShape::Unknown,
        skeleton: SkeletonFacts {
            kind: skeleton_kind,
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
        pattern6_nested_minimal: None,
        pattern8_bool_predicate_scan: None,
        pattern9_accum_const_loop: None,
        pattern2_break: None,
        pattern2_loopbodylocal: None,
    }
}

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
    let ctx = LoopPatternContext::new(&condition, &[], "coreloop_v0_non_loop", false, false);
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
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
    let ctx = LoopPatternContext::new(&condition, &[], "coreloop_v0_value_join", false, false);
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
}

#[test]
fn coreloop_v0_composes_pattern1_skeleton() {
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
        pattern1_simplewhile: Some(Pattern1SimpleWhileFacts {
            loop_var: "i".to_string(),
            condition: condition.clone(),
            loop_increment: loop_increment.clone(),
        }),
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
    builder.enter_function_for_test("coreloop_v0_pattern1".to_string());
    let init = builder.alloc_typed(MirType::Integer);
    builder
        .variable_ctx
        .variable_map
        .insert("i".to_string(), init);
    let ctx = LoopPatternContext::new(&condition, &[], "coreloop_v0_pattern1", false, false);
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
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
        pattern1_simplewhile: Some(Pattern1SimpleWhileFacts {
            loop_var: "i".to_string(),
            condition: condition.clone(),
            loop_increment: loop_increment.clone(),
        }),
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
    builder.enter_function_for_test("coreloop_v0_exitmap".to_string());
    let init = builder.alloc_typed(MirType::Integer);
    builder
        .variable_ctx
        .variable_map
        .insert("i".to_string(), init);
    let ctx = LoopPatternContext::new(&condition, &[], "coreloop_v0_exitmap", false, false);
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
}

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
    let ctx = LoopPatternContext::new(&condition, &[], "coreloop_v0_scan", false, false);
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
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
    builder.enter_function_for_test("coreloop_v0_scan_mismatch".to_string());
    let ctx = LoopPatternContext::new(&condition, &[], "coreloop_v0_scan", false, false);
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(composed.is_none());
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
    builder.enter_function_for_test("coreloop_v0_scan_with_init_join".to_string());
    let ctx = LoopPatternContext::new(
        &condition,
        &[],
        "coreloop_v0_scan_with_init_join",
        false,
        false,
    );
    let composed =
        try_compose_scan_with_init_unified(&mut builder, &canonical, &ctx)
            .expect("Ok");
    assert!(
        composed.is_none(),
        "scan_with_init rejects value_join_needed"
    );
}

#[test]
fn coreloop_v0_composes_split_scan_subset() {
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
    builder.enter_function_for_test("coreloop_v0_split_scan".to_string());
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
        LoopPatternContext::new(&condition, &[], "coreloop_v0_split_scan", false, false);
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(matches!(composed, Some(CorePlan::Loop(_))));
}

#[test]
fn coreloop_v0_rejects_split_scan_value_join_entrypoint() {
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
    builder.enter_function_for_test("coreloop_v0_split_scan_join".to_string());
    let ctx = LoopPatternContext::new(
        &condition,
        &[],
        "coreloop_v0_split_scan_join",
        false,
        false,
    );
    let composed =
        try_compose_core_loop_v0(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(
        composed.is_none(),
        "v0 entrypoint rejects split_scan value_join_needed"
    );
}

#[test]
fn coreloop_v0_rejects_split_scan_value_join_direct() {
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
    builder.enter_function_for_test("coreloop_v0_split_scan_join_direct".to_string());
    let ctx = LoopPatternContext::new(
        &condition,
        &[],
        "coreloop_v0_split_scan_join_direct",
        false,
        false,
    );
    let composed =
        try_compose_split_scan_unified(&mut builder, &canonical, &ctx).expect("Ok");
    assert!(
        composed.is_none(),
        "split_scan rejects value_join_needed"
    );
}
