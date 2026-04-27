//! ExitMap feature helpers (break/continue blocks → CorePlan exits).

use super::exit_branch;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_if_exit_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    else_exit_override: Option<CoreExitPlan>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let cond_view = CondBlockView::from_expr(condition);
    let has_else = else_body.is_some() || else_exit_override.is_some();
    let else_exit_override = else_exit_override.clone();
    let mut then_carrier_updates = BTreeMap::new();
    let mut else_carrier_updates = BTreeMap::new();

    let mut lower_then =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            lower_exit_block(
                builder,
                bindings,
                carrier_phis,
                &mut then_carrier_updates,
                then_body,
                error_prefix,
            )
        };

    let mut lower_else_closure =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            match else_body {
                Some(body) => lower_exit_block(
                    builder,
                    bindings,
                    carrier_phis,
                    &mut else_carrier_updates,
                    body,
                    error_prefix,
                ),
                None => Ok(else_exit_override
                    .as_ref()
                    .map(|exit| vec![CorePlan::Exit(exit.clone())])
                    .unwrap_or_default()),
            }
        };

    let mut current_bindings = phi_bindings.clone();
    let plans = parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        &mut current_bindings,
        &cond_view,
        error_prefix,
        &mut lower_then,
        has_else.then_some(
            &mut lower_else_closure
                as &mut dyn FnMut(
                    &mut MirBuilder,
                    &mut BTreeMap<String, crate::mir::ValueId>,
                ) -> Result<Vec<LoweredRecipe>, String>,
        ),
        &|_name, _bindings| false,
    )?;

    carrier_updates.extend(then_carrier_updates);
    carrier_updates.extend(else_carrier_updates);

    Ok(plans)
}

pub(in crate::mir::builder) fn lower_exit_block(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if body.is_empty() {
        return Err(format!("{error_prefix}: empty exit block"));
    }

    let mut out = Vec::new();
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if is_last {
                    return Err(format!("{error_prefix}: exit missing at tail"));
                }
                let effects = lower_assignment_stmt(
                    builder,
                    phi_bindings,
                    carrier_phis,
                    carrier_updates,
                    target,
                    value,
                    error_prefix,
                )?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if is_last {
                    return Err(format!("{error_prefix}: exit missing at tail"));
                }
                let effects = lower_local_init_stmt(
                    builder,
                    phi_bindings,
                    variables,
                    initial_values,
                    error_prefix,
                )?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::MethodCall { .. } => {
                if is_last {
                    return Err(format!("{error_prefix}: exit missing at tail"));
                }
                let effects = loop_body_lowering::lower_method_call_stmt(
                    builder,
                    phi_bindings,
                    stmt,
                    error_prefix,
                )?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::FunctionCall { .. } => {
                if is_last {
                    return Err(format!("{error_prefix}: exit missing at tail"));
                }
                let effects = loop_body_lowering::lower_function_call_stmt(
                    builder,
                    phi_bindings,
                    stmt,
                    error_prefix,
                )?;
                out.extend(effects_to_plans(effects));
            }
            ASTNode::Break { .. } => {
                if !is_last {
                    return Err(format!("{error_prefix}: break must be at tail"));
                }
                out.push(exit_branch::build_break_only(1));
            }
            ASTNode::Continue { .. } => {
                if !is_last {
                    return Err(format!("{error_prefix}: continue must be at tail"));
                }
                out.push(exit_branch::build_continue_only(1));
            }
            ASTNode::Return { value, .. } => {
                if !is_last {
                    return Err(format!("{error_prefix}: return must be at tail"));
                }
                let value_ast = value
                    .as_ref()
                    .ok_or_else(|| format!("{error_prefix}: return without value"))?;
                out.extend(parts::entry::lower_return_with_effects(
                    builder,
                    Some(value_ast),
                    phi_bindings,
                    error_prefix,
                )?);
            }
            _ => {
                return Err(format!("{error_prefix}: unsupported stmt in exit block"));
            }
        }
    }
    Ok(out)
}

pub(in crate::mir::builder) fn lower_assignment_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    target: &ASTNode,
    value: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String> {
    let (name, value_id, effects) = loop_body_lowering::lower_assignment_value(
        builder,
        phi_bindings,
        target,
        value,
        error_prefix,
    )?;
    if carrier_phis.contains_key(&name) {
        carrier_updates.insert(name.clone(), value_id);
    }
    builder.variable_ctx.variable_map.insert(name, value_id);
    Ok(effects)
}

pub(in crate::mir::builder) fn lower_local_init_stmt(
    builder: &mut MirBuilder,
    phi_bindings: &BTreeMap<String, crate::mir::ValueId>,
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String> {
    let (inits, effects) = loop_body_lowering::lower_local_init_values(
        builder,
        phi_bindings,
        variables,
        initial_values,
        error_prefix,
    )?;
    for (name, value_id) in inits {
        builder.variable_ctx.variable_map.insert(name, value_id);
    }
    Ok(effects)
}
