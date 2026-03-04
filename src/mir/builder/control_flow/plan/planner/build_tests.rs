//! Tests for build.rs planner functionality
//!
//! Extracted from build.rs for better maintainability

#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use super::super::build::*;
    use super::super::context::PlannerContext;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::facts::feature_facts::{
        ExitKindFacts, ExitMapFacts, ExitUsageFacts, LoopFeatureFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::loop_types::{
        LoopFacts, ScanWithInitFacts, SplitScanFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::pattern1_array_join_facts::
        Pattern1ArrayJoinFacts;
    use crate::mir::builder::control_flow::plan::facts::pattern1_char_map_facts::
        Pattern1CharMapFacts;
    use crate::mir::builder::control_flow::plan::facts::pattern1_simplewhile_facts::
        Pattern1SimpleWhileFacts;
    use crate::mir::builder::control_flow::plan::facts::pattern8_bool_predicate_scan_facts::
        Pattern8BoolPredicateScanFacts;
    use crate::mir::builder::control_flow::plan::facts::pattern9_accum_const_loop_facts::
        Pattern9AccumConstLoopFacts;
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
        cond_profile_from_scan_shapes, ConditionShape, StepShape,
    };
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
        SkeletonFacts, SkeletonKind,
    };
    use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::DomainPlan;
    use std::collections::{BTreeMap, BTreeSet};

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

    fn feature_facts_with_usage(exit_usage: ExitUsageFacts) -> LoopFeatureFacts {
        let mut kinds_present = BTreeSet::new();
        if exit_usage.has_return {
            kinds_present.insert(ExitKindFacts::Return);
        }
        if exit_usage.has_break {
            kinds_present.insert(ExitKindFacts::Break);
        }
        if exit_usage.has_continue {
            kinds_present.insert(ExitKindFacts::Continue);
        }
        if exit_usage.has_unwind {
            kinds_present.insert(ExitKindFacts::Unwind);
        }
        let exit_map = if kinds_present.is_empty() {
            None
        } else {
            Some(ExitMapFacts { kinds_present })
        };
        LoopFeatureFacts {
            exit_usage,
            exit_map,
            value_join: None,
            cleanup: None,
            nested_loop: false,
        }
    }

    fn base_loop_facts() -> LoopFacts {
        LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features: LoopFeatureFacts::default(),
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
            pattern2_break: None,
            pattern2_loopbodylocal: None,
            pattern6_nested_minimal: None,
        }
    }

    fn base_loop_facts_with_skeleton(kind: SkeletonKind) -> LoopFacts {
        LoopFacts {
            skeleton: SkeletonFacts {
                kind,
                ..Default::default()
            },
            ..base_loop_facts()
        }
    }

    fn base_loop_facts_with_features(features: LoopFeatureFacts) -> LoopFacts {
        LoopFacts {
            features,
            ..base_loop_facts()
        }
    }

    fn plan_from_facts(facts: LoopFacts) -> Option<DomainPlan> {
        let canonical = canonicalize_loop_facts(facts);
        build_plan_from_facts(canonical).expect("Ok")
    }

    fn scan_with_init(step_lit: i64) -> ScanWithInitFacts {
        ScanWithInitFacts {
            loop_var: "i".to_string(),
            haystack: "s".to_string(),
            needle: "ch".to_string(),
            step_lit,
            dynamic_needle: false,
        }
    }

    #[test]
    fn planner_skips_split_scan_domain_plan() {
        let facts = LoopFacts {
            split_scan: Some(SplitScanFacts {
                s_var: "s".to_string(),
                sep_var: "separator".to_string(),
                result_var: "result".to_string(),
                i_var: "i".to_string(),
                start_var: "start".to_string(),
                shape: crate::mir::builder::control_flow::plan::facts::scan_shapes::SplitScanShape::Minimal,
            }),
            ..base_loop_facts()
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_prefers_none_when_no_candidates() {
        let plan = plan_from_facts(base_loop_facts());
        assert!(plan.is_none());
    }

    #[test]
    fn planner_skips_scan_with_init_domain_plan() {
        let facts = LoopFacts {
            scan_with_init: Some(scan_with_init(1)),
            ..base_loop_facts()
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_ignores_scan_with_init_negative_step() {
        let facts = LoopFacts {
            scan_with_init: Some(scan_with_init(-1)),
            ..base_loop_facts()
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_ignores_scan_with_init_feature_staging() {
        let facts = LoopFacts {
            scan_with_init: Some(scan_with_init(1)),
            ..base_loop_facts_with_features(feature_facts_with_usage(ExitUsageFacts {
                has_break: true,
                has_continue: false,
                has_return: false,
                has_unwind: false,
            }))
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_gates_non_loop_skeletons() {
        let facts = LoopFacts {
            scan_with_init: Some(scan_with_init(1)),
            ..base_loop_facts_with_skeleton(SkeletonKind::If2)
        };

        let canonical = canonicalize_loop_facts(facts);
        let plan = build_plan_from_facts_ctx(&PlannerContext::default_for_legacy(), canonical)
            .expect("Ok");
        assert!(plan.is_none());
    }

    #[test]
    fn planner_does_not_build_pattern1_simplewhile_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
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
            pattern1_simplewhile: Some(Pattern1SimpleWhileFacts {
                loop_var: "i".to_string(),
                condition: loop_condition,
                loop_increment,
            }),
            ..base_loop_facts()
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_does_not_build_pattern1_char_map_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
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
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            pattern1_char_map: Some(Pattern1CharMapFacts {
                loop_var: "i".to_string(),
                condition: loop_condition,
                loop_increment,
                haystack_var: "s".to_string(),
                result_var: "result".to_string(),
                receiver_var: "me".to_string(),
                transform_method: "char_to_lower".to_string(),
                cond_profile: cond_profile_from_scan_shapes(
                    &ConditionShape::Unknown,
                    &StepShape::Unknown,
                ),
            }),
            ..base_loop_facts()
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_does_not_build_pattern1_array_join_plan_from_facts() {
        let loop_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(v("arr")),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let if_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Greater,
            left: Box::new(v("i")),
            right: Box::new(lit_int(0)),
            span: Span::unknown(),
        };
        let loop_increment = ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v("i")),
            right: Box::new(lit_int(1)),
            span: Span::unknown(),
        };

        let facts = LoopFacts {
            pattern1_array_join: Some(Pattern1ArrayJoinFacts {
                loop_var: "i".to_string(),
                condition: loop_condition,
                if_condition,
                loop_increment,
                array_var: "arr".to_string(),
                result_var: "result".to_string(),
                separator_var: "sep".to_string(),
                cond_profile: cond_profile_from_scan_shapes(
                    &ConditionShape::Unknown,
                    &StepShape::Unknown,
                ),
            }),
            ..base_loop_facts()
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_does_not_build_pattern8_bool_predicate_scan_plan_from_facts() {
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

        let facts = LoopFacts {
            pattern8_bool_predicate_scan: Some(Pattern8BoolPredicateScanFacts {
                loop_var: "i".to_string(),
                haystack: "s".to_string(),
                predicate_receiver: "me".to_string(),
                predicate_method: "is_digit".to_string(),
                condition,
                step_lit: 1,
                cond_profile: cond_profile_from_scan_shapes(
                    &ConditionShape::Unknown,
                    &StepShape::Unknown,
                ),
            }),
            ..base_loop_facts_with_features(feature_facts_with_usage(ExitUsageFacts {
                has_break: true,
                has_continue: false,
                has_return: false,
                has_unwind: false,
            }))
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }

    #[test]
    fn planner_does_not_build_pattern9_accum_const_loop_plan_from_facts() {
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v("i")),
            right: Box::new(lit_int(3)),
            span: Span::unknown(),
        };
        let acc_update = ASTNode::BinaryOp {
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

        let facts = LoopFacts {
            pattern9_accum_const_loop: Some(Pattern9AccumConstLoopFacts {
                loop_var: "i".to_string(),
                acc_var: "sum".to_string(),
                condition,
                acc_update,
                loop_increment,
                cond_profile: cond_profile_from_scan_shapes(
                    &ConditionShape::Unknown,
                    &StepShape::Unknown,
                ),
            }),
            ..base_loop_facts()
        };

        let plan = plan_from_facts(facts);
        assert!(plan.is_none());
    }
}
