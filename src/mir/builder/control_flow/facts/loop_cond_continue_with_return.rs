//! Phase 29bq P2.x: LoopCondContinueWithReturnFacts (Facts SSOT)
//!
//! Minimal sibling box for continue-only loops with nested return.
//! This is a fixture-derived 1-shape box (BoxCount approach).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::loop_cond::loop_cond_unified_helpers;
use crate::mir::builder::control_flow::plan::loop_cond_shared::{
    branch_tail_is_continue, LoopCondRecipe,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::recipes::loop_cond_continue_with_return::{
    ContinueWithReturnItem, ContinueWithReturnRecipe,
};
use crate::mir::builder::control_flow::recipes::refs::{StmtRef, StmtSpan};
use crate::mir::builder::control_flow::recipes::RecipeBody;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondContinueWithReturnFacts {
    pub condition: ASTNode,
    pub recipe: ContinueWithReturnRecipe,
}

pub(in crate::mir::builder) fn try_extract_loop_cond_continue_with_return_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondContinueWithReturnFacts>, Freeze> {
    if !loop_cond_unified_helpers::entry_gate_ok() {
        return Ok(None);
    }
    let debug = loop_cond_unified_helpers::debug_enabled();

    if !loop_cond_unified_helpers::validate_loop_condition(condition) {
        return Ok(None);
    }

    // Independent observation: continue>0 && break==0 && return>0
    // (avoid box order dependency)
    let counts = loop_cond_unified_helpers::count_control_flow_with_returns(body);

    if counts.continue_count == 0 {
        return Ok(None);
    }
    if counts.break_count > 0 {
        return Ok(None);
    }
    if counts.return_count == 0 {
        return Ok(None);
    }
    if counts.has_nested_loop {
        return Ok(None);
    }

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!("[loop_cond_continue_with_return] MATCHED: continue-only with return (fixture-derived 1-shape)"));
    }

    let recipe = build_block_recipe(body, debug)?;

    Ok(Some(LoopCondContinueWithReturnFacts {
        condition: condition.clone(),
        recipe,
    }))
}

fn build_block_recipe(body: &[ASTNode], debug: bool) -> Result<ContinueWithReturnRecipe, Freeze> {
    let recipe_body = RecipeBody::new(body.to_vec());
    let items = build_block_items(&recipe_body.body, debug)?;
    Ok(LoopCondRecipe::new(recipe_body.body, items))
}

fn build_block_items(body: &[ASTNode], debug: bool) -> Result<Vec<ContinueWithReturnItem>, Freeze> {
    let mut items = Vec::with_capacity(body.len());
    for (idx, stmt) in body.iter().enumerate() {
        let item = build_stmt_recipe(stmt, idx, debug)?;
        items.push(item);
    }
    Ok(items)
}

fn build_stmt_recipe(
    stmt: &ASTNode,
    idx: usize,
    debug: bool,
) -> Result<ContinueWithReturnItem, Freeze> {
    match stmt {
        ASTNode::If {
            condition: _condition,
            then_body,
            else_body,
            ..
        } => {
            // Check for continue-if pattern (tail continue in then, no else)
            if branch_tail_is_continue(then_body) && else_body.is_none() {
                let prelude_len = then_body.len().saturating_sub(1);
                let prelude = &then_body[..prelude_len];
                let prelude_items = build_block_items(prelude, debug)?;
                return Ok(ContinueWithReturnItem::ContinueIf {
                    if_stmt: StmtRef::new(idx),
                    prelude_span: StmtSpan::new(0, prelude_len),
                    prelude_items,
                });
            }

            // Check for fixture-derived 1-shape: if-else-if chain with nested return
            if let Some(else_body) = else_body {
                if debug {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[loop_cond_continue_with_return] checking hetero: then_body.len={}, else_body.len={}", then_body.len(), else_body.len()));
                }
                if is_hetero_return_if_shape(then_body, else_body, debug)? {
                    return Ok(ContinueWithReturnItem::HeteroReturnIf {
                        if_stmt: StmtRef::new(idx),
                    });
                }
            }

            Ok(ContinueWithReturnItem::IfAny(StmtRef::new(idx)))
        }
        _ => Ok(ContinueWithReturnItem::Stmt(StmtRef::new(idx))),
    }
}

/// Check if this matches the fixture-derived shape:
/// - then: single assignment (e.g., in_str = 1)
/// - else: if-else-if chain with nested return (depth <= 3)
fn is_hetero_return_if_shape(
    then_body: &[ASTNode],
    else_body: &[ASTNode],
    debug: bool,
) -> Result<bool, Freeze> {
    // then must be single statement
    if then_body.len() != 1 {
        return Ok(false);
    }

    // Check for return in else body up to depth 3
    // Depth is counted from 0 at the outer else-body.
    let depth = find_return_in_else_chain_depth(else_body, 0, debug);

    if depth == 0 {
        return Ok(false);
    }

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[loop_cond_continue_with_return] detected hetero-return-if shape (depth={})",
            depth
        ));
    }

    Ok(true)
}

/// Find return statement in if-else chain up to max_depth.
/// Returns the depth where return was found (0 if not found).
///
/// Depth counting:
/// - Depth 1: Top-level else_body statements
/// - Depth 2: First level of nested if (either then_body or else_body)
/// - Depth 3: Second level of nested if
/// - Depth 4: Third level of nested if (for return inside then_body of depth 3 if)
fn find_return_in_else_chain_depth(body: &[ASTNode], current_depth: usize, debug: bool) -> usize {
    const MAX_DEPTH: usize = 3;

    for stmt in body {
        match stmt {
            ASTNode::Return { .. } => {
                if current_depth > MAX_DEPTH {
                    return 0;
                }
                if debug {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[loop_cond_continue_with_return] found Return at depth={}",
                        current_depth
                    ));
                }
                return current_depth;
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if debug {
                    let ring0 = crate::runtime::get_global_ring0();
                    let else_len = else_body.as_ref().map(|b| b.len()).unwrap_or(0);
                    ring0.log.debug(&format!("[loop_cond_continue_with_return] checking If at depth={}, then_body.len={}, else_body.len={}", current_depth, then_body.len(), else_len));
                }
                // Check then_body (for patterns like: if cond { ...; return })
                // Only recurse if we haven't exceeded MAX_DEPTH for the next level
                if current_depth < MAX_DEPTH {
                    let found_in_then =
                        find_return_in_else_chain_depth(then_body, current_depth + 1, debug);
                    if found_in_then > 0 {
                        return found_in_then;
                    }
                } else {
                    // At MAX_DEPTH, check then_body directly without further recursion
                    for then_stmt in then_body {
                        if matches!(then_stmt, ASTNode::Return { .. }) {
                            if debug {
                                let ring0 = crate::runtime::get_global_ring0();
                                ring0.log.debug(&format!("[loop_cond_continue_with_return] found Return in then_body at depth={}", current_depth + 1));
                            }
                            return current_depth + 1;
                        }
                    }
                }
                // Recurse into else body for else-if chains
                if let Some(else_b) = else_body {
                    if current_depth < MAX_DEPTH {
                        let found =
                            find_return_in_else_chain_depth(else_b, current_depth + 1, debug);
                        if found > 0 {
                            return found;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    0
}
