//! Associated-input core for loop-header condition lowering.
//!
//! This owner performs CFG-shaped `!`, `&&`, and `||` descent and lowers every
//! leaf through the same borrowed expression port. It owns no source paths,
//! callable-result claims, or route selection.

use super::cond_lowering_loop_header::LoopHeaderCondResult;
use super::PlanNormalizer;
use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, LoopPlanExpressionPortV1};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::{BasicBlockId, EdgeArgs, MirType, ValueId};
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_loop_header_cond_input<'input, P>(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    port: &P,
    input: P::ExprInput<'input>,
    current_bb: BasicBlockId,
    body_bb: BasicBlockId,
    after_bb: BasicBlockId,
    body_args: EdgeArgs,
    after_args: EdgeArgs,
    error_prefix: &str,
) -> Result<LoopHeaderCondResult, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    match port.expr_syntax(&input) {
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            ..
        } => {
            let operand = port
                .child_expr(&input, ExprChildRoleV1::UnaryOperand)
                .map_err(|error| error.render())?;
            lower_loop_header_cond_input(
                builder,
                phi_bindings,
                port,
                operand,
                current_bb,
                after_bb,
                body_bb,
                after_args,
                body_args,
                error_prefix,
            )
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            ..
        } => {
            let left = port
                .child_expr(&input, ExprChildRoleV1::BinaryLeft)
                .map_err(|error| error.render())?;
            let right = port
                .child_expr(&input, ExprChildRoleV1::BinaryRight)
                .map_err(|error| error.render())?;
            let intermediate_bb = builder.next_block_id();
            let lhs_result = lower_loop_header_cond_input(
                builder,
                phi_bindings,
                port,
                left,
                current_bb,
                intermediate_bb,
                after_bb,
                empty_carriers_args(),
                after_args.clone(),
                error_prefix,
            )?;
            let rhs_result = lower_loop_header_cond_input(
                builder,
                phi_bindings,
                port,
                right,
                intermediate_bb,
                body_bb,
                after_bb,
                body_args,
                after_args,
                error_prefix,
            )?;
            Ok(merge_short_circuit(lhs_result, rhs_result))
        }
        ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            ..
        } => {
            let left = port
                .child_expr(&input, ExprChildRoleV1::BinaryLeft)
                .map_err(|error| error.render())?;
            let right = port
                .child_expr(&input, ExprChildRoleV1::BinaryRight)
                .map_err(|error| error.render())?;
            let intermediate_bb = builder.next_block_id();
            let lhs_result = lower_loop_header_cond_input(
                builder,
                phi_bindings,
                port,
                left,
                current_bb,
                body_bb,
                intermediate_bb,
                body_args.clone(),
                empty_carriers_args(),
                error_prefix,
            )?;
            let rhs_result = lower_loop_header_cond_input(
                builder,
                phi_bindings,
                port,
                right,
                intermediate_bb,
                body_bb,
                after_bb,
                body_args,
                after_args,
                error_prefix,
            )?;
            Ok(merge_short_circuit(lhs_result, rhs_result))
        }
        _ => lower_leaf(
            builder,
            phi_bindings,
            port,
            input,
            current_bb,
            body_bb,
            after_bb,
            body_args,
            after_args,
            error_prefix,
        ),
    }
}

fn merge_short_circuit(
    lhs: LoopHeaderCondResult,
    rhs: LoopHeaderCondResult,
) -> LoopHeaderCondResult {
    let mut block_effects = lhs.block_effects;
    for (block, effects) in rhs.block_effects {
        block_effects.entry(block).or_default().extend(effects);
    }
    let mut branches = lhs.branches;
    branches.extend(rhs.branches);
    LoopHeaderCondResult {
        block_effects,
        branches,
        first_cond: lhs.first_cond,
    }
}

#[allow(clippy::too_many_arguments)]
fn lower_leaf<'input, P>(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    port: &P,
    input: P::ExprInput<'input>,
    current_bb: BasicBlockId,
    body_bb: BasicBlockId,
    after_bb: BasicBlockId,
    body_args: EdgeArgs,
    after_args: EdgeArgs,
    _error_prefix: &str,
) -> Result<LoopHeaderCondResult, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let (cond_id, effects) = match port.expr_syntax(&input) {
        ASTNode::BinaryOp {
            operator:
                BinaryOperator::Less
                | BinaryOperator::LessEqual
                | BinaryOperator::Greater
                | BinaryOperator::GreaterEqual
                | BinaryOperator::Equal
                | BinaryOperator::NotEqual,
            ..
        } => {
            let (lhs, op, rhs, mut effects) =
                PlanNormalizer::lower_compare_input(port, input, builder, phi_bindings)?;
            let dst = builder.alloc_typed(MirType::Bool);
            effects.push(CoreEffectPlan::Compare { dst, lhs, op, rhs });
            (dst, effects)
        }
        _ => PlanNormalizer::lower_value_input(port, input, builder, phi_bindings)?,
    };

    debug_leaf(builder, current_bb, cond_id, &effects);
    let mut block_effects = BTreeMap::new();
    block_effects.insert(current_bb, effects);
    let branch = edgecfg_stubs::build_branch_stub(
        current_bb, cond_id, body_bb, body_args, after_bb, after_args,
    );
    Ok(LoopHeaderCondResult {
        block_effects,
        branches: vec![branch],
        first_cond: cond_id,
    })
}

fn debug_leaf(
    builder: &MirBuilder,
    current_bb: BasicBlockId,
    cond_id: ValueId,
    effects: &[CoreEffectPlan],
) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }
    let int3_dsts = effects
        .iter()
        .filter_map(|effect| match effect {
            CoreEffectPlan::Const {
                dst,
                value: crate::mir::ConstValue::Integer(3),
            } => Some(format!("%{}", dst.0)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if int3_dsts.is_empty() {
        return;
    }
    let binop_add_rhs = effects
        .iter()
        .filter_map(|effect| match effect {
            CoreEffectPlan::BinOp {
                op: crate::mir::BinaryOp::Add,
                rhs,
                ..
            } => Some(format!("%{}", rhs.0)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .map(|function| function.signature.name.as_str())
        .unwrap_or("<none>");
    crate::runtime::get_global_ring0().log.debug(&format!(
        "[loop_header/effects:leaf] fn={} bb={:?} cond=%{} effects_len={} const_int3_dsts=[{}] binop_add_rhs=[{}]",
        function,
        current_bb,
        cond_id.0,
        effects.len(),
        int3_dsts.join(","),
        binop_add_rhs.join(","),
    ));
}
