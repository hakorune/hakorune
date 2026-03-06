//! Phase 29ap P10: CoreLoopComposer v2 (nested minimal, strict/dev only)

use crate::mir::basic_block::EdgeArgs;
use crate::mir::builder::control_flow::plan::edgecfg_facade::Frag;
use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::composer::coreloop_gates::{
    coreloop_base_gate, exit_kinds_empty,
};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreLoopPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{ConstValue, MirType};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn try_compose_core_loop_v2_nested_minimal(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    _ctx: &LoopRouteContext,
) -> Result<Option<LoweredRecipe>, String> {
    if !coreloop_base_gate(facts) {
        return Ok(None);
    }
    if facts.value_join_needed {
        return Ok(None);
    }
    if !facts.nested_loop {
        return Ok(None);
    }
    if !exit_kinds_empty(facts) {
        return Ok(None);
    }

    let Some(nested) = facts.facts.nested_loop_minimal() else {
        return Ok(None);
    };

    let outer_loop_var_init = builder
        .variable_ctx
        .variable_map
        .get(&nested.outer_loop_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "[coreloop_v2_nested] outer loop var '{}' not in variable_map",
                nested.outer_loop_var
            )
        })?;
    let acc_var_init = builder
        .variable_ctx
        .variable_map
        .get(&nested.acc_var)
        .copied()
        .ok_or_else(|| {
            format!(
                "[coreloop_v2_nested] acc var '{}' not in variable_map",
                nested.acc_var
            )
        })?;

    let preheader_bb = builder
        .current_block
        .ok_or_else(|| "[coreloop_v2_nested] No current block for loop entry".to_string())?;
    let header_bb = builder.next_block_id();
    let body_bb = builder.next_block_id();
    let inner_header_bb = builder.next_block_id();
    let inner_body_bb = builder.next_block_id();
    let inner_step_bb = builder.next_block_id();
    let inner_after_bb = builder.next_block_id();
    let step_bb = builder.next_block_id();
    let after_bb = builder.next_block_id();

    let outer_loop_var_current = builder.alloc_typed(MirType::Integer);
    let acc_outer_current = builder.alloc_typed(MirType::Integer);
    let cond_outer = builder.alloc_typed(MirType::Bool);
    let outer_loop_var_next = builder.alloc_typed(MirType::Integer);

    let inner_loop_var_current = builder.alloc_typed(MirType::Integer);
    let acc_inner_current = builder.alloc_typed(MirType::Integer);
    let cond_inner = builder.alloc_typed(MirType::Bool);
    let acc_inner_next = builder.alloc_typed(MirType::Integer);
    let inner_loop_var_next = builder.alloc_typed(MirType::Integer);

    let j_init_value = builder.alloc_typed(MirType::Integer);
    let j_init_effect = CoreEffectPlan::Const {
        dst: j_init_value,
        value: ConstValue::Integer(nested.inner_init_lit),
    };

    let mut outer_phi_bindings = BTreeMap::new();
    outer_phi_bindings.insert(nested.outer_loop_var.clone(), outer_loop_var_current);

    let mut inner_phi_bindings = BTreeMap::new();
    inner_phi_bindings.insert(nested.inner_loop_var.clone(), inner_loop_var_current);
    inner_phi_bindings.insert(nested.acc_var.clone(), acc_inner_current);

    let (outer_cond_lhs, outer_cond_op, outer_cond_rhs, mut outer_cond_consts) =
        PlanNormalizer::lower_compare_ast(&nested.outer_condition, builder, &outer_phi_bindings)?;
    outer_cond_consts.push(CoreEffectPlan::Compare {
        dst: cond_outer,
        lhs: outer_cond_lhs,
        op: outer_cond_op,
        rhs: outer_cond_rhs,
    });

    let (inner_cond_lhs, inner_cond_op, inner_cond_rhs, mut inner_cond_consts) =
        PlanNormalizer::lower_compare_ast(&nested.inner_condition, builder, &inner_phi_bindings)?;
    inner_cond_consts.push(CoreEffectPlan::Compare {
        dst: cond_inner,
        lhs: inner_cond_lhs,
        op: inner_cond_op,
        rhs: inner_cond_rhs,
    });

    let (outer_inc_lhs, outer_inc_op, outer_inc_rhs, mut outer_inc_consts) =
        PlanNormalizer::lower_binop_ast(&nested.outer_increment, builder, &outer_phi_bindings)?;
    outer_inc_consts.push(CoreEffectPlan::BinOp {
        dst: outer_loop_var_next,
        lhs: outer_inc_lhs,
        op: outer_inc_op,
        rhs: outer_inc_rhs,
    });

    let (acc_update_lhs, acc_update_op, acc_update_rhs, mut acc_update_consts) =
        PlanNormalizer::lower_binop_ast(&nested.acc_update, builder, &inner_phi_bindings)?;
    acc_update_consts.push(CoreEffectPlan::BinOp {
        dst: acc_inner_next,
        lhs: acc_update_lhs,
        op: acc_update_op,
        rhs: acc_update_rhs,
    });

    let (inner_inc_lhs, inner_inc_op, inner_inc_rhs, mut inner_inc_consts) =
        PlanNormalizer::lower_binop_ast(&nested.inner_increment, builder, &inner_phi_bindings)?;
    inner_inc_consts.push(CoreEffectPlan::BinOp {
        dst: inner_loop_var_next,
        lhs: inner_inc_lhs,
        op: inner_inc_op,
        rhs: inner_inc_rhs,
    });

    let mut inner_step_effects = acc_update_consts;
    inner_step_effects.extend(inner_inc_consts);

    let block_effects = vec![
        (preheader_bb, vec![]),
        (header_bb, outer_cond_consts),
        (body_bb, vec![]),
        (inner_header_bb, inner_cond_consts),
        (inner_body_bb, vec![]),
        (inner_step_bb, inner_step_effects),
        (inner_after_bb, vec![]),
        (step_bb, outer_inc_consts),
    ];

    let phis = vec![
        build_loop_phi_info(
            header_bb,
            preheader_bb,
            step_bb,
            outer_loop_var_current,
            outer_loop_var_init,
            outer_loop_var_next,
            format!("loop_var_{}", nested.outer_loop_var),
        ),
        build_loop_phi_info(
            header_bb,
            preheader_bb,
            step_bb,
            acc_outer_current,
            acc_var_init,
            acc_inner_current,
            format!("acc_var_{}", nested.acc_var),
        ),
        build_loop_phi_info(
            inner_header_bb,
            body_bb,
            inner_step_bb,
            inner_loop_var_current,
            j_init_value,
            inner_loop_var_next,
            format!("inner_loop_var_{}", nested.inner_loop_var),
        ),
        build_loop_phi_info(
            inner_header_bb,
            body_bb,
            inner_step_bb,
            acc_inner_current,
            acc_outer_current,
            acc_inner_next,
            format!("inner_acc_var_{}", nested.acc_var),
        ),
    ];

    let empty_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    };

    let branches = vec![
        edgecfg_stubs::build_loop_header_branch_with_args(
            header_bb,
            cond_outer,
            body_bb,
            empty_args.clone(),
            after_bb,
            empty_args.clone(),
        ),
        edgecfg_stubs::build_loop_header_branch_with_args(
            inner_header_bb,
            cond_inner,
            inner_body_bb,
            empty_args.clone(),
            inner_after_bb,
            empty_args.clone(),
        ),
    ];

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge_with_args(body_bb, inner_header_bb, empty_args.clone()),
        edgecfg_stubs::build_loop_back_edge_with_args(
            inner_body_bb,
            inner_step_bb,
            empty_args.clone(),
        ),
        edgecfg_stubs::build_loop_back_edge_with_args(
            inner_step_bb,
            inner_header_bb,
            empty_args.clone(),
        ),
        edgecfg_stubs::build_loop_back_edge_with_args(inner_after_bb, step_bb, empty_args.clone()),
        edgecfg_stubs::build_loop_back_edge_with_args(step_bb, header_bb, empty_args.clone()),
    ];

    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches,
    };

    let body_plans = vec![CorePlan::Effect(j_init_effect)];

    let final_values = vec![
        (nested.outer_loop_var.clone(), outer_loop_var_current),
        (nested.acc_var.clone(), acc_outer_current),
    ];
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
        body: body_plans,
        cond_loop: cond_outer,
        cond_match: cond_inner,
        block_effects,
        phis,
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    };

    Ok(Some(CorePlan::Loop(loop_plan)))
}

