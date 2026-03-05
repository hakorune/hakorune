//! Exit-branch lowering helpers (Parts SSOT).
//!
//! Functions moved from features::exit_branch to make parts independent.
//! SSOT: docs/development/current/main/design/recipe-tree-and-parts-ssot.md

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::PlanNormalizer;
use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::exit as parts_exit;
use super::stmt as parts_stmt;
use super::var_map_scope::with_saved_variable_map;
use super::super::steps::effects_to_plans;

pub(in crate::mir::builder) fn split_exit_branch<'a>(
    body: &'a [ASTNode],
    error_prefix: &str,
) -> Result<(Vec<&'a ASTNode>, &'a ASTNode, bool), String> {
    let Some(last) = body.last() else {
        return Err(format!("{error_prefix}: if body must be single-exit (empty)"));
    };
    if matches!(
        last,
        ASTNode::Return { .. } | ASTNode::Break { .. } | ASTNode::Continue { .. }
    ) {
        if body.len() == 1 {
            return Ok((Vec::new(), last, matches!(last, ASTNode::Return { .. })));
        }
        let mut prelude = Vec::new();
        for stmt in &body[..body.len() - 1] {
            prelude.push(stmt);
        }
        return Ok((prelude, last, matches!(last, ASTNode::Return { .. })));
    }
    Err(format!(
        "{error_prefix}: if body must be single-exit{}",
        match last {
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                let then_last = then_body
                    .last()
                    .map(|n| n.node_type())
                    .unwrap_or("empty");
                let else_last = else_body
                    .as_ref()
                    .and_then(|b| b.last())
                    .map(|n| n.node_type())
                    .unwrap_or("none");
                format!(" (last=If then_last={then_last} else_last={else_last})")
            }
            _ => format!(" (last={})", last.node_type()),
        }
    ))
}

pub(in crate::mir::builder) fn lower_exit_branch_with_prelude(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_exit_branch_with_prelude_impl(
        builder,
        current_bindings,
        carrier_step_phis,
        None,
        prelude,
        exit_stmt,
        error_prefix,
    )
}

pub(in crate::mir::builder) fn lower_exit_branch_with_prelude_with_break_phi_args(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    lower_exit_branch_with_prelude_impl(
        builder,
        current_bindings,
        carrier_step_phis,
        Some(break_phi_dsts),
        prelude,
        exit_stmt,
        error_prefix,
    )
}

fn lower_exit_branch_with_prelude_impl(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    prelude: &[&ASTNode],
    exit_stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    if !prelude.is_empty() {
        return with_saved_variable_map(builder, |builder| {
            let mut plans = Vec::new();
            let mut branch_bindings = current_bindings.clone();
            for stmt in prelude {
                plans.extend(parts_stmt::lower_return_prelude_stmt(
                    builder,
                    &mut branch_bindings,
                    carrier_step_phis,
                    break_phi_dsts,
                    stmt,
                    error_prefix,
                )?);
            }
            plans.extend(lower_exit_branch(
                builder,
                &branch_bindings,
                carrier_step_phis,
                break_phi_dsts,
                exit_stmt,
                error_prefix,
            )?);
            Ok(plans)
        });
    }
    let mut plans = Vec::new();
    plans.extend(lower_exit_branch(
        builder,
        current_bindings,
        carrier_step_phis,
        break_phi_dsts,
        exit_stmt,
        error_prefix,
    )?);
    Ok(plans)
}

fn lower_exit_branch(
    builder: &mut MirBuilder,
    current_bindings: &BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: Option<&BTreeMap<String, crate::mir::ValueId>>,
    stmt: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<LoweredRecipe>, String> {
    let mut plans = Vec::new();
    let exit = match stmt {
        ASTNode::Break { .. } => match break_phi_dsts {
            Some(break_phi_dsts) => {
                parts_exit::build_break_with_phi_args(break_phi_dsts, current_bindings, error_prefix)?
            }
            None => CoreExitPlan::Break(1),
        },
        ASTNode::Continue { .. } => {
            parts_exit::build_continue_with_phi_args(
                builder,
                carrier_step_phis,
                current_bindings,
                error_prefix,
            )?
        }
        ASTNode::Return { value, .. } => {
            let Some(value) = value.as_ref() else {
                return Err(format!("{error_prefix}: return without value"));
            };
            let (value_id, effects) =
                PlanNormalizer::lower_value_ast(value, builder, current_bindings)?;
            plans.extend(effects_to_plans(effects));
            CoreExitPlan::Return(Some(value_id))
        }
        _ => {
            return Err(format!(
                "{error_prefix}: if body must be break/continue/return"
            ));
        }
    };
    plans.push(CorePlan::Exit(exit));
    Ok(plans)
}
