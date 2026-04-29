use super::helpers_layout::{create_phi_bindings, LoopBlocksStandard5};
use super::CoreEffectPlan;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::edgecfg::api::Frag;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::features::loop_carriers::build_loop_phi_info;
use crate::mir::builder::control_flow::plan::CoreLoopPlan;
use crate::mir::builder::control_flow::plan::step_mode::extract_to_step_bb_explicit_step;
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::EdgeArgs;
use crate::mir::MirType;
use std::collections::BTreeMap;

/// Build the canonical simple-while coreloop scaffold.
///
/// Runtime callers should import this builder module directly from `plan::normalizer`.
pub(in crate::mir::builder) fn build_simple_while_coreloop(
    builder: &mut MirBuilder,
    loop_var: &str,
    condition: &ASTNode,
    loop_increment: &ASTNode,
    _ctx: &LoopRouteContext,
) -> Result<CoreLoopPlan, String> {
    let loop_var_init = builder
        .variable_ctx
        .variable_map
        .get(loop_var)
        .copied()
        .ok_or_else(|| format!("[normalizer] Loop variable {} not found", loop_var))?;

    let blocks = LoopBlocksStandard5::allocate(builder)?;
    let LoopBlocksStandard5 {
        preheader_bb,
        header_bb,
        body_bb,
        step_bb,
        after_bb,
    } = blocks;

    let loop_var_current = builder.alloc_typed(MirType::Integer);
    let cond_loop = builder.alloc_typed(MirType::Bool);
    let loop_var_next = builder.alloc_typed(MirType::Integer);

    let phi_bindings = create_phi_bindings(&[(loop_var, loop_var_current)]);

    let (loop_cond_lhs, loop_cond_op, loop_cond_rhs, loop_cond_consts) =
        super::PlanNormalizer::lower_compare_ast(condition, builder, &phi_bindings)?;

    let (loop_inc_lhs, loop_inc_op, loop_inc_rhs, loop_inc_consts) =
        super::PlanNormalizer::lower_binop_ast(loop_increment, builder, &phi_bindings)?;

    let mut header_effects = loop_cond_consts;
    header_effects.push(CoreEffectPlan::Compare {
        dst: cond_loop,
        lhs: loop_cond_lhs,
        op: loop_cond_op,
        rhs: loop_cond_rhs,
    });

    let mut step_effects = loop_inc_consts;
    step_effects.push(CoreEffectPlan::BinOp {
        dst: loop_var_next,
        lhs: loop_inc_lhs,
        op: loop_inc_op,
        rhs: loop_inc_rhs,
    });

    let block_effects = vec![
        (preheader_bb, vec![]),
        (header_bb, header_effects),
        (body_bb, vec![]),
        (step_bb, step_effects),
    ];

    let phis = vec![build_loop_phi_info(
        header_bb,
        preheader_bb,
        step_bb,
        loop_var_current,
        loop_var_init,
        loop_var_next,
        format!("loop_var_{}", loop_var),
    )];

    let empty_args = EdgeArgs {
        layout: JumpArgsLayout::CarriersOnly,
        values: vec![],
    };

    let branches = vec![edgecfg_stubs::build_loop_header_branch_with_args(
        header_bb,
        cond_loop,
        body_bb,
        empty_args.clone(),
        after_bb,
        empty_args.clone(),
    )];

    let wires = vec![
        edgecfg_stubs::build_loop_back_edge_with_args(body_bb, step_bb, empty_args.clone()),
        edgecfg_stubs::build_loop_back_edge_with_args(step_bb, header_bb, empty_args.clone()),
    ];

    let frag = Frag {
        entry: header_bb,
        block_params: BTreeMap::new(),
        exits: BTreeMap::new(),
        wires,
        branches,
    };

    let final_values = vec![(loop_var.to_string(), loop_var_current)];
    let (step_mode, has_explicit_step) = extract_to_step_bb_explicit_step();

    Ok(CoreLoopPlan {
        preheader_bb,
        preheader_is_fresh: false,
        header_bb,
        body_bb,
        step_bb,
        continue_target: step_bb,
        after_bb,
        found_bb: after_bb,
        body: vec![],
        cond_loop,
        cond_match: cond_loop,
        block_effects,
        phis,
        frag,
        final_values,
        step_mode,
        has_explicit_step,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
    use crate::mir::builder::MirBuilder;

    fn make_condition(loop_var: &str, limit: i64) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: loop_var.to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(limit),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn make_increment(loop_var: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(ASTNode::Variable {
                name: loop_var.to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn make_body(loop_var: &str) -> Vec<ASTNode> {
        vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: loop_var.to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(make_increment(loop_var)),
            span: Span::unknown(),
        }]
    }

    #[test]
    fn build_simple_while_coreloop_has_expected_frag_shape() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("simple_while_coreloop_test".to_string());

        let loop_var = "i";
        let loop_var_init = builder.alloc_typed(MirType::Integer);
        builder
            .variable_ctx
            .variable_map
            .insert(loop_var.to_string(), loop_var_init);

        let condition = make_condition(loop_var, 3);
        let loop_increment = make_increment(loop_var);
        let body = make_body(loop_var);
        let ctx = LoopRouteContext::new(
            &condition,
            &body,
            "simple_while_coreloop_test",
            false,
            false,
        );

        let loop_plan =
            build_simple_while_coreloop(&mut builder, loop_var, &condition, &loop_increment, &ctx)
                .expect("simple_while coreloop build should succeed");

        assert_eq!(loop_plan.phis.len(), 1);
        assert_eq!(loop_plan.frag.branches.len(), 1);
        assert_eq!(loop_plan.frag.wires.len(), 2);
        assert!(loop_plan.frag.exits.is_empty());

        builder.exit_function_for_test();
    }
}
