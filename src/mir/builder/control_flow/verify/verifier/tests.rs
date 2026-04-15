#[cfg(test)]
mod tests {
    use super::super::core::PlanVerifier;
    use super::super::primitives::debug_assert_value_join_invariants;
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::facts::feature_facts::{
        LoopFeatureFacts, ValueJoinFacts,
    };
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::facts::scan_shapes::{ConditionShape, StepShape};
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::facts::skeleton_facts::{SkeletonFacts, SkeletonKind};
    #[cfg(debug_assertions)]
    use crate::mir::builder::control_flow::facts::LoopFacts;
    use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::step_mode::{
        extract_to_step_bb_explicit_step, inline_in_body_explicit_step,
    };
    use crate::mir::builder::control_flow::lower::{
        CoreBranchArmPlan, CoreBranchNPlan, CoreEffectPlan, CoreExitPlan, CoreIfJoin, CoreIfPlan,
        CoreLoopPlan, CorePlan, Frag,
    };
    use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
    use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::EdgeArgs;
    use crate::mir::{BasicBlockId, ConstValue, ValueId};
    use std::collections::BTreeMap;

    fn make_loop_plan(body: Vec<CorePlan>) -> CoreLoopPlan {
        let preheader_bb = BasicBlockId(0);
        let header_bb = BasicBlockId(1);
        let body_bb = BasicBlockId(2);
        let step_bb = BasicBlockId(3);
        let after_bb = BasicBlockId(4);
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb,
            continue_target: step_bb,
            after_bb,
            found_bb: after_bb,
            body,
            cond_loop: ValueId(100),
            cond_match: ValueId(101),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                (body_bb, vec![]),
                (step_bb, vec![]),
            ],
            phis: vec![build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                ValueId(102),
                ValueId(103),
                ValueId(104),
                "test_phi".to_string(),
            )],
            frag: Frag::new(header_bb),
            final_values: vec![("i".to_string(), ValueId(102))],
            step_mode,
            has_explicit_step,
        }
    }

    #[test]
    fn test_verify_empty_seq_ok() {
        let plan = CorePlan::Seq(vec![]);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_break_outside_loop_fails() {
        let plan = CorePlan::Exit(CoreExitPlan::Break(1));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V3]"));
    }

    #[test]
    fn test_verify_const_effect_succeeds() {
        let plan = CorePlan::Effect(CoreEffectPlan::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(42),
        });
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_seq_effects_ok() {
        let body = vec![CorePlan::Seq(vec![
            CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(10),
                value: ConstValue::Integer(1),
            }),
            CorePlan::Seq(vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(11),
                value: ConstValue::Integer(2),
            })]),
        ])];
        let plan = CorePlan::Loop(make_loop_plan(body));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_if_effect_ok() {
        let if_effect = CoreEffectPlan::IfEffect {
            cond: ValueId(1),
            then_effects: vec![CoreEffectPlan::Const {
                dst: ValueId(2),
                value: ConstValue::Integer(1),
            }],
            else_effects: None,
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Effect(if_effect)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_if_effect_exit_fails() {
        let if_effect = CoreEffectPlan::IfEffect {
            cond: ValueId(1),
            then_effects: vec![CoreEffectPlan::ExitIf {
                cond: ValueId(2),
                exit: CoreExitPlan::Return(Some(ValueId(3))),
            }],
            else_effects: None,
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Effect(if_effect)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V12]"));
    }

    #[test]
    fn test_verify_loop_body_if_effect_continue_ok() {
        let if_effect = CoreEffectPlan::IfEffect {
            cond: ValueId(1),
            then_effects: vec![CoreEffectPlan::ExitIf {
                cond: ValueId(1),
                exit: CoreExitPlan::Continue(1),
            }],
            else_effects: None,
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Effect(if_effect)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_loop_body_if_fails() {
        let if_plan = CoreIfPlan {
            condition: ValueId(1),
            then_plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(2),
                value: ConstValue::Integer(1),
            })],
            else_plans: None,
            joins: Vec::new(),
        };
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::If(if_plan)]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V12]"));
    }

    #[test]
    fn test_verify_loop_body_exit_fails() {
        let plan = CorePlan::Loop(make_loop_plan(vec![CorePlan::Exit(CoreExitPlan::Return(
            None,
        ))]));
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V12]"));
    }

    #[test]
    fn test_verify_if_empty_else_fails() {
        let if_plan = CoreIfPlan {
            condition: ValueId(1),
            then_plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(2),
                value: ConstValue::Integer(1),
            })],
            else_plans: Some(vec![]),
            joins: Vec::new(),
        };
        let plan = CorePlan::If(if_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V5]"));
    }

    #[test]
    fn test_verify_if_then_empty_with_joins_ok() {
        let if_plan = CoreIfPlan {
            condition: ValueId(1),
            then_plans: Vec::new(),
            else_plans: Some(Vec::new()),
            joins: vec![CoreIfJoin {
                name: "join_only".to_string(),
                dst: ValueId(10),
                pre_val: None,
                then_val: ValueId(11),
                else_val: ValueId(12),
            }],
        };
        let plan = CorePlan::If(if_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_exit_not_last_fails() {
        let plan = CorePlan::Seq(vec![
            CorePlan::Exit(CoreExitPlan::Return(None)),
            CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(1),
                value: ConstValue::Integer(0),
            }),
        ]);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V11]"));
    }

    #[test]
    fn test_verify_if_exit_not_last_fails() {
        let if_plan = CoreIfPlan {
            condition: ValueId(1),
            then_plans: vec![
                CorePlan::Exit(CoreExitPlan::Return(None)),
                CorePlan::Effect(CoreEffectPlan::Const {
                    dst: ValueId(2),
                    value: ConstValue::Integer(1),
                }),
            ],
            else_plans: Some(vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(3),
                value: ConstValue::Integer(2),
            })]),
            joins: Vec::new(),
        };
        let plan = CorePlan::If(if_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V11]"));
    }

    #[test]
    fn test_verify_branchn_ok() {
        let branch_plan = CoreBranchNPlan {
            arms: vec![
                CoreBranchArmPlan {
                    condition: ValueId(1),
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: ValueId(10),
                        value: ConstValue::Integer(1),
                    })],
                },
                CoreBranchArmPlan {
                    condition: ValueId(2),
                    plans: vec![CorePlan::Exit(CoreExitPlan::Return(None))],
                },
            ],
            else_plans: Some(vec![CorePlan::Effect(CoreEffectPlan::Const {
                dst: ValueId(11),
                value: ConstValue::Integer(2),
            })]),
        };
        let plan = CorePlan::BranchN(branch_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_branchn_single_arm_fails() {
        let branch_plan = CoreBranchNPlan {
            arms: vec![CoreBranchArmPlan {
                condition: ValueId(1),
                plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                    dst: ValueId(10),
                    value: ConstValue::Integer(1),
                })],
            }],
            else_plans: None,
        };
        let plan = CorePlan::BranchN(branch_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V5]"));
    }

    #[test]
    fn test_verify_branchn_exit_not_last_fails() {
        let branch_plan = CoreBranchNPlan {
            arms: vec![
                CoreBranchArmPlan {
                    condition: ValueId(1),
                    plans: vec![
                        CorePlan::Exit(CoreExitPlan::Return(None)),
                        CorePlan::Effect(CoreEffectPlan::Const {
                            dst: ValueId(10),
                            value: ConstValue::Integer(1),
                        }),
                    ],
                },
                CoreBranchArmPlan {
                    condition: ValueId(2),
                    plans: vec![CorePlan::Effect(CoreEffectPlan::Const {
                        dst: ValueId(11),
                        value: ConstValue::Integer(2),
                    })],
                },
            ],
            else_plans: None,
        };
        let plan = CorePlan::BranchN(branch_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V11]"));
    }

    #[test]
    fn test_verify_expr_result_plus_carriers_requires_value() {
        let mut loop_plan = make_loop_plan(vec![]);
        loop_plan.frag.wires = vec![edgecfg_stubs::build_loop_back_edge_with_args(
            loop_plan.body_bb,
            loop_plan.step_bb,
            EdgeArgs {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                values: vec![],
            },
        )];
        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V13]"));
    }

    #[test]
    fn test_verify_expr_result_plus_carriers_with_value_ok() {
        let mut loop_plan = make_loop_plan(vec![]);
        loop_plan.frag.wires = vec![edgecfg_stubs::build_loop_back_edge_with_args(
            loop_plan.body_bb,
            loop_plan.step_bb,
            EdgeArgs {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                values: vec![ValueId(200)],
            },
        )];
        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok());
    }

    #[cfg(debug_assertions)]
    #[test]
    fn debug_value_join_invariant_allows_empty_when_not_needed() {
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
        debug_assert_value_join_invariants(&canonical);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_value_join_invariant_panics_without_exit_kinds() {
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features: LoopFeatureFacts {
                exit_usage: Default::default(),
                exit_map: None,
                value_join: Some(ValueJoinFacts { needed: true }),
                cleanup: None,
                nested_loop: false,
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
        debug_assert_value_join_invariants(&canonical);
    }

    #[test]
    fn test_v10_body_bb_effects_in_block_effects_fails() {
        // V10: body_bb effects must be empty in block_effects
        // This test verifies that having effects in body_bb's block_effects fails validation
        let preheader_bb = BasicBlockId(0);
        let header_bb = BasicBlockId(1);
        let body_bb = BasicBlockId(2);
        let step_bb = BasicBlockId(3);
        let after_bb = BasicBlockId(4);
        let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

        let loop_plan = CoreLoopPlan {
            preheader_bb,
            preheader_is_fresh: false,
            header_bb,
            body_bb,
            step_bb,
            continue_target: step_bb,
            after_bb,
            found_bb: after_bb,
            body: vec![],
            cond_loop: ValueId(100),
            cond_match: ValueId(101),
            block_effects: vec![
                (preheader_bb, vec![]),
                (header_bb, vec![]),
                // V10 violation: body_bb has effects in block_effects
                (
                    body_bb,
                    vec![CoreEffectPlan::Const {
                        dst: ValueId(102),
                        value: ConstValue::Integer(42),
                    }],
                ),
                (step_bb, vec![]),
            ],
            phis: vec![build_loop_phi_info(
                header_bb,
                preheader_bb,
                step_bb,
                ValueId(103),
                ValueId(104),
                ValueId(105),
                "test_phi".to_string(),
            )],
            frag: Frag {
                entry: header_bb,
                block_params: BTreeMap::new(),
                exits: BTreeMap::new(),
                wires: vec![],
                branches: vec![],
            },
            final_values: vec![("i".to_string(), ValueId(103))],
            step_mode,
            has_explicit_step,
        };

        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("[V10]"), "Expected V10 error, got: {}", err);
        assert!(
            err.contains("body_bb"),
            "Expected body_bb in error, got: {}",
            err
        );
    }

    #[test]
    fn test_v10c_inline_explicit_step_forbids_continue_depth1() {
        let mut loop_plan = make_loop_plan(vec![CorePlan::Exit(CoreExitPlan::Continue(1))]);
        let (step_mode, has_explicit_step) = inline_in_body_explicit_step();
        loop_plan.step_mode = step_mode;
        loop_plan.has_explicit_step = has_explicit_step;
        loop_plan.frag.wires = vec![edgecfg_stubs::build_loop_back_edge_with_args(
            loop_plan.body_bb,
            loop_plan.continue_target,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        )];
        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V10c]"));
    }

    #[test]
    fn test_v10d_inline_explicit_step_requires_single_normal_backedge() {
        let mut loop_plan = make_loop_plan(vec![]);
        let (step_mode, has_explicit_step) = inline_in_body_explicit_step();
        loop_plan.step_mode = step_mode;
        loop_plan.has_explicit_step = has_explicit_step;
        loop_plan.frag.wires = vec![];
        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("[V10d]"));
    }

    #[test]
    fn test_v10d_inline_explicit_step_single_normal_backedge_ok() {
        let mut loop_plan = make_loop_plan(vec![]);
        let (step_mode, has_explicit_step) = inline_in_body_explicit_step();
        loop_plan.step_mode = step_mode;
        loop_plan.has_explicit_step = has_explicit_step;
        loop_plan.frag.wires = vec![edgecfg_stubs::build_loop_back_edge_with_args(
            loop_plan.body_bb,
            loop_plan.continue_target,
            EdgeArgs {
                layout: JumpArgsLayout::CarriersOnly,
                values: vec![],
            },
        )];
        let plan = CorePlan::Loop(loop_plan);
        let result = PlanVerifier::verify(&plan);
        assert!(result.is_ok(), "Expected pass, got: {:?}", result);
    }
}
