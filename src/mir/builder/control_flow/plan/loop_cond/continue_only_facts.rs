//! Phase 29bq P2.x: LoopCondContinueOnlyFacts (Facts SSOT)

use super::continue_only_recipe::{ContinueOnlyRecipe, ContinueOnlyStmtRecipe};
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::loop_cond_shared::{
    branch_tail_is_continue, LoopCondRecipe,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipes::refs::{StmtRef, StmtSpan};
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondContinueOnlyFacts {
    pub condition: ASTNode,
    pub recipe: ContinueOnlyRecipe,
}

pub(in crate::mir::builder) fn try_extract_loop_cond_continue_only_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondContinueOnlyFacts>, Freeze> {
    if !super::loop_cond_unified_helpers::entry_gate_ok() {
        return Ok(None);
    }
    let debug = super::loop_cond_unified_helpers::debug_enabled();

    if !super::loop_cond_unified_helpers::validate_loop_condition(condition) {
        return Ok(None);
    }

    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    if counts.break_count > 0 {
        log_reject(
            "loop_cond_continue_only",
            RejectReason::BreakPresent,
            handoff_tables::for_loop_cond_continue_only,
        );
        return Ok(None);
    }
    // Phase 29bq: Allow nested loops only inside If body (ContinueIfNestedLoop pattern).
    // Top-level nested loops are still rejected.
    if has_top_level_nested_loop(body) {
        log_reject(
            "loop_cond_continue_only",
            RejectReason::TopLevelNestedLoop,
            handoff_tables::for_loop_cond_continue_only,
        );
        return Ok(None);
    }
    if counts.continue_count == 0 {
        log_reject(
            "loop_cond_continue_only",
            RejectReason::NoContinue,
            handoff_tables::for_loop_cond_continue_only,
        );
        return Ok(None);
    }

    // Reject loops with return statements (continue-only invariant)
    if counts.return_count > 0 {
        log_reject(
            "loop_cond_continue_only",
            RejectReason::ReturnInBody,
            handoff_tables::for_loop_cond_continue_only,
        );
        return Ok(None);
    }

    // Else付きcontinue-ifはこの箱の受理範囲外。
    if has_continue_in_if_with_else(body) {
        log_reject(
            "loop_cond_continue_only",
            RejectReason::ContinueInIfWithElse,
            handoff_tables::for_loop_cond_continue_only,
        );
        return Ok(None);
    }

    // Build recipe and validate: continue must be inside if branches only.
    let mut saw_continue_if = false;
    let recipe_body = RecipeBody::new(body.to_vec());
    let mut items = Vec::with_capacity(recipe_body.body.len());
    for (idx, stmt) in recipe_body.body.iter().enumerate() {
        if matches!(
            stmt,
            ASTNode::Continue { .. } | ASTNode::Break { .. } | ASTNode::Return { .. }
        ) {
            return Err(Freeze::contract(format!(
                "loop_cond_continue_only: top-level exit stmt at idx={} kind={}",
                idx,
                stmt.node_type()
            )));
        }
        let recipe = build_stmt_recipe(stmt, idx, &mut saw_continue_if, debug)?;
        items.push(recipe);
    }

    if !saw_continue_if {
        // Continue exists, but not in the supported “tail-continue if” form.
        return Err(Freeze::contract(
            "loop_cond_continue_only: continue-only loop detected, but no supported continue-if sites",
        ));
    }

    Ok(Some(LoopCondContinueOnlyFacts {
        condition: condition.clone(),
        recipe: LoopCondRecipe::new(recipe_body.body, items),
    }))
}

/// Phase 29bq: Check for top-level nested loops only.
/// If body内のnested loopは許可（ContinueIfNestedLoopで処理）。
fn has_top_level_nested_loop(body: &[ASTNode]) -> bool {
    for stmt in body {
        match stmt {
            ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {
                return true;
            }
            _ => {}
        }
    }
    false
}

fn has_continue_in_if_with_else(body: &[ASTNode]) -> bool {
    fn scan(stmt: &ASTNode) -> bool {
        match stmt {
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                // Detect only the direct "continue-if with else" shape; nested continues are handled by this box.
                let then_tail = branch_tail_is_continue(then_body);
                let else_tail = else_body
                    .as_ref()
                    .is_some_and(|b| branch_tail_is_continue(b));
                if else_body.is_some() && (then_tail || else_tail) {
                    return true;
                }
                then_body.iter().any(scan) || else_body.as_ref().is_some_and(|b| b.iter().any(scan))
            }
            ASTNode::Loop { body, .. }
            | ASTNode::While { body, .. }
            | ASTNode::ForRange { body, .. } => body.iter().any(scan),
            ASTNode::ScopeBox { body, .. } => body.iter().any(scan),
            _ => false,
        }
    }

    body.iter().any(scan)
}

