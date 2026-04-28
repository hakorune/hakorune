//! Conditional update + join feature (Parts SSOT, loop-cond bodies).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::normalizer::common::negate_bool_cond;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_entry::lower_cond_value;
use crate::mir::builder::control_flow::plan::recipe_tree::common::ExitKind;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeItem;
use crate::mir::builder::control_flow::plan::{
    CoreEffectPlan, CoreExitPlan, CorePlan, LoweredRecipe,
};
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use std::collections::BTreeMap;

use super::exit as parts_exit;
use super::if_general as parts_if_general;
use crate::mir::builder::control_flow::plan::steps::effects_to_plans;
mod helpers;
use helpers::{
    attach_phi_args_if_continue_or_break, collect_conditional_update_assignment_or_local,
    current_value_for_join, has_any_assignment, is_conditional_update_branch_supported,
};

pub(in crate::mir::builder) use helpers::collect_conditional_update_branch;

pub(in crate::mir::builder) struct CondUpdateBranch {
    pub(in crate::mir::builder) updates: BTreeMap<String, crate::mir::ValueId>,
    pub(in crate::mir::builder) effects: Vec<CoreEffectPlan>,
    pub(in crate::mir::builder) exit: Option<CoreExitPlan>,
    pub(in crate::mir::builder) saw_assignment: bool,
}

