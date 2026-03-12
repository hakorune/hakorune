//! Phase 29ai P0: Canonicalize Facts (pure transform) — skeleton

use crate::mir::builder::control_flow::plan::facts::feature_facts::{
    CleanupKindFacts, ExitKindFacts, ExitUsageFacts,
};
use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;
use crate::mir::builder::control_flow::plan::facts::LoopFacts;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct CanonicalLoopFacts {
    pub facts: LoopFacts,
    pub skeleton_kind: SkeletonKind,
    pub nested_loop: bool,
    pub exit_usage: ExitUsageFacts,
    pub exit_kinds_present: BTreeSet<ExitKindFacts>,
    pub cleanup_kinds_present: BTreeSet<CleanupKindFacts>,
    pub value_join_needed: bool,
}

pub(in crate::mir::builder) fn canonicalize_loop_facts(facts: LoopFacts) -> CanonicalLoopFacts {
    CanonicalLoopFacts {
        skeleton_kind: facts.skeleton.kind,
        nested_loop: facts.features.nested_loop,
        exit_usage: facts.features.exit_usage.clone(),
        exit_kinds_present: facts
            .features
            .exit_map
            .as_ref()
            .map(|map| map.kinds_present.clone())
            .unwrap_or_default(),
        cleanup_kinds_present: facts
            .features
            .cleanup
            .as_ref()
            .map(|cleanup| cleanup.kinds_present.clone())
            .unwrap_or_default(),
        value_join_needed: facts.features.value_join.is_some(),
        facts,
    }
}

#[cfg(test)]
mod tests {
    use super::canonicalize_loop_facts;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::feature_facts::{
        CleanupFacts, CleanupKindFacts, ExitKindFacts, ExitMapFacts, ExitUsageFacts,
        LoopFeatureFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{ConditionShape, StepShape};
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;
    use crate::mir::builder::control_flow::plan::facts::LoopFacts;
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
    fn canonical_projects_skeleton_and_exit_usage() {
        let mut kinds_present = BTreeSet::new();
        kinds_present.insert(ExitKindFacts::Break);
        kinds_present.insert(ExitKindFacts::Continue);
        kinds_present.insert(ExitKindFacts::Return);
        let mut cleanup_kinds_present = BTreeSet::new();
        cleanup_kinds_present.insert(CleanupKindFacts::Return);
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton:
                crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonFacts {
                    kind: SkeletonKind::Loop,
                    ..Default::default()
                },
            features: LoopFeatureFacts {
                nested_loop: false,
                exit_usage: ExitUsageFacts {
                    has_break: true,
                    has_continue: true,
                    has_return: true,
                    has_unwind: false,
                },
                exit_map: Some(ExitMapFacts { kinds_present }),
                value_join: None,
                cleanup: Some(CleanupFacts {
                    kinds_present: cleanup_kinds_present,
                }),
            },
            scan_with_init: None,
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
        assert_eq!(canonical.skeleton_kind, SkeletonKind::Loop);
        assert!(!canonical.nested_loop);
        assert!(canonical.exit_usage.has_break);
        assert!(canonical.exit_usage.has_continue);
        assert!(canonical.exit_usage.has_return);
        assert_eq!(canonical.exit_kinds_present.len(), 3);
        assert!(canonical.exit_kinds_present.contains(&ExitKindFacts::Break));
        assert!(canonical
            .exit_kinds_present
            .contains(&ExitKindFacts::Continue));
        assert!(canonical
            .exit_kinds_present
            .contains(&ExitKindFacts::Return));
        assert_eq!(canonical.cleanup_kinds_present.len(), 1);
        assert!(canonical
            .cleanup_kinds_present
            .contains(&CleanupKindFacts::Return));
        assert!(!canonical.value_join_needed);
    }

    #[test]
    fn canonical_projects_empty_exit_kinds_present() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton:
                crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonFacts {
                    kind: SkeletonKind::Loop,
                    ..Default::default()
                },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
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
        assert!(canonical.exit_kinds_present.is_empty());
        assert!(canonical.cleanup_kinds_present.is_empty());
        assert!(!canonical.value_join_needed);
    }

    #[test]
    fn canonical_preserves_loop_facts_content() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features: LoopFeatureFacts::default(),
            scan_with_init: None,
            split_scan: None,
            loop_simple_while: Some(
                crate::mir::builder::control_flow::plan::facts::loop_simple_while_facts::LoopSimpleWhileFacts {
                    loop_var: "i".to_string(),
                    condition: ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(v("i")),
                        right: Box::new(lit_int(3)),
                        span: Span::unknown(),
                    },
                    loop_increment: ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(v("i")),
                        right: Box::new(lit_int(1)),
                        span: Span::unknown(),
                    },
                },
            ),
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
        assert!(canonical.facts.loop_simple_while.is_some());
    }
}
