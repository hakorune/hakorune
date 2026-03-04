//! Shared scan-loop segment vocabulary (SSOT).
//!
//! Purpose:
//! - Keep "scan系は線形=RecipeBlock / nested=LoopV0(or planner fallback)" の語彙を1本化する。
//! - scan 系の recipe/facts/pipeline 間で型の重複を避け、drift を防ぐ。
//!
//! Notes:
//! - This module is *vocab-only*. It does not define acceptance policies.
//! - `TLinear` is the linear segment payload type (e.g. `NoExitBlockRecipe` or `ExitAllowedBlockRecipe`).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::stmt_view::StmtOnlyBlockRecipe;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct NestedLoopRecipe {
    pub cond_view: CondBlockView,
    pub loop_stmt: ASTNode,
    pub body: RecipeBody,
    pub body_stmt_only: Option<StmtOnlyBlockRecipe>,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum LoopScanSegment<TLinear> {
    Linear(TLinear),
    NestedLoop(NestedLoopRecipe),
}

// Prefer these aliases at call sites to avoid reintroducing local "segment vocab" bags.
pub(in crate::mir::builder) type ScanNestedLoopRecipe = NestedLoopRecipe;
pub(in crate::mir::builder) type ScanSegment<TLinear> = LoopScanSegment<TLinear>;
