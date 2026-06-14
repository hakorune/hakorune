//! Exit-if lowering helpers (Parts).
//!
//! Scope: behavior-preserving extraction of existing lowering logic.
//! SSOT for lower_if_exit_stmt.

use super::exit_branch::{
    lower_exit_branch_with_prelude, lower_exit_branch_with_prelude_with_break_phi_args,
    split_exit_branch,
};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_entry::lower_cond_branch;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::if_exit_special::{
    try_lower_else_nested_exit_if_view, try_lower_nested_exit_if_view,
    try_lower_return_before_continue_view,
};

/// View-first exit-if lowering (SSOT).
pub(in crate::mir::builder) fn lower_if_exit_stmt_view(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_if_exit_stmt_impl_view(
        builder,
        current_bindings,
        carrier_step_phis,
        None,
        cond_view,
        then_body,
        else_body,
        error_prefix,
    )
}

/// View-first exit-if lowering with break PHI args (SSOT).
pub(in crate::mir::builder) fn lower_if_exit_stmt_with_break_phi_args_view(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_if_exit_stmt_impl_view(
        builder,
        current_bindings,
        carrier_step_phis,
        Some(break_phi_dsts),
        cond_view,
        then_body,
        else_body,
        error_prefix,
    )
}

/// ASTNode-based wrapper (delegates to view-first SSOT).
pub(in crate::mir::builder) fn lower_if_exit_stmt(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let cond_view = CondBlockView::from_expr(condition);
    lower_if_exit_stmt_view(
        builder,
        current_bindings,
        carrier_step_phis,
        &cond_view,
        then_body,
        else_body,
        error_prefix,
    )
}

/// ASTNode-based wrapper with break PHI args (delegates to view-first SSOT).
pub(in crate::mir::builder) fn lower_if_exit_stmt_with_break_phi_args(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let cond_view = CondBlockView::from_expr(condition);
    lower_if_exit_stmt_with_break_phi_args_view(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        &cond_view,
        then_body,
        else_body,
        error_prefix,
    )
}

fn lower_if_exit_stmt_impl_view(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if let Some(plans) = try_lower_return_before_continue_view(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        cond_view,
        then_body,
        else_body,
        error_prefix,
    )? {
        return Ok(plans);
    }
    if let Some(plans) = try_lower_nested_exit_if_view(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        cond_view,
        then_body,
        else_body,
        error_prefix,
    )? {
        return Ok(plans);
    }
    if let Some(plans) = try_lower_else_nested_exit_if_view(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        cond_view,
        then_body,
        else_body,
        error_prefix,
    )? {
        return Ok(plans);
    }
    let (then_prelude, then_exit, then_is_return) = split_exit_branch(then_body, error_prefix)?;
    if then_is_return && !then_prelude.is_empty() && else_body.is_some() {
        return Err(format!(
            "{error_prefix}: return prelude cannot have else branch"
        ));
    }
    let (else_prelude, else_exit, _else_is_return) = match else_body {
        Some(body) => {
            let (prelude, exit, is_return) = split_exit_branch(body, error_prefix)?;
            // Allow else-prelude for exit-if lowering as well.
            // This keeps then/else branch handling symmetric and supports
            // `if { ...; continue } else { <no-exit prelude>; break }` shapes.
            (Some(prelude), Some(exit), is_return)
        }
        None => (None, None, false),
    };

    let then_plans = match break_phi_dsts {
        Some(break_phi_dsts) => lower_exit_branch_with_prelude_with_break_phi_args(
            builder,
            current_bindings,
            carrier_step_phis,
            break_phi_dsts,
            &then_prelude,
            then_exit,
            error_prefix,
        )?,
        None => lower_exit_branch_with_prelude(
            builder,
            current_bindings,
            carrier_step_phis,
            &then_prelude,
            then_exit,
            error_prefix,
        )?,
    };
    let else_plans = match else_exit {
        Some(exit) => Some(match break_phi_dsts {
            Some(break_phi_dsts) => lower_exit_branch_with_prelude_with_break_phi_args(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                else_prelude.as_deref().unwrap_or(&[]),
                exit,
                error_prefix,
            )?,
            None => lower_exit_branch_with_prelude(
                builder,
                current_bindings,
                carrier_step_phis,
                else_prelude.as_deref().unwrap_or(&[]),
                exit,
                error_prefix,
            )?,
        }),
        None => None,
    };

    lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        else_plans,
        Vec::new(),
        error_prefix,
    )
}
