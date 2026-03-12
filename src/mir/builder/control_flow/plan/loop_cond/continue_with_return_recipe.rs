//! LoopCondContinueWithReturn recipe (shape-only refs).

use crate::mir::builder::control_flow::plan::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::plan::recipes::refs::{StmtRef, StmtSpan};

pub(in crate::mir::builder) type ContinueWithReturnRecipe = LoopCondRecipe<ContinueWithReturnItem>;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum ContinueWithReturnItem {
    /// A normal statement (assignment/local/call/print/return/etc.).
    Stmt(StmtRef),
    /// `if <cond> { <prelude>; continue }` (else無し)
    ContinueIf {
        if_stmt: StmtRef,
        prelude_span: StmtSpan,
        prelude_items: Vec<ContinueWithReturnItem>,
    },
    /// Fixture-derived 1-shape: if-else-if chain with nested return in else branch.
    HeteroReturnIf { if_stmt: StmtRef },
    /// Any other if accepted by facts (lowered in pipeline ordering).
    IfAny(StmtRef),
}
