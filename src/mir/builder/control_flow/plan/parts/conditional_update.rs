//! Conditional update + join feature (Parts SSOT, loop-cond bodies).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::common::negate_bool_cond;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_value_expr::lower_cond_value_input;
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering_associated_input::{
    lower_assignment_inputs, lower_local_statement_input,
};
use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::MirType;
use std::collections::BTreeMap;

use super::exit as parts_exit;
use super::var_map_scope::publish_emission_cache;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
mod helpers;
use helpers::{attach_phi_args_if_continue_or_break, current_value_for_join};


pub(in crate::mir::builder) struct CondUpdateBranch {
    pub(in crate::mir::builder) updates: BTreeMap<String, crate::mir::ValueId>,
    pub(in crate::mir::builder) effects: Vec<CoreEffectPlan>,
    pub(in crate::mir::builder) exit: Option<CoreExitPlan>,
    pub(in crate::mir::builder) saw_assignment: bool,
}

/// Port-aware conditional-update lowering used by located direct-loop input.
///
/// This is deliberately an associated-input facade over the existing
/// conditional-update/Select owner. It performs the same shape preflight and
/// branch semantics as the raw entry, while obtaining every statement and
/// expression child from the caller's source port.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::builder) fn try_lower_conditional_update_if_input<'input, P>(
    port: &P,
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    condition: P::ExprInput<'input>,
    then_body: &P::BodyInput<'input>,
    else_body: Option<&P::BodyInput<'input>>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let has_update = has_any_assignment_input(port, then_body)
        || else_body.is_some_and(|body| has_any_assignment_input(port, body));
    if !has_update {
        return Ok(None);
    }
    if !is_conditional_update_branch_supported_input(port, then_body)
        || else_body.is_some_and(|body| !is_conditional_update_branch_supported_input(port, body))
    {
        return Ok(None);
    }

    let then_branch = collect_conditional_update_branch_input(
        port,
        builder,
        current_bindings,
        then_body,
        error_prefix,
    )?;
    let else_branch = match else_body {
        Some(body) => collect_conditional_update_branch_input(
            port,
            builder,
            current_bindings,
            body,
            error_prefix,
        )?,
        None => CondUpdateBranch {
            updates: BTreeMap::new(),
            effects: Vec::new(),
            exit: None,
            saw_assignment: false,
        },
    };

    if !then_branch.saw_assignment && !else_branch.saw_assignment {
        return Ok(None);
    }

    let (cond_id, cond_effects) =
        lower_cond_value_input(port, condition, builder, current_bindings, error_prefix)?;
    let plans = lower_conditional_update_if_from_branch_values(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        break_phi_dsts,
        cond_id,
        cond_effects,
        then_branch,
        else_branch,
        error_prefix,
    )?;

    Ok(Some(plans))
}

fn has_any_assignment_input<'input, P>(port: &P, body: &P::BodyInput<'input>) -> bool
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    port.body_statements(body).iter().any(|stmt| match stmt {
        ASTNode::Assignment { .. } => true,
        ASTNode::Local { initial_values, .. } => initial_values.iter().all(|init| init.is_some()),
        _ => false,
    })
}

fn is_conditional_update_branch_supported_input<'input, P>(
    port: &P,
    body: &P::BodyInput<'input>,
) -> bool
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let statements = port.body_statements(body);
    let mut updated_vars = std::collections::BTreeSet::new();
    let mut saw_exit = false;
    for (idx, stmt) in statements.iter().enumerate() {
        let is_last = idx + 1 == statements.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if saw_exit
                    || !matches!(target.as_ref(), ASTNode::Variable { .. })
                    || !helpers::is_pure_value_expr(value)
                {
                    return false;
                }
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return false;
                };
                if !updated_vars.insert(name.clone()) {
                    return false;
                }
            }
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if saw_exit || variables.len() != initial_values.len() {
                    return false;
                }
                if initial_values.iter().any(|init| {
                    init.as_deref()
                        .map_or(true, |init| !helpers::is_pure_value_expr(init))
                }) {
                    return false;
                }
                for name in variables {
                    if !updated_vars.insert(name.clone()) {
                        return false;
                    }
                }
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last || saw_exit {
                    return false;
                }
                saw_exit = true;
            }
            _ => return false,
        }
    }
    true
}