fn build_stmt_recipe(
    stmt: &ASTNode,
    idx: usize,
    saw_continue_if: &mut bool,
    debug: bool,
) -> Result<ContinueOnlyStmtRecipe, Freeze> {
    let if_ref = StmtRef::new(idx);
    match stmt {
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            if branch_tail_is_continue(then_body) && else_body.is_none() {
                *saw_continue_if = true;
                let prelude = if then_body.len() > 1 {
                    &then_body[..then_body.len() - 1]
                } else {
                    &[]
                };
                if !continue_prelude_is_allowed(prelude) {
                    let counts =
                        super::loop_cond_unified_helpers::count_control_flow_with_returns(prelude);
                    if counts.break_count > 0 {
                        return Err(Freeze::contract(
                            "loop_cond_continue_only: continue-if prelude contains break",
                        ));
                    }
                    // Phase 29bq: Check for nested loop in prelude (ContinueIfNestedLoop pattern)
                    if counts.has_nested_loop {
                        if let Some(nested_loop_recipe) =
                            try_extract_continue_if_nested_loop(if_ref, prelude, debug)
                        {
                            return Ok(nested_loop_recipe);
                        }
                        return Err(Freeze::contract(
                            "loop_cond_continue_only: continue-if prelude contains unsupported nested loop",
                        ));
                    }
                    if prelude_has_illegal_return(prelude) {
                        return Err(Freeze::contract(
                            "loop_cond_continue_only: continue-if prelude contains top-level return",
                        ));
                    }
                    let prelude_items = build_group_body_items(prelude, saw_continue_if, debug)?;
                    return Ok(ContinueOnlyStmtRecipe::ContinueIfGroupPrelude {
                        if_stmt: if_ref,
                        prelude_span: StmtSpan::new(0, prelude.len()),
                        prelude_items,
                    });
                }
                return Ok(ContinueOnlyStmtRecipe::ContinueIf {
                    if_stmt: if_ref,
                    prelude_span: StmtSpan::new(0, prelude.len()),
                });
            }

            // Reject if-else with heterogeneous exit types (return mixed with continue)
            // This is outside the "continue-only" boundary and should be handled by loop_cond_break_continue
            let then_has_return = contains_return_in_branch(then_body);
            let else_has_return = else_body
                .as_ref()
                .map_or(false, |b| contains_return_in_branch(b));

            if (then_has_return && !else_has_return)
                || (!then_has_return && else_has_return)
                || (then_has_return && else_has_return)
            {
                // Heterogeneous exit types - not continue-only
                if then_has_return && !else_has_return || !then_has_return && else_has_return {
                    log_reject(
                        "loop_cond_continue_only",
                        RejectReason::ReturnInBranch,
                        handoff_tables::for_loop_cond_continue_only,
                    );
                } else {
                    log_reject(
                        "loop_cond_continue_only",
                        RejectReason::ReturnInBothBranches,
                        handoff_tables::for_loop_cond_continue_only,
                    );
                }
                // Fall through to Stmt (generic statement) which will be rejected elsewhere
                // Don't return Ok(None) here - let the rest of the system handle it
            }

            let then_has_continue = contains_continue_in_branch(then_body);
            let else_has_continue = else_body
                .as_ref()
                .map_or(false, |b| contains_continue_in_branch(b));
            if then_has_continue || else_has_continue {
                if debug {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0
                        .log
                        .debug(&format!("[loop_cond_continue_only] group_if detected"));
                }
                let then_recipes = build_group_body_recipe(then_body, saw_continue_if, debug)?;
                let else_recipes = match else_body.as_ref() {
                    Some(b) => Some(build_group_body_recipe(b, saw_continue_if, debug)?),
                    None => None,
                };
                if !group_if_fallthrough_is_allowed(&then_recipes)
                    || else_recipes
                        .as_ref()
                        .is_some_and(|b| !group_if_fallthrough_is_allowed(b))
                {
                    return Err(Freeze::contract(
                        "loop_cond_continue_only: group-if fallthrough mutation is out-of-scope",
                    ));
                }
                return Ok(ContinueOnlyStmtRecipe::GroupIf {
                    if_stmt: if_ref,
                    then_body: then_recipes,
                    else_body: else_recipes,
                });
            }

            Ok(ContinueOnlyStmtRecipe::Stmt(if_ref))
        }
        _ => Ok(ContinueOnlyStmtRecipe::Stmt(StmtRef::new(idx))),
    }
}

