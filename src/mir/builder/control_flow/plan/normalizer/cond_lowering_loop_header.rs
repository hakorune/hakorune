//! Loop header short-circuit condition lowering.

use crate::ast::{ASTNode, BinaryOperator, UnaryOperator};
use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
use crate::mir::builder::control_flow::plan::edgecfg_facade::BranchStub;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::features::edgecfg_stubs;
use crate::mir::builder::control_flow::plan::steps::empty_carriers_args;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use crate::mir::{ConstValue, ValueId};
use std::collections::{BTreeMap, BTreeSet};
use super::cond_lowering_prelude::lower_cond_prelude_stmts;
use super::cond_lowering_value_expr::lower_cond_value_expr;

#[derive(Debug)]
pub struct LoopHeaderCondResult {
    pub block_effects: BTreeMap<BasicBlockId, Vec<CoreEffectPlan>>,
    pub branches: Vec<BranchStub>,
    pub first_cond: ValueId,
}

impl LoopHeaderCondResult {
    pub fn preds_to(&self, target: BasicBlockId) -> BTreeSet<BasicBlockId> {
        let mut preds = BTreeSet::new();
        for br in &self.branches {
            if br.then_target == target || br.else_target == target {
                preds.insert(br.from);
            }
        }
        preds
    }
}

pub fn lower_loop_header_cond(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    cond: &CondBlockView,
    current_bb: BasicBlockId,
    body_bb: BasicBlockId,
    after_bb: BasicBlockId,
    body_args: EdgeArgs,
    after_args: EdgeArgs,
    error_prefix: &str,
) -> Result<LoopHeaderCondResult, String> {
    let (bindings, prelude_effects) = lower_cond_prelude_stmts(builder, phi_bindings, &cond.prelude_stmts, error_prefix)?;
    let mut result = lower_loop_header_cond_expr(builder, &bindings, &cond.tail_expr, current_bb, body_bb, after_bb, body_args, after_args, error_prefix)?;

    if !prelude_effects.is_empty() {
        let entry = result.block_effects.entry(current_bb).or_default();
        let mut merged = prelude_effects;
        merged.append(entry);
        *entry = merged;
    }

    Ok(result)
}

fn lower_loop_header_cond_expr(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, ValueId>,
    expr: &ASTNode,
    current_bb: BasicBlockId,
    body_bb: BasicBlockId,
    after_bb: BasicBlockId,
    body_args: EdgeArgs,
    after_args: EdgeArgs,
    error_prefix: &str,
) -> Result<LoopHeaderCondResult, String> {
    match expr {
        ASTNode::UnaryOp { operator: UnaryOperator::Not, operand, .. } => {
            lower_loop_header_cond_expr(builder, phi_bindings, operand, current_bb, after_bb, body_bb, after_args, body_args, error_prefix)
        }
        ASTNode::BinaryOp { operator: BinaryOperator::And, left, right, .. } => {
            let intermediate_bb = builder.next_block_id();
            let lhs_result = lower_loop_header_cond_expr(builder, phi_bindings, left, current_bb, intermediate_bb, after_bb, empty_carriers_args(), after_args.clone(), error_prefix)?;
            let rhs_result = lower_loop_header_cond_expr(builder, phi_bindings, right, intermediate_bb, body_bb, after_bb, body_args, after_args, error_prefix)?;

            let mut block_effects = lhs_result.block_effects;
            for (bb, effects) in rhs_result.block_effects {
                block_effects.entry(bb).or_default().extend(effects);
            }

            let mut branches = lhs_result.branches;
            branches.extend(rhs_result.branches);

            Ok(LoopHeaderCondResult { block_effects, branches, first_cond: lhs_result.first_cond })
        }
        ASTNode::BinaryOp { operator: BinaryOperator::Or, left, right, .. } => {
            let intermediate_bb = builder.next_block_id();
            let lhs_result = lower_loop_header_cond_expr(builder, phi_bindings, left, current_bb, body_bb, intermediate_bb, body_args.clone(), empty_carriers_args(), error_prefix)?;
            let rhs_result = lower_loop_header_cond_expr(builder, phi_bindings, right, intermediate_bb, body_bb, after_bb, body_args, after_args, error_prefix)?;

            let mut block_effects = lhs_result.block_effects;
            for (bb, effects) in rhs_result.block_effects {
                block_effects.entry(bb).or_default().extend(effects);
            }

            let mut branches = lhs_result.branches;
            branches.extend(rhs_result.branches);

            Ok(LoopHeaderCondResult { block_effects, branches, first_cond: lhs_result.first_cond })
        }
        _ => {
            let (cond_id, effects) = lower_cond_value_expr(builder, phi_bindings, expr, error_prefix)?;
            if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                let mut int3_dsts = Vec::new();
                let mut binop_add_rhs = Vec::new();
                for effect in &effects {
                    match effect {
                        CoreEffectPlan::Const { dst, value } => {
                            if matches!(value, ConstValue::Integer(3)) {
                                int3_dsts.push(*dst);
                            }
                        }
                        CoreEffectPlan::BinOp { op, rhs, .. } => {
                            if *op == crate::mir::BinaryOp::Add {
                                binop_add_rhs.push(*rhs);
                            }
                        }
                        _ => {}
                    }
                }
                if !int3_dsts.is_empty() {
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
                    let binop_add_rhs = binop_add_rhs
                        .iter()
                        .map(|v| format!("%{}", v.0))
                        .collect::<Vec<_>>()
                        .join(",");
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[loop_header/effects:leaf] fn={} bb={:?} cond=%{} effects_len={} const_int3_dsts=[{}] binop_add_rhs=[{}]",
                        fn_name,
                        current_bb,
                        cond_id.0,
                        effects.len(),
                        const_int3_dsts,
                        binop_add_rhs
                    ));
                }
            }
            let mut block_effects = BTreeMap::new();
            block_effects.insert(current_bb, effects);

            let branch = edgecfg_stubs::build_branch_stub(
                current_bb,
                cond_id,
                body_bb,
                body_args,
                after_bb,
                after_args,
            );

            Ok(LoopHeaderCondResult { block_effects, branches: vec![branch], first_cond: cond_id })
        }
    }
}
