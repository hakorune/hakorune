//! Phase 12: Unified normalizer for nested loop depth1.
//!
//! This module provides a single entry point for lowering all nested loop depth1
//! patterns, replacing 4 separate normalizer modules.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1_preheader::apply_nested_loop_preheader_freshness;
use crate::mir::builder::control_flow::plan::nested_loop_depth1::facts::{
    try_extract_nested_loop_depth1_facts, NestedLoopDepth1Facts, NestedLoopDepth1Kind,
};
use crate::mir::builder::control_flow::plan::nested_loop_plan::lower_nested_loop_plan_with_recipe_first;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use std::cell::Cell;

thread_local! {
    static STMT_ONLY_FASTPATH_DEPTH: Cell<u32> = Cell::new(0);
}

struct StmtOnlyFastpathGuard {
    prev: u32,
}

impl StmtOnlyFastpathGuard {
    fn enter_if_outermost() -> Option<Self> {
        STMT_ONLY_FASTPATH_DEPTH.with(|depth| {
            let prev = depth.get();
            if prev != 0 {
                return None;
            }
            depth.set(prev + 1);
            Some(Self { prev })
        })
    }
}

impl Drop for StmtOnlyFastpathGuard {
    fn drop(&mut self) {
        STMT_ONLY_FASTPATH_DEPTH.with(|depth| depth.set(self.prev));
    }
}

/// Try to lower a nested loop using the unified nested_loop_depth1 pattern.
///
/// This function tries all 4 nested loop depth1 kinds in priority order
/// and returns the lowered plan for the first match.
pub(in crate::mir::builder) fn try_lower_nested_loop_depth1(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    error_prefix: &str,
) -> Result<Option<LoweredRecipe>, String> {
    let Some(facts) = try_extract_nested_loop_depth1_facts(condition, body) else {
        return Ok(None);
    };

    // BreakContinuePure has an additional empty body check
    if facts.kind == NestedLoopDepth1Kind::BreakContinuePure && facts.body.is_empty() {
        return Ok(None);
    }

    if let Some(body_recipe) = facts.body_stmt_only.as_ref() {
        if let Some(_guard) = StmtOnlyFastpathGuard::enter_if_outermost() {
            let cond_view = CondBlockView::from_expr(&facts.condition);
            if let Ok(plan) = parts::entry::lower_nested_loop_depth1_stmt_only(
                builder,
                &cond_view,
                body_recipe,
                error_prefix,
            ) {
                return Ok(Some(plan));
            }
        }
    }

    let plan = lower_nested_loop_depth1_from_facts(builder, facts, error_prefix)?;
    Ok(Some(apply_nested_loop_preheader_freshness(builder, plan)))
}

fn lower_nested_loop_depth1_from_facts(
    builder: &mut MirBuilder,
    facts: NestedLoopDepth1Facts,
    error_prefix: &str,
) -> Result<LoweredRecipe, String> {
    let ctx = LoopRouteContext::new(
        &facts.condition,
        &facts.body,
        facts.kind.context_name(),
        false,
        false,
    );
    lower_nested_loop_plan_with_recipe_first(
        builder,
        &facts.condition,
        &facts.body.body,
        &ctx,
        error_prefix,
        "nested_loop_depth1",
    )
}
