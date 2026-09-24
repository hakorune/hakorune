//! Group-if and nested-loop route lowering functions.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::loop_cond_continue_only::{
    ContinueOnlyRecipe, ContinueOnlyStmtRecipe,
};
use crate::mir::builder::control_flow::recipes::refs::{StmtRef, StmtSpan};
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::loop_cond_co_block::lower_continue_only_block;
use super::loop_cond_co_helpers::{get_body_span, get_body_stmt, map_mutates_existing_vars};

const LOOP_COND_CONTINUE_ONLY_ERR: &str = "[normalizer] loop_cond_continue_only";

/// Lower a group-if statement.
pub(super) fn lower_group_if(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    if_stmt: &StmtRef,
    body: &BodyView<'_>,
    then_body: &ContinueOnlyRecipe,
    else_body: Option<&ContinueOnlyRecipe>,
) -> Result<Vec<LoweredRecipe>, String> {
    let stmt = get_body_stmt(body, *if_stmt, LOOP_COND_CONTINUE_ONLY_ERR)?;
    let ASTNode::If { condition, .. } = stmt else {
        return Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: recipe if mismatch (GroupIf)"
        ));
    };
    let pre_if_map = builder.function_state.variable_ctx.variable_map.clone();
    let pre_bindings = current_bindings.clone();

    // Then
    builder.function_state.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();
    let then_view = BodyView::Recipe(&then_body.body);
    let then_plans = lower_continue_only_block(
        builder,
        current_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        &then_view,
        &then_body.items,
    )?;
    let then_map = builder.function_state.variable_ctx.variable_map.clone();

    // Else
    builder.function_state.variable_ctx.variable_map = pre_if_map.clone();
    *current_bindings = pre_bindings.clone();
    let else_plans = match else_body {
        Some(body) => {
            let else_view = BodyView::Recipe(&body.body);
            Some(lower_continue_only_block(
                builder,
                current_bindings,
                carrier_phis,
                carrier_step_phis,
                carrier_updates,
                &else_view,
                &body.items,
            )?)
        }
        None => None,
    };
    let else_map = builder.function_state.variable_ctx.variable_map.clone();

    // Fallthrough mutation is out-of-scope: no join generation here.
    if map_mutates_existing_vars(&pre_if_map, &then_map)
        || map_mutates_existing_vars(&pre_if_map, &else_map)
    {
        return Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: group-if fallthrough mutates existing vars (join out-of-scope)"
        ));
    }

    builder.function_state.variable_ctx.variable_map = pre_if_map;
    *current_bindings = pre_bindings;
    let cond_view = CondBlockView::from_expr(condition);
    let mut then_plans_once = Some(then_plans);
    let mut else_plans_once = else_plans;
    let has_else = else_plans_once.is_some();
    let mut lower_else =
        |_builder: &mut MirBuilder, _bindings: &mut BTreeMap<String, crate::mir::ValueId>| {
            Ok(else_plans_once.take().ok_or_else(|| {
                format!("{LOOP_COND_CONTINUE_ONLY_ERR}: internal error: else_plans consumed twice")
            })?)
        };
    let lower_else: Option<
        &mut dyn FnMut(
            &mut MirBuilder,
            &mut BTreeMap<String, crate::mir::ValueId>,
        ) -> Result<Vec<LoweredRecipe>, String>,
    > = if has_else {
        Some(&mut lower_else)
    } else {
        None
    };

    parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        LOOP_COND_CONTINUE_ONLY_ERR,
        &mut |_builder, _bindings| {
            Ok(then_plans_once.take().ok_or_else(|| {
                format!("{LOOP_COND_CONTINUE_ONLY_ERR}: internal error: then_plans consumed twice")
            })?)
        },
        lower_else,
        &|_name, _bindings| false,
    )
}
