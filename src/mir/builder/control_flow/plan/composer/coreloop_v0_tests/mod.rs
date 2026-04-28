//! Tests for CoreLoopComposer v0.

mod scan_with_init;
mod simple_while;
mod split_scan;

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::facts::feature_facts::LoopFeatureFacts;
use crate::mir::builder::control_flow::plan::facts::loop_types::LoopFacts;
use crate::mir::builder::control_flow::plan::facts::scan_shapes::{ConditionShape, StepShape};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{SkeletonFacts, SkeletonKind};

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

fn base_facts(skeleton_kind: SkeletonKind, features: LoopFeatureFacts) -> LoopFacts {
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
        loop_simple_while: None,
        loop_char_map: None,
        loop_array_join: None,
        string_is_integer: None,
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
        nested_loop_minimal: None,
        bool_predicate_scan: None,
        accum_const_loop: None,
        loop_break: None,
        loop_break_body_local: None,
    }
}
