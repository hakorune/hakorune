//! Continue-if route lowering functions.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::control_flow::recipes::loop_cond_continue_only::ContinueOnlyStmtRecipe;
use crate::mir::builder::control_flow::recipes::refs::StmtSpan;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::loop_cond_co_block::lower_continue_only_block;
use super::loop_cond_co_helpers::get_body_span;
use super::loop_cond_co_stmt::lower_stmt_ast;

const LOOP_COND_CONTINUE_ONLY_ERR: &str = "[normalizer] loop_cond_continue_only";

/// Lower prelude span for continue-if pattern.
pub(super) fn lower_continue_if_prelude_span(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    then_body: &BodyView<'_>,
    prelude_span: StmtSpan,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut out = Vec::new();
    let prelude_body = get_body_span(
        then_body,
        prelude_span,
        LOOP_COND_CONTINUE_ONLY_ERR,
        "continue-if prelude",
    )?;
    for stmt in prelude_body {
        let mut plans = lower_stmt_ast(
            builder,
            current_bindings,
            carrier_phis,
            carrier_step_phis,
            carrier_updates,
            stmt,
        )?;
        out.append(&mut plans);
    }
    Ok(out)
}

/// Lower continue-if without else branch.
pub(super) fn lower_continue_if_no_else(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &BodyView<'_>,
    prelude_span: StmtSpan,
) -> Result<Vec<LoweredRecipe>, String> {
    let saved_map = builder.variable_ctx.variable_map.clone();
    let saved_bindings = current_bindings.clone();

    builder.variable_ctx.variable_map = saved_map.clone();
    let mut branch_bindings = saved_bindings.clone();
    let mut then_plans = lower_continue_if_prelude_span(
        builder,
        &mut branch_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        then_body,
        prelude_span,
    )?;
    if then_plans.iter().any(|p| matches!(p, CorePlan::Exit(_))) {
        return Err(format!(
            "{LOOP_COND_CONTINUE_ONLY_ERR}: continue-if prelude contains exit"
        ));
    }
    let exit = parts::exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        &branch_bindings,
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?;
    then_plans.push(CorePlan::Exit(exit));

    builder.variable_ctx.variable_map = saved_map;
    *current_bindings = saved_bindings;

    let cond_view = CondBlockView::from_expr(condition);
    let mut then_plans_once = Some(then_plans);
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
        None,
        &|_name, _bindings| false,
    )
}

/// Lower continue-if with group prelude.
pub(super) fn lower_continue_if_group_prelude(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &BodyView<'_>,
    prelude_span: StmtSpan,
    prelude_items: &[ContinueOnlyStmtRecipe],
) -> Result<Vec<LoweredRecipe>, String> {
    let saved_map = builder.variable_ctx.variable_map.clone();
    let saved_bindings = current_bindings.clone();

    builder.variable_ctx.variable_map = saved_map.clone();
    let mut branch_bindings = saved_bindings.clone();
    let prelude_body = get_body_span(
        then_body,
        prelude_span,
        LOOP_COND_CONTINUE_ONLY_ERR,
        "continue-if prelude",
    )?;
    let prelude_view = BodyView::Slice(prelude_body);
    let mut then_plans = lower_continue_only_block(
        builder,
        &mut branch_bindings,
        carrier_phis,
        carrier_step_phis,
        carrier_updates,
        &prelude_view,
        prelude_items,
    )?;
    let exit = parts::exit::build_continue_with_phi_args(
        builder,
        carrier_step_phis,
        &branch_bindings,
        LOOP_COND_CONTINUE_ONLY_ERR,
    )?;
    then_plans.push(CorePlan::Exit(exit));

    builder.variable_ctx.variable_map = saved_map;
    *current_bindings = saved_bindings;

    let cond_view = CondBlockView::from_expr(condition);
    let mut then_plans_once = Some(then_plans);
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
        None,
        &|_name, _bindings| false,
    )
}