pub(in crate::mir::builder) fn try_lower_conditional_update_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    try_lower_conditional_update_if_impl(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        None,
        condition,
        then_body,
        else_body,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn try_lower_conditional_update_if_with_break_phi_args(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    try_lower_conditional_update_if_impl(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        Some(break_phi_dsts),
        condition,
        then_body,
        else_body,
        error_prefix,
    )
}

fn try_lower_conditional_update_if_impl(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    let has_update =
        has_any_assignment(then_body) || else_body.map_or(false, |body| has_any_assignment(body));
    if !has_update {
        return Ok(None);
    }
    if !is_conditional_update_branch_supported(then_body)
        || else_body.map_or(false, |body| !is_conditional_update_branch_supported(body))
    {
        return Ok(None);
    }

    let then_branch =
        collect_conditional_update_branch(builder, current_bindings, then_body, error_prefix)?;
    let else_branch = match else_body {
        Some(body) => {
            collect_conditional_update_branch(builder, current_bindings, body, error_prefix)?
        }
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

    let plans = lower_conditional_update_if_from_branches(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        break_phi_dsts,
        condition,
        then_branch,
        else_branch,
        error_prefix,
    )?;

    Ok(Some(plans))
}

pub(in crate::mir::builder) fn lower_conditional_update_if_assume(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let then_branch =
        collect_conditional_update_branch(builder, current_bindings, then_body, error_prefix)?;
    let else_branch = match else_body {
        Some(body) => {
            collect_conditional_update_branch(builder, current_bindings, body, error_prefix)?
        }
        None => CondUpdateBranch {
            updates: BTreeMap::new(),
            effects: Vec::new(),
            exit: None,
            saw_assignment: false,
        },
    };

    if !then_branch.saw_assignment && !else_branch.saw_assignment {
        return Err(format!(
            "{error_prefix}: conditional update has no assignments"
        ));
    }

    lower_conditional_update_if_from_branches(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        None,
        condition,
        then_branch,
        else_branch,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_conditional_update_if_assume_with_break_phi_args(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let then_branch =
        collect_conditional_update_branch(builder, current_bindings, then_body, error_prefix)?;
    let else_branch = match else_body {
        Some(body) => {
            collect_conditional_update_branch(builder, current_bindings, body, error_prefix)?
        }
        None => CondUpdateBranch {
            updates: BTreeMap::new(),
            effects: Vec::new(),
            exit: None,
            saw_assignment: false,
        },
    };

    if !then_branch.saw_assignment && !else_branch.saw_assignment {
        return Err(format!(
            "{error_prefix}: conditional update has no assignments"
        ));
    }

    lower_conditional_update_if_from_branches(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        Some(break_phi_dsts),
        condition,
        then_branch,
        else_branch,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_conditional_update_if_assume_with_break_phi_args_recipe_first(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_body: Option<&NoExitBlockRecipe>,
    then_exit: Option<ExitKind>,
    else_body: Option<&NoExitBlockRecipe>,
    else_exit: Option<ExitKind>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let then_branch = collect_conditional_update_branch_from_recipe(
        builder,
        current_bindings,
        then_body,
        then_exit,
        error_prefix,
    )?;
    let else_branch = collect_conditional_update_branch_from_recipe(
        builder,
        current_bindings,
        else_body,
        else_exit,
        error_prefix,
    )?;

    if !then_branch.saw_assignment && !else_branch.saw_assignment {
        return Err(format!(
            "{error_prefix}: conditional update has no assignments"
        ));
    }

    lower_conditional_update_if_from_branches_with_cond_view(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        Some(break_phi_dsts),
        cond_view,
        then_branch,
        else_branch,
        error_prefix,
    )
}

fn collect_conditional_update_branch_from_recipe(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    recipe: Option<&NoExitBlockRecipe>,
    exit_kind: Option<ExitKind>,
    error_prefix: &str,
) -> Result<CondUpdateBranch, String> {
    let mut updates = BTreeMap::new();
    let mut effects = Vec::new();
    let mut saw_assignment = false;

    if let Some(recipe) = recipe {
        let body = recipe.arena.get(recipe.block.body_id).ok_or_else(|| {
            format!(
                "[freeze:contract][conditional_update][recipe] {error_prefix}: missing body_id={:?}",
                recipe.block.body_id
            )
        })?;

        for item in &recipe.block.items {
            let RecipeItem::Stmt(stmt_ref) = item else {
                return Err(format!(
                    "[freeze:contract][conditional_update][recipe] {error_prefix}: expected stmt-only NoExitBlockRecipe"
                ));
            };
            let stmt = body.get_ref(*stmt_ref).ok_or_else(|| {
                format!(
                    "[freeze:contract][conditional_update][recipe] {error_prefix}: missing stmt idx={}",
                    stmt_ref.index()
                )
            })?;
            if !collect_conditional_update_assignment_or_local(
                builder,
                current_bindings,
                stmt,
                &mut updates,
                &mut effects,
                &mut saw_assignment,
                error_prefix,
            )? {
                return Err(format!(
                    "[freeze:contract][conditional_update][recipe] {error_prefix}: unexpected stmt {}",
                    stmt.node_type()
                ));
            }
        }
    }

    let exit = match exit_kind {
        None => None,
        Some(ExitKind::Break { depth: 1 }) => Some(CoreExitPlan::Break(1)),
        Some(ExitKind::Continue { depth: 1 }) => Some(CoreExitPlan::Continue(1)),
        Some(other) => {
            return Err(format!(
                "[freeze:contract][conditional_update][recipe] {error_prefix}: unsupported tail exit {other:?}"
            ));
        }
    };

    Ok(CondUpdateBranch {
        updates,
        effects,
        exit,
        saw_assignment,
    })
}

fn lower_conditional_update_if_from_branches(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    condition: &ASTNode,
    then_branch: CondUpdateBranch,
    else_branch: CondUpdateBranch,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let cond_view = CondBlockView::from_expr(condition);
    lower_conditional_update_if_from_branches_with_cond_view(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        break_phi_dsts,
        &cond_view,
        then_branch,
        else_branch,
        error_prefix,
    )
}

fn lower_conditional_update_if_from_branches_with_cond_view(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    cond_view: &CondBlockView,
    then_branch: CondUpdateBranch,
    else_branch: CondUpdateBranch,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let (cond_id, cond_effects) =
        lower_cond_value(builder, current_bindings, cond_view, error_prefix)?;

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
        builder.variable_ctx.variable_map.insert(var, dst);
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

/// Delegate to parts::if_general (SSOT for general if lowering).
pub(in crate::mir::builder) fn try_lower_general_if<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    parts_if_general::try_lower_general_if(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        condition,
        then_body,
        else_body,
        error_prefix,
        lower_block,
    )
}

/// Delegate to parts::if_general recipe-authority wrapper.
///
/// Use this only when the caller already owns a release-valid recipe-first route.
pub(in crate::mir::builder) fn try_lower_general_if_recipe_authority<F>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    parts_if_general::try_lower_general_if_recipe_authority(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        condition,
        then_body,
        else_body,
        error_prefix,
        lower_block,
    )
}

pub(in crate::mir::builder) fn try_lower_general_if_adapter<F, U, T>(
    builder: &mut MirBuilder,
    base_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
    lower_block: F,
    update_state: &mut T,
    mut update_carriers: U,
) -> Result<Option<Vec<LoweredRecipe>>, String>
where
    F: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &BTreeMap<String, crate::mir::ValueId>,
        &[ASTNode],
    ) -> Result<Vec<LoweredRecipe>, String>,
    U: FnMut(&BTreeMap<String, crate::mir::ValueId>, &mut T),
{
    let mut current_bindings = base_bindings.clone();
    let plans = try_lower_general_if(
        builder,
        &mut current_bindings,
        carrier_phis,
        carrier_step_phis,
        condition,
        then_body,
        else_body,
        error_prefix,
        lower_block,
    )?;
    if plans.is_some() {
        update_carriers(&current_bindings, update_state);
    }
    Ok(plans)
}
