//! Arena reference checks for RecipeBlock verification.
//!
//! This module owns mechanical "does this item point at a valid AST node?"
//! checks. Higher-level contract interpretation stays in `verify.rs`.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfContractKind, RecipeItem};
use crate::mir::builder::control_flow::recipes::RecipeBody;

pub(super) fn verify_item_refs_in_range(
    body: &RecipeBody,
    item: &RecipeItem,
    context: &str,
) -> Result<(), String> {
    #[allow(unreachable_patterns)]
    match item {
        RecipeItem::Stmt(r) => {
            if body.get_ref(*r).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                    r.index()
                ));
            }
            Ok(())
        }
        RecipeItem::Exit { kind, stmt } => {
            if body.get_ref(*stmt).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                    stmt.index()
                ));
            }
            match kind {
                ExitKind::Break { depth } | ExitKind::Continue { depth } if *depth != 1 => {
                    return Err(format!(
                        "[freeze:contract][recipe][exit_depth] depth={} unsupported (only depth=1): ctx={context}",
                        depth
                    ));
                }
                _ => {}
            }
            Ok(())
        }
        RecipeItem::IfV2 { contract, .. } if matches!(contract, IfContractKind::Join) => Err(
            format!("[freeze:contract][recipe] exit_only_verifier_saw_if_join: ctx={context}"),
        ),
        RecipeItem::IfV2 {
            if_stmt,
            cond_view: _cond_view,
            contract,
            then_block,
            else_block,
        } => {
            match contract {
                IfContractKind::ExitOnly { .. } | IfContractKind::ExitAllowed { .. } => {}
                _ => {
                    return Err(format!(
                        "[freeze:contract][recipe] verifier_saw_unsupported_item: ctx={context}"
                    ));
                }
            }

            if body.get_ref(*if_stmt).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] if_stmt_ref_out_of_range: idx={} ctx={context}",
                    if_stmt.index()
                ));
            }
            if let Some(node) = body.get_ref(*if_stmt) {
                if !matches!(node, ASTNode::If { .. }) {
                    return Err(format!(
                        "[freeze:contract][recipe] if_stmt_is_not_if_node: idx={} ctx={context}",
                        if_stmt.index()
                    ));
                }
            }
            let _ = then_block;
            let _ = else_block;
            Ok(())
        }
        RecipeItem::LoopV0 {
            loop_stmt,
            cond_view: _cond_view,
            body_block: _body_block,
            body_contract: _body_contract,
            kind: _kind,
            features: _features,
        } => {
            if body.get_ref(*loop_stmt).is_none() {
                return Err(format!(
                    "[freeze:contract][recipe] stmt_ref_out_of_range: idx={} ctx={context}",
                    loop_stmt.index()
                ));
            }
            if let Some(node) = body.get_ref(*loop_stmt) {
                if !matches!(node, ASTNode::Loop { .. }) {
                    return Err(format!(
                        "[freeze:contract][recipe] loop_stmt_is_not_loop_node: idx={} ctx={context}",
                        loop_stmt.index()
                    ));
                }
            }
            Ok(())
        }
        _ => Err(format!(
            "[freeze:contract][recipe] verifier_saw_unsupported_item: ctx={context}"
        )),
    }
}
