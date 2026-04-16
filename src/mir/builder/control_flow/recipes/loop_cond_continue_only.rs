//! Recipe surface for loop_cond_continue_only (recipes-owned surface).

use crate::mir::builder::control_flow::plan::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::recipes::refs::{StmtRef, StmtSpan};
use crate::mir::builder::control_flow::recipes::RecipeBody;

pub(in crate::mir::builder) type ContinueOnlyRecipe = LoopCondRecipe<ContinueOnlyStmtRecipe>;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum ContinueOnlyStmtRecipe {
    /// A normal statement (lower_stmt_ast).
    Stmt(StmtRef),
    /// `if <cond> { <prelude>; continue }` (else無し)
    ContinueIf {
        if_stmt: StmtRef,
        /// Prelude span inside the if-body (tail-continue excluded).
        prelude_span: StmtSpan,
    },
    /// `if <cond> { <prelude>; continue }` where prelude contains nested continue-if/group-if.
    ContinueIfGroupPrelude {
        if_stmt: StmtRef,
        prelude_span: StmtSpan,
        prelude_items: Vec<ContinueOnlyStmtRecipe>,
    },
    /// A grouping `if` that contains nested ContinueIf recipes.
    ///
    /// Fallthrough mutation is forbidden (no joins here) to keep this lego minimal.
    GroupIf {
        if_stmt: StmtRef,
        then_body: ContinueOnlyRecipe,
        else_body: Option<ContinueOnlyRecipe>,
    },
    /// `if <cond> { <inner-loop-prelude>; loop(k < N) { ... }; <inner-loop-postlude>; continue }`
    /// Phase 29bq: _decode_escapes/1 blocker pattern.
    ContinueIfNestedLoop {
        if_stmt: StmtRef,
        inner_loop_prelude_span: StmtSpan,
        inner_loop_prelude_items: Vec<ContinueOnlyStmtRecipe>,
        inner_loop_body: RecipeBody,
        inner_loop_stmt: StmtRef,
        inner_loop_postlude_span: StmtSpan,
        inner_loop_postlude_items: Vec<ContinueOnlyStmtRecipe>,
    },
}