fn collect_conditional_update_branch_input<'input, P>(
    port: &P,
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    body: &P::BodyInput<'input>,
    error_prefix: &str,
) -> Result<CondUpdateBranch, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    let statements = port.body_statements(body);
    let mut updates = BTreeMap::new();
    let mut effects = Vec::new();
    let mut exit = None;
    let mut saw_assignment = false;

    for (idx, _) in statements.iter().enumerate() {
        let statement = port.body_stmt(body, idx).map_err(|error| error.render())?;
        let is_last = idx + 1 == statements.len();
        let is_assignment_or_local = matches!(
            port.stmt_syntax(&statement),
            ASTNode::Assignment { .. } | ASTNode::Local { .. }
        );
        if is_assignment_or_local {
            collect_conditional_update_assignment_or_local_input(
                port,
                builder,
                current_bindings,
                statement,
                &mut updates,
                &mut effects,
                &mut saw_assignment,
                error_prefix,
            )?;
            continue;
        }
        match port.stmt_syntax(&statement) {
            ASTNode::Break { .. } => {
                if !is_last || exit.is_some() {
                    return Err(format!("{error_prefix}: break not at tail"));
                }
                exit = Some(CoreExitPlan::Break(1));
            }
            ASTNode::Continue { .. } => {
                if !is_last || exit.is_some() {
                    return Err(format!("{error_prefix}: continue not at tail"));
                }
                exit = Some(CoreExitPlan::Continue(1));
            }
            _ => {
                return Err(format!(
                    "{error_prefix}: conditional update has unsupported stmt"
                ));
            }
        }
    }

    Ok(CondUpdateBranch {
        updates,
        effects,
        exit,
        saw_assignment,
    })
}

#[allow(clippy::too_many_arguments)]
fn collect_conditional_update_assignment_or_local_input<'input, P>(
    port: &P,
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    statement: P::StmtInput<'input>,
    updates: &mut BTreeMap<String, crate::mir::ValueId>,
    effects: &mut Vec<CoreEffectPlan>,
    saw_assignment: &mut bool,
    error_prefix: &str,
) -> Result<bool, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
{
    match port.stmt_syntax(&statement) {
        ASTNode::Assignment { .. } => {
            let target = port
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentTarget)
                .map_err(|error| error.render())?;
            let value = port
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .map_err(|error| error.render())?;
            let (binding, mut new_effects) = lower_assignment_inputs(
                port,
                target,
                value,
                builder,
                current_bindings,
                error_prefix,
            )?;
            let Some((name, value_id)) = binding else {
                return Err(format!("{error_prefix}: non-variable assignment"));
            };
            if updates.contains_key(&name) {
                return Err(format!("{error_prefix}: duplicate update for {name}"));
            }
            effects.append(&mut new_effects);
            updates.insert(name, value_id);
            *saw_assignment = true;
            Ok(true)
        }
        ASTNode::Local { .. } => {
            let (inits, mut init_effects) = lower_local_statement_input(
                port,
                &statement,
                builder,
                current_bindings,
                error_prefix,
            )?;
            effects.append(&mut init_effects);
            for (name, value_id) in inits {
                if updates.contains_key(&name) {
                    return Err(format!("{error_prefix}: duplicate update for {name}"));
                }
                updates.insert(name, value_id);
                *saw_assignment = true;
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[allow(clippy::too_many_arguments)]
fn lower_conditional_update_if_from_branch_values(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    cond_id: crate::mir::ValueId,
    cond_effects: Vec<CoreEffectPlan>,
    then_branch: CondUpdateBranch,
    else_branch: CondUpdateBranch,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = effects_to_plans(cond_effects);

    let mut select_effects = Vec::new();
    select_effects.extend(then_branch.effects);
    select_effects.extend(else_branch.effects);

    let mut update_vars = BTreeMap::<String, ()>::new();
    for key in then_branch.updates.keys() {
        update_vars.insert(key.clone(), ());
    }
    for key in else_branch.updates.keys() {
        update_vars.insert(key.clone(), ());
    }

    for (var, _) in update_vars {
        let current = current_value_for_join(builder, current_bindings, &var, error_prefix)?;
        let then_val = then_branch.updates.get(&var).copied().unwrap_or(current);
        let else_val = else_branch.updates.get(&var).copied().unwrap_or(current);
        let ty = builder
            .function_state
            .type_ctx
            .get_type(current)
            .cloned()
            .unwrap_or(MirType::Unknown);
        let dst = builder.alloc_typed(ty);
        select_effects.push(CoreEffectPlan::Select {
            dst,
            cond: cond_id,
            then_val,
            else_val,
        });
        if carrier_phis.contains_key(&var) {
            carrier_updates.insert(var.clone(), dst);
        }
        if carrier_phis.contains_key(&var) || current_bindings.contains_key(&var) {
            current_bindings.insert(var.clone(), dst);
        }
        publish_emission_cache(builder, var, dst);
    }

    plans.extend(effects_to_plans(select_effects));

    if let Some(exit) = then_branch.exit {
        let exit = attach_phi_args_if_continue_or_break(
            builder,
            exit,
            carrier_step_phis,
            break_phi_dsts,
            current_bindings,
            error_prefix,
        )?;
        plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
            cond: cond_id,
            exit,
        }));
    }
    if let Some(exit) = else_branch.exit {
        let (cond_neg, neg_effects) = negate_bool_cond(builder, cond_id);
        plans.extend(effects_to_plans(neg_effects));
        let exit = attach_phi_args_if_continue_or_break(
            builder,
            exit,
            carrier_step_phis,
            break_phi_dsts,
            current_bindings,
            error_prefix,
        )?;
        plans.push(CorePlan::Effect(CoreEffectPlan::ExitIf {
            cond: cond_neg,
            exit,
        }));
    }

    Ok(plans)
}
