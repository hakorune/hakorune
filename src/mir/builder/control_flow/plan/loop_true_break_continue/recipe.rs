//! Recipe for loop(true) break/continue lowering
//!
//! Recipe SSOT: Facts → Recipe → Pipeline (no re-validation)
//!
//! Classification: Stmt = simple (no If/Loop), ProgramGeneralBlock = Program(general-if), GeneralIf = carrier-only (no exit)
//!
//! - Stmt: simple statements only (Assignment/Local/MethodCall/FunctionCall/Print)
//! - ProgramGeneralBlock: Program stmt with "general-if body" only (no loops/exits)
//! - ExitIf: if blocks ending with break/continue/return(value)
//! - IfTailExitPair: if { ... exit } + tail exit pattern
//! - NestedLoopDepth1: nested loop(cond) or loop(true) with depth=1
//! - GeneralIf: carrier update only (no exit)

use crate::mir::builder::control_flow::plan::recipes::refs::{StmtPair, StmtRef};
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use crate::mir::builder::control_flow::recipes::loop_cond_shared::LoopCondRecipe;

/// Recipe は body を所有し、items は Idx で参照
pub(in crate::mir::builder) type LoopTrueBreakContinueRecipe = LoopCondRecipe<LoopTrueItem>;

/// Else-side item for GeneralIfElseExit pattern
#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum ElseItem {
    /// Exit-if: if block ending with break/continue/return(value)
    ExitIf(StmtRef),
    /// Prelude statement: Assignment/Local/MethodCall/FunctionCall/Print
    PreludeStmt(StmtRef),
}

/// Else-side recipe for exit-mixed pattern (exit-if + preludes)
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ElseExitMixedRecipe {
    /// Else body statements (RecipeBody for get_ref() access)
    pub else_body: RecipeBody,
    /// Classified items (ExitIf or PreludeStmt)
    /// Index refers to position in else_body (0..else_body.len())
    pub items: Vec<ElseItem>,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum LoopTrueItem {
    /// 単純stmt のみ（Assignment/Local/MethodCall/FunctionCall/Print）
    /// If/Loop は含まない（recipe 側で保証）
    Stmt(StmtRef),
    /// Program stmt（ブロック）。中身は general-if body のみ（no loop / no exit）
    ProgramGeneralBlock(StmtRef),
    /// exit_if_map を呼ぶ（body 内の位置情報のみ）
    ExitIf(StmtRef),
    /// 既存の "if + tail exit" ペア（if の then は exit で終わり、次の stmt が対の exit）
    IfTailExitPair(StmtPair),
    /// ネストループ（body 内の位置情報のみ）
    NestedLoopDepth1(StmtRef),
    /// general if（carrier 更新のみ、exit なし）
    GeneralIf(StmtRef),
    /// General-if with exit-if in else branch.
    /// Then-side: general-if body (no exit, local/assign/nested-if only)
    /// Else-side: exit-mixed (exit-if + preludes, stored in recipe)
    GeneralIfElseExit {
        if_ref: StmtRef,
        else_recipe: ElseExitMixedRecipe,
    },
    /// Unconditional tail return inside loop body (must be the final stmt).
    TailReturn(StmtRef),
}
