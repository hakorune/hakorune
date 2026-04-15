use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::facts::stmt_view::try_build_stmt_only_block_recipe;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

use super::nested_loop_recipe_handoff::lower_loop_scan_phi_vars_nested_loop_recipe;
use super::recipe::NestedLoopRecipe;

const LOOP_SCAN_PHI_VARS_ERR: &str = "[normalizer] loop_scan_phi_vars_v0";

pub(in crate::mir::builder) fn lower_loop_scan_phi_vars_found_if_branch_body(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_step_phis: &BTreeMap<String, crate::mir::ValueId>,
    break_phi_dsts: &BTreeMap<String, crate::mir::ValueId>,
    stmts: &[ASTNode],
) -> Result<Vec<LoweredRecipe>, String> {
    const ALLOW_EXTENDED: bool = true;
    let mut plans = Vec::new();

    let mut idx = 0;
    while idx < stmts.len() {
        if matches!(stmts[idx], ASTNode::Loop { .. } | ASTNode::While { .. }) {
            let nested = match &stmts[idx] {
                ASTNode::Loop {
                    condition, body, ..
                }
                | ASTNode::While {
                    condition, body, ..
                } => NestedLoopRecipe {
                    cond_view: CondBlockView::from_expr(condition),
                    loop_stmt: stmts[idx].clone(),
                    body: RecipeBody::new(body.to_vec()),
                    body_stmt_only: try_build_stmt_only_block_recipe(body),
                },
                _ => unreachable!(),
            };
            plans.extend(lower_loop_scan_phi_vars_nested_loop_recipe(
                builder,
                current_bindings,
                carrier_step_phis,
                break_phi_dsts,
                &nested,
            )?);
            idx += 1;
            continue;
        }

        let start = idx;
        idx += 1;
        while idx < stmts.len() {
            if matches!(stmts[idx], ASTNode::Loop { .. } | ASTNode::While { .. }) {
                break;
            }
            idx += 1;
        }

        let slice = &stmts[start..idx];
        let Some(no_exit) = try_build_no_exit_block_recipe(slice, ALLOW_EXTENDED) else {
            return Err(format!(
                "[freeze:contract][loop_scan_phi_vars_v0] found_if_branch_linear_not_no_exit: ctx={}",
                LOOP_SCAN_PHI_VARS_ERR
            ));
        };
        let verified = parts::entry::verify_no_exit_block_with_pre(
            &no_exit.arena,
            &no_exit.block,
            LOOP_SCAN_PHI_VARS_ERR,
            Some(current_bindings),
        )?;
        plans.extend(parts::entry::lower_no_exit_block_verified(
            builder,
            current_bindings,
            carrier_step_phis,
            Some(break_phi_dsts),
            verified,
            LOOP_SCAN_PHI_VARS_ERR,
        )?);
    }

    Ok(plans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, Span};

    fn span() -> Span {
        Span::unknown()
    }

    #[test]
    fn loop_scan_phi_vars_found_if_branch_rejects_non_no_exit_linear_slice() {
        let mut builder = MirBuilder::new();
        let mut current_bindings = BTreeMap::new();
        let carrier_step_phis = BTreeMap::new();
        let break_phi_dsts = BTreeMap::new();
        let stmts = vec![ASTNode::Return {
            value: None,
            span: span(),
        }];

        let err = lower_loop_scan_phi_vars_found_if_branch_body(
            &mut builder,
            &mut current_bindings,
            &carrier_step_phis,
            &break_phi_dsts,
            &stmts,
        )
        .expect_err("unsupported linear branch slice should reject");

        assert!(err.contains("found_if_branch_linear_not_no_exit"));
    }
}
