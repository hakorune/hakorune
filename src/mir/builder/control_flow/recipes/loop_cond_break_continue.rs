//! Recipe surface for loop_cond_break_continue (recipes-owned surface).
//!
//! The route-specific builders stay in the loop_cond family, but the recipe
//! vocabulary is owned here so non-family callers can depend on a top-level
//! recipes surface.

use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::facts::stmt_view::StmtOnlyBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::{
    ExitAllowedBlockRecipe, ExitOnlyBlockRecipe,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfMode};
use crate::mir::builder::control_flow::recipes::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::recipes::refs::StmtRef;

pub(in crate::mir::builder) type LoopCondBreakContinueRecipe =
    LoopCondRecipe<LoopCondBreakContinueItem>;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum LoopCondBreakContinueItem {
    Stmt(StmtRef),
    ProgramBlock {
        stmt: StmtRef,
        stmt_only: Option<StmtOnlyBlockRecipe>,
    },
    ExitIf {
        if_stmt: StmtRef,
        block: Option<ExitAllowedBlockRecipe>,
    },
    ContinueIfWithElse {
        if_stmt: StmtRef,
        continue_in_then: bool,
        continue_prelude: Option<NoExitBlockRecipe>,
        fallthrough_body: Option<NoExitBlockRecipe>,
    },
    ConditionalUpdateIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_body: Option<NoExitBlockRecipe>,
        then_exit: Option<ExitKind>,
        else_body: Option<NoExitBlockRecipe>,
        else_exit: Option<ExitKind>,
    },
    GeneralIf(NoExitBlockRecipe),
    TailBreak {
        block: Option<ExitAllowedBlockRecipe>,
    },
    ElseOnlyReturnIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_no_exit: Option<NoExitBlockRecipe>,
        else_return_stmt: StmtRef,
    },
    ThenOnlyReturnIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_return_stmt: StmtRef,
        else_no_exit: Option<NoExitBlockRecipe>,
    },
    ElseOnlyBreakIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_no_exit: Option<NoExitBlockRecipe>,
        else_break_stmt: StmtRef,
    },
    ThenOnlyBreakIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_break_stmt: StmtRef,
        else_no_exit: Option<NoExitBlockRecipe>,
    },
    ElseGuardBreakIf {
        if_stmt: StmtRef,
        then_no_exit: Option<NoExitBlockRecipe>,
        else_exit_allowed: Option<ExitAllowedBlockRecipe>,
        then_body: LoopCondBreakContinueRecipe,
        else_body: LoopCondBreakContinueRecipe,
    },
    // Phase 291x-126: recursive exit-tree vocabulary, matched before construction is widened.
    ExitLeaf {
        kind: ExitKind,
        stmt: StmtRef,
    },
    ExitIfTree {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        mode: IfMode,
        then_body: ExitOnlyBlockRecipe,
        else_body: Option<ExitOnlyBlockRecipe>,
    },
}

impl LoopCondBreakContinueItem {
    pub fn exit_if_with_optional_block(
        if_stmt: StmtRef,
        exit_allowed_block: Option<ExitAllowedBlockRecipe>,
    ) -> Self {
        Self::ExitIf {
            if_stmt,
            block: exit_allowed_block,
        }
    }

    pub fn tail_break_with_optional_block(
        exit_allowed_block: Option<ExitAllowedBlockRecipe>,
    ) -> Self {
        Self::TailBreak {
            block: exit_allowed_block,
        }
    }

    pub fn else_guard_break_if_with_optional_else_exit_allowed(
        if_stmt: StmtRef,
        then_no_exit: Option<NoExitBlockRecipe>,
        then_recipe: LoopCondBreakContinueRecipe,
        else_recipe: LoopCondBreakContinueRecipe,
        else_exit_allowed: Option<ExitAllowedBlockRecipe>,
    ) -> Self {
        Self::ElseGuardBreakIf {
            if_stmt,
            then_no_exit,
            else_exit_allowed,
            then_body: then_recipe,
            else_body: else_recipe,
        }
    }
}
