//! Special exit-if lowering shapes.
//!
//! These helpers are ordered probes used by `if_exit.rs`. They return `Ok(None)`
//! when a shape does not match and must not claim general exit-if ownership.

use super::exit_branch::{
    lower_exit_branch_with_prelude, lower_exit_branch_with_prelude_with_break_phi_args,
    split_exit_branch,
};
use super::if_exit::{lower_if_exit_stmt, lower_if_exit_stmt_with_break_phi_args};
use super::stmt::lower_return_prelude_stmt;
use super::var_map_scope::with_saved_variable_map;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::normalizer::cond_lowering_entry::lower_cond_branch;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(super) fn try_lower_return_before_continue_view(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    if else_body.is_some() || then_body.len() < 2 {
        return Ok(None);
    }
    let Some(last) = then_body.last() else {
        return Ok(None);
    };
    if !matches!(last, ASTNode::Continue { .. }) {
        return Ok(None);
    }
    let return_if = &then_body[then_body.len() - 2];
    let ASTNode::If {
        condition: inner_cond,
        then_body: inner_then,
        else_body: inner_else,
        ..
    } = return_if
    else {
        return Ok(None);
    };
    if inner_else.is_some() {
        return Ok(None);
    }
    let Ok((inner_prelude, _inner_exit, inner_is_return)) =
        split_exit_branch(inner_then, error_prefix)
    else {
        return Ok(None);
    };
    if !inner_is_return || !inner_prelude.is_empty() {
        return Ok(None);
    }

    let then_plans = with_saved_variable_map(builder, |builder| {
        let mut branch_bindings = current_bindings.clone();
        let mut then_plans = Vec::new();
        for stmt in &then_body[..then_body.len() - 2] {
            then_plans.extend(lower_return_prelude_stmt(
                builder,
                &mut branch_bindings,
                carrier_step_phis,
                break_phi_dsts,
                stmt,
                error_prefix,
            )?);
        }
        then_plans.extend(match break_phi_dsts {
            Some(break_phi_dsts) => lower_if_exit_stmt_with_break_phi_args(
                builder,
                &branch_bindings,
                carrier_step_phis,
                break_phi_dsts,
                inner_cond,
                inner_then,
                None,
                error_prefix,
            )?,
            None => lower_if_exit_stmt(
                builder,
                &branch_bindings,
                carrier_step_phis,
                inner_cond,
                inner_then,
                None,
                error_prefix,
            )?,
        });
        then_plans.extend(match break_phi_dsts {
            Some(break_phi_dsts) => lower_exit_branch_with_prelude_with_break_phi_args(
                builder,
                &branch_bindings,
                carrier_step_phis,
                break_phi_dsts,
                &[],
                last,
                error_prefix,
            )?,
            None => lower_exit_branch_with_prelude(
                builder,
                &branch_bindings,
                carrier_step_phis,
                &[],
                last,
                error_prefix,
            )?,
        });
        Ok(then_plans)
    })?;

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        None,
        Vec::new(),
        error_prefix,
    )?;
    Ok(Some(plans))
}

pub(super) fn try_lower_nested_exit_if_view(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    if else_body.is_some() {
        return Ok(None);
    }
    let Some(last) = then_body.last() else {
        return Ok(None);
    };
    let ASTNode::If {
        condition: inner_cond,
        then_body: inner_then,
        else_body: inner_else,
        ..
    } = last
    else {
        return Ok(None);
    };
    if split_exit_branch(inner_then, error_prefix).is_err() {
        return Ok(None);
    }
    if let Some(inner_else) = inner_else.as_ref() {
        if split_exit_branch(inner_else, error_prefix).is_err() {
            return Ok(None);
        }
    }

    let then_plans = with_saved_variable_map(builder, |builder| {
        let mut branch_bindings = current_bindings.clone();
        let mut then_plans = Vec::new();
        if then_body.len() > 1 {
            for stmt in &then_body[..then_body.len() - 1] {
                then_plans.extend(lower_return_prelude_stmt(
                    builder,
                    &mut branch_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    stmt,
                    error_prefix,
                )?);
            }
        }
        then_plans.extend(match break_phi_dsts {
            Some(break_phi_dsts) => lower_if_exit_stmt_with_break_phi_args(
                builder,
                &branch_bindings,
                carrier_step_phis,
                break_phi_dsts,
                inner_cond,
                inner_then,
                inner_else.as_ref(),
                error_prefix,
            )?,
            None => lower_if_exit_stmt(
                builder,
                &branch_bindings,
                carrier_step_phis,
                inner_cond,
                inner_then,
                inner_else.as_ref(),
                error_prefix,
            )?,
        });
        Ok(then_plans)
    })?;

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        None,
        Vec::new(),
        error_prefix,
    )?;

    Ok(Some(plans))
}

pub(super) fn try_lower_else_nested_exit_if_view(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    cond_view: &CondBlockView,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
    error_prefix: &str,
) -> Result<Option<Vec<LoweredRecipe>>, String> {
    let Some(else_body) = else_body else {
        return Ok(None);
    };
    if else_body.len() != 1 {
        return Ok(None);
    }

    let Ok((then_prelude, then_exit, _then_is_return)) = split_exit_branch(then_body, error_prefix)
    else {
        return Ok(None);
    };

    let ASTNode::If {
        condition: inner_cond,
        then_body: inner_then,
        else_body: inner_else,
        ..
    } = &else_body[0]
    else {
        return Ok(None);
    };

    if split_exit_branch(inner_then, error_prefix).is_err() {
        return Ok(None);
    }
    if let Some(inner_else) = inner_else.as_ref() {
        if split_exit_branch(inner_else, error_prefix).is_err() {
            return Ok(None);
        }
    }

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

    let else_plans = with_saved_variable_map(builder, |builder| {
        let branch_bindings = current_bindings.clone();
        match break_phi_dsts {
            Some(break_phi_dsts) => lower_if_exit_stmt_with_break_phi_args(
                builder,
                &branch_bindings,
                carrier_step_phis,
                break_phi_dsts,
                inner_cond,
                inner_then,
                inner_else.as_ref(),
                error_prefix,
            ),
            None => lower_if_exit_stmt(
                builder,
                &branch_bindings,
                carrier_step_phis,
                inner_cond,
                inner_then,
                inner_else.as_ref(),
                error_prefix,
            ),
        }
    })?;

    let plans = lower_cond_branch(
        builder,
        current_bindings,
        cond_view,
        then_plans,
        Some(else_plans),
        Vec::new(),
        error_prefix,
    )?;

    Ok(Some(plans))
}
