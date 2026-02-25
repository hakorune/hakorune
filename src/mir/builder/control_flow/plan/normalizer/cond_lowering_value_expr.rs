//! Value expression lowering for conditions.

use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, MirType, ValueId};
use std::collections::BTreeMap;
use super::cond_lowering_prelude::lower_cond_prelude_stmts;

pub(super) fn lower_cond_to_value_impl(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
    let (bindings, mut prelude_effects) = lower_cond_prelude_stmts(builder, phi_bindings, &cond.prelude_stmts, error_prefix)?;
    let (value_id, mut cond_effects) = loop_body_lowering::lower_bool_expr(builder, &bindings, &cond.tail_expr, error_prefix)?;
    prelude_effects.append(&mut cond_effects);
    Ok((value_id, prelude_effects))
}

pub(super) fn lower_cond_value_expr(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    expr: &ASTNode,
    error_prefix: &str,
) -> Result<(ValueId, Vec<CoreEffectPlan>), String> {
    match expr {
        ASTNode::BinaryOp { operator, .. } => match operator {
            BinaryOperator::And | BinaryOperator::Or => Err(format!("{error_prefix}: short-circuit condition must be lowered via CondBlockView")),
            BinaryOperator::Less | BinaryOperator::LessEqual | BinaryOperator::Greater | BinaryOperator::GreaterEqual | BinaryOperator::Equal | BinaryOperator::NotEqual => {
                let (lhs, op, rhs, mut consts) = PlanNormalizer::lower_compare_ast(expr, builder, phi_bindings)?;
                let dst = builder.alloc_typed(MirType::Bool);
                consts.push(CoreEffectPlan::Compare { dst, lhs, op, rhs });
                Ok((dst, consts))
            }
            _ => {
                let (value_id, effects) =
                    PlanNormalizer::lower_value_ast(expr, builder, phi_bindings)?;
                debug_log_cond_value_int3(builder, value_id, &effects);
                Ok((value_id, effects))
            }
        },
        ASTNode::UnaryOp { operator: UnaryOperator::Not, .. } => Err(format!("{error_prefix}: short-circuit condition must be lowered via CondBlockView")),
        _ => {
            let (value_id, effects) =
                PlanNormalizer::lower_value_ast(expr, builder, phi_bindings)?;
            debug_log_cond_value_int3(builder, value_id, &effects);
            Ok((value_id, effects))
        }
    }
}

fn debug_log_cond_value_int3(
    builder: &mut MirBuilder,
    value_id: ValueId,
    effects: &[CoreEffectPlan],
) {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return;
    }

    let mut int3_dsts: Vec<ValueId> = Vec::new();
    for effect in effects {
        if let CoreEffectPlan::Const { dst, value } = effect {
            if matches!(value, ConstValue::Integer(3)) {
                int3_dsts.push(*dst);
            }
        }
    }

    if int3_dsts.is_empty() {
        return;
    }

    let fn_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<none>");
    let const_int3_dsts = int3_dsts
        .iter()
        .map(|v| format!("%{}", v.0))
        .collect::<Vec<_>>()
        .join(",");
    let ring0 = crate::runtime::get_global_ring0();
    ring0.log.debug(&format!(
        "[cond_value/effects:int3] fn={} bb={:?} value=%{} effects_len={} const_int3_dsts=[{}]",
        fn_name,
        builder.current_block,
        value_id.0,
        effects.len(),
        const_int3_dsts
    ));
}