fn build_simple_block_items(body: &[ASTNode]) -> Vec<ContinueOnlyStmtRecipe> {
    (0..body.len())
        .map(|idx| ContinueOnlyStmtRecipe::Stmt(StmtRef::new(idx)))
        .collect()
}

fn build_group_body_recipe(
    body: &[ASTNode],
    saw_continue_if: &mut bool,
    debug: bool,
) -> Result<ContinueOnlyRecipe, Freeze> {
    let recipe_body = body.to_vec();
    let items = build_group_body_items(&recipe_body, saw_continue_if, debug)?;
    Ok(LoopCondRecipe::new(recipe_body, items))
}

fn build_group_body_items(
    body: &[ASTNode],
    saw_continue_if: &mut bool,
    debug: bool,
) -> Result<Vec<ContinueOnlyStmtRecipe>, Freeze> {
    let mut items = Vec::with_capacity(body.len());
    for (idx, stmt) in body.iter().enumerate() {
        if matches!(stmt, ASTNode::Break { .. } | ASTNode::Return { .. }) {
            return Err(Freeze::contract(
                "loop_cond_continue_only: break/return inside group-if is forbidden",
            ));
        }
        if matches!(stmt, ASTNode::Continue { .. }) {
            return Err(Freeze::contract(
                "loop_cond_continue_only: continue must be inside a tail-continue if",
            ));
        }
        items.push(build_stmt_recipe(stmt, idx, saw_continue_if, debug)?);
    }
    Ok(items)
}

fn contains_continue_in_branch(body: &[ASTNode]) -> bool {
    count_control_flow(body, ControlFlowDetector::default()).continue_count > 0
}

/// Check if a branch (then/else body) contains return statements.
fn contains_return_in_branch(body: &[ASTNode]) -> bool {
    super::loop_cond_unified_helpers::count_control_flow_with_returns(body).return_count > 0
}

fn continue_prelude_is_allowed(body: &[ASTNode]) -> bool {
    let counts = super::loop_cond_unified_helpers::count_control_flow_with_returns(body);
    !counts.has_nested_loop
        && counts.break_count == 0
        && counts.continue_count == 0
        && counts.return_count == 0
}

fn prelude_has_illegal_return(body: &[ASTNode]) -> bool {
    fn scan(stmt: &ASTNode, in_if: bool) -> bool {
        match stmt {
            ASTNode::Return { .. } => !in_if,
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                then_body.iter().any(|s| scan(s, true))
                    || else_body
                        .as_ref()
                        .is_some_and(|b| b.iter().any(|s| scan(s, true)))
            }
            ASTNode::ScopeBox { body, .. } => body.iter().any(|s| scan(s, in_if)),
            ASTNode::Loop { body, .. }
            | ASTNode::While { body, .. }
            | ASTNode::ForRange { body, .. } => body.iter().any(|s| scan(s, in_if)),
            _ => false,
        }
    }

    body.iter().any(|stmt| scan(stmt, false))
}

