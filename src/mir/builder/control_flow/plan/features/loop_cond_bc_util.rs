//! Utility functions for item/statement lowering.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::carrier_merge::{
    lower_assignment_stmt, lower_local_init_stmt,
};
use crate::mir::builder::control_flow::plan::normalizer::{loop_body_lowering, PlanNormalizer};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{CoreEffectPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::{Effect, EffectMask};
use std::collections::BTreeMap;

use super::loop_cond_bc::LOOP_COND_ERR;
use super::loop_cond_bc_item_stmt::lower_loop_cond_stmt;

pub(super) fn get_stmt<'a>(body: &'a RecipeBody, stmt_ref: StmtRef) -> Result<&'a ASTNode, String> {
    body.get_ref(stmt_ref)
        .ok_or_else(|| format!("{LOOP_COND_ERR}: missing stmt idx={}", stmt_ref.index()))
}

pub(super) fn lower_simple_effect_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    match stmt {
        ASTNode::Assignment { target, value, .. } => {
            let effects = lower_assignment_stmt(
                builder,
                current_bindings,
                carrier_phis,
                carrier_updates,
                target,
                value,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            let effects = lower_local_init_stmt(
                builder,
                current_bindings,
                variables,
                initial_values,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::MethodCall { .. } => {
            let effects = loop_body_lowering::lower_method_call_stmt(
                builder,
                current_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::FunctionCall { .. } => {
            let effects = loop_body_lowering::lower_function_call_stmt(
                builder,
                current_bindings,
                stmt,
                error_prefix,
            )?;
            Ok(Some(effects_to_plans(effects)))
        }
        ASTNode::Print { expression, .. } => {
            let (value_id, mut effects) =
                PlanNormalizer::lower_value_ast(expression, builder, current_bindings)?;
            effects.push(CoreEffectPlan::ExternCall {
                dst: None,
                iface_name: "env.console".to_string(),
                method_name: "log".to_string(),
                args: vec![value_id],
                effects: EffectMask::PURE.add(Effect::Io),
            });
            Ok(Some(effects_to_plans(effects)))
        }
        _ => Ok(None),
    }
}

pub(super) fn lower_stmt_list_no_exit(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmts: &[ASTNode],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    use crate::mir::builder::control_flow::plan::CoreEffectPlan;

    let mut block_plans = Vec::new();
    for (idx, stmt) in stmts.iter().enumerate() {
        let is_last = idx + 1 == stmts.len();
        let mut plans = lower_loop_cond_stmt(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            is_last,
            stmt,
        )?;
        if plans.iter().any(|plan| {
            matches!(plan, CorePlan::Exit(_))
                || matches!(plan, CorePlan::Effect(CoreEffectPlan::ExitIf { .. }))
        }) {
            return Err(format!("{error_prefix}: block contains exit"));
        }
        block_plans.append(&mut plans);
    }
    Ok(block_plans)
}

pub(super) fn lower_stmt_list_no_direct_exit(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    stmts: &[ASTNode],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut block_plans = Vec::new();
    for (idx, stmt) in stmts.iter().enumerate() {
        let is_last = idx + 1 == stmts.len();
        let mut plans = lower_loop_cond_stmt(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            break_phi_dsts,
            carrier_updates,
            is_last,
            stmt,
        )?;
        if plans.iter().any(|plan| matches!(plan, CorePlan::Exit(_))) {
            return Err(format!("{error_prefix}: block contains direct exit"));
        }
        block_plans.append(&mut plans);
    }
    Ok(block_plans)
}