#[cfg(test)]
mod tests {
    use super::try_compose_core_loop_v2_nested_minimal;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::patterns::router::LoopRouteContext;
    use crate::mir::builder::control_flow::plan::facts::feature_facts::{
        LoopFeatureFacts, ValueJoinFacts,
    };
    use crate::mir::builder::control_flow::plan::facts::LoopFacts;
    use crate::mir::builder::control_flow::plan::facts::nested_loop_minimal_facts::
        NestedLoopMinimalFacts;
    use crate::mir::builder::control_flow::plan::facts::scan_shapes::{
        ConditionShape, StepShape,
    };
    use crate::mir::builder::control_flow::plan::facts::skeleton_facts::{
        SkeletonFacts, SkeletonKind,
    };
    use crate::mir::builder::control_flow::plan::normalize::canonicalize_loop_facts;
    use crate::mir::builder::control_flow::plan::CorePlan;
    use crate::mir::builder::MirBuilder;
    use crate::mir::MirType;

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

    fn condition_lt(loop_var: &str, bound: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(v(loop_var)),
            right: Box::new(lit_int(bound)),
            span: Span::unknown(),
        }
    }

    fn increment_value(loop_var: &str, step: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(v(loop_var)),
            right: Box::new(lit_int(step)),
            span: Span::unknown(),
        }
    }

    fn nested_facts() -> (NestedLoopMinimalFacts, ASTNode, Vec<ASTNode>) {
        let outer_condition = condition_lt("i", 3);
        let inner_condition = condition_lt("j", 3);
        let acc_update = increment_value("sum", 1);
        let outer_increment = increment_value("i", 1);
        let inner_increment = increment_value("j", 1);

        let inner_loop = ASTNode::Loop {
            condition: Box::new(inner_condition.clone()),
            body: vec![
                ASTNode::Assignment {
                    target: Box::new(v("sum")),
                    value: Box::new(acc_update.clone()),
                    span: Span::unknown(),
                },
                ASTNode::Assignment {
                    target: Box::new(v("j")),
                    value: Box::new(inner_increment.clone()),
                    span: Span::unknown(),
                },
            ],
            span: Span::unknown(),
        };

        let body = vec![
            ASTNode::Local {
                variables: vec!["j".to_string()],
                initial_values: vec![None],
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(v("j")),
                value: Box::new(lit_int(0)),
                span: Span::unknown(),
            },
            inner_loop,
            ASTNode::Assignment {
                target: Box::new(v("i")),
                value: Box::new(outer_increment.clone()),
                span: Span::unknown(),
            },
        ];

        (
            NestedLoopMinimalFacts {
                outer_loop_var: "i".to_string(),
                outer_condition: outer_condition.clone(),
                outer_increment: outer_increment.clone(),
                inner_loop_var: "j".to_string(),
                inner_condition,
                inner_increment,
                acc_var: "sum".to_string(),
                acc_update,
                inner_init_lit: 0,
            },
            outer_condition,
            body,
        )
    }

    #[test]
    fn coreloop_v2_composes_nested_minimal() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("coreloop_v2_nested".to_string());

        let i_init = builder.alloc_typed(MirType::Integer);
        let sum_init = builder.alloc_typed(MirType::Integer);
        builder
            .variable_ctx
            .variable_map
            .insert("i".to_string(), i_init);
        builder
            .variable_ctx
            .variable_map
            .insert("sum".to_string(), sum_init);

        let (nested, condition, body) = nested_facts();
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features: LoopFeatureFacts {
                nested_loop: true,
                ..LoopFeatureFacts::default()
            },
            scan_with_init: None,
            split_scan: None,
            loop_simple_while: None,
            loop_char_map: None,
            loop_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
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
            nested_loop_minimal: Some(nested),
            bool_predicate_scan: None,
            accum_const_loop: None,
            loop_break: None,
            loop_break_body_local: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let ctx = LoopRouteContext::new(&condition, &body, "coreloop_v2_nested", false, false);

        let composed = try_compose_core_loop_v2_nested_minimal(&mut builder, &canonical, &ctx)
            .expect("Ok");
        assert!(matches!(composed, Some(CorePlan::Loop(_))));
    }

    #[test]
    fn coreloop_v2_rejects_value_join() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("coreloop_v2_nested_join".to_string());

        let i_init = builder.alloc_typed(MirType::Integer);
        let sum_init = builder.alloc_typed(MirType::Integer);
        builder
            .variable_ctx
            .variable_map
            .insert("i".to_string(), i_init);
        builder
            .variable_ctx
            .variable_map
            .insert("sum".to_string(), sum_init);

        let (nested, condition, body) = nested_facts();
        let facts = LoopFacts {
            condition_shape: ConditionShape::Unknown,
            step_shape: StepShape::Unknown,
            skeleton: SkeletonFacts {
                kind: SkeletonKind::Loop,
                ..Default::default()
            },
            features: LoopFeatureFacts {
                nested_loop: true,
                value_join: Some(ValueJoinFacts { needed: true }),
                ..LoopFeatureFacts::default()
            },
            scan_with_init: None,
            split_scan: None,
            loop_simple_while: None,
            loop_char_map: None,
            loop_array_join: None,
            pattern_is_integer: None,

            pattern_starts_with: None,


            pattern_int_to_str: None,


            pattern_escape_map: None,


            pattern_split_lines: None,



            pattern_skip_ws: None,
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
            nested_loop_minimal: Some(nested),
            bool_predicate_scan: None,
            accum_const_loop: None,
            loop_break: None,
            loop_break_body_local: None,
        };
        let canonical = canonicalize_loop_facts(facts);
        let ctx = LoopRouteContext::new(&condition, &body, "coreloop_v2_nested_join", false, false);

        let composed = try_compose_core_loop_v2_nested_minimal(&mut builder, &canonical, &ctx)
            .expect("Ok");
        assert!(composed.is_none());
    }
}