fn group_if_fallthrough_is_allowed(body: &ContinueOnlyRecipe) -> bool {
    // Minimal contract: group-if may only contain locals, nested ifs, and nested loops;
    // fallthrough assignments/calls are out-of-scope.
    // (Assignments inside ContinueIf prelude are allowed.)
    // Phase 29bq: Allow Loop/While in group-if body - these run to completion without
    // affecting outer loop control flow (e.g., hex-parsing loop in _decode_escapes).
    for stmt in &body.items {
        match stmt {
            ContinueOnlyStmtRecipe::ContinueIf { .. } => {}
            ContinueOnlyStmtRecipe::ContinueIfGroupPrelude { .. } => {}
            ContinueOnlyStmtRecipe::ContinueIfNestedLoop { .. } => {}
            ContinueOnlyStmtRecipe::GroupIf { .. } => {}
            ContinueOnlyStmtRecipe::Stmt(node) => {
                let Some(stmt) = body.body.get_ref(*node) else {
                    return false;
                };
                match stmt {
                    ASTNode::Local { .. }
                    | ASTNode::If { .. }
                    | ASTNode::Loop { .. }
                    | ASTNode::While { .. }
                    | ASTNode::ForRange { .. } => {}
                    _ => return false,
                }
            }
        }
    }
    true
}

/// Phase 29bq: Extract ContinueIfNestedLoop from a continue-if prelude that contains exactly one nested loop.
/// Pattern: `if <outer_cond> { <inner_prelude>; loop(...) { ... }; <inner_postlude>; continue }`
fn try_extract_continue_if_nested_loop(
    if_ref: StmtRef,
    prelude: &[ASTNode],
    debug: bool,
) -> Option<ContinueOnlyStmtRecipe> {
    // Find exactly one loop in prelude
    let mut loop_idx = None;
    for (idx, stmt) in prelude.iter().enumerate() {
        match stmt {
            ASTNode::Loop { .. } | ASTNode::While { .. } | ASTNode::ForRange { .. } => {
                if loop_idx.is_some() {
                    // Multiple loops - not supported
                    log_reject(
                        "loop_cond_continue_only",
                        RejectReason::MultipleNestedLoopsInContinueIfPrelude,
                        handoff_tables::for_loop_cond_continue_only,
                    );
                    return None;
                }
                loop_idx = Some(idx);
            }
            _ => {}
        }
    }

    let loop_idx = loop_idx?;
    let inner_loop = prelude[loop_idx].clone();
    let inner_loop_prelude = &prelude[..loop_idx];
    let inner_loop_postlude = &prelude[loop_idx + 1..];

    // Validate inner_loop_prelude/postlude don't contain control flow
    for stmt in inner_loop_prelude.iter().chain(inner_loop_postlude.iter()) {
        match stmt {
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {
                log_reject(
                    "loop_cond_continue_only",
                    RejectReason::ControlFlowInNestedLoopPreludePostlude,
                    handoff_tables::for_loop_cond_continue_only,
                );
                return None;
            }
            _ => {}
        }
    }

    if debug {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[loop_cond_continue_only] ContinueIfNestedLoop detected: prelude={}, postlude={}",
            inner_loop_prelude.len(),
            inner_loop_postlude.len()
        ));
    }

    Some(ContinueOnlyStmtRecipe::ContinueIfNestedLoop {
        if_stmt: if_ref,
        inner_loop_prelude_span: StmtSpan::new(0, loop_idx),
        inner_loop_prelude_items: build_simple_block_items(inner_loop_prelude),
        inner_loop_body: RecipeBody::new(vec![inner_loop]),
        inner_loop_stmt: StmtRef::new(0),
        inner_loop_postlude_span: StmtSpan::new(loop_idx + 1, prelude.len()),
        inner_loop_postlude_items: build_simple_block_items(inner_loop_postlude),
    })
}
