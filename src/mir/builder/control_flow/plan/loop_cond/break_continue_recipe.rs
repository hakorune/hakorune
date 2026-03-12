//! LoopCondBreakContinue recipe (shape-only refs).

use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitOnlyBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::facts::stmt_view::StmtOnlyBlockRecipe;
use crate::mir::builder::control_flow::plan::loop_cond_shared::LoopCondRecipe;
use crate::mir::builder::control_flow::plan::recipe_tree::common::{ExitKind, IfMode};
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;

pub(in crate::mir::builder) type LoopCondBreakContinueRecipe =
    LoopCondRecipe<LoopCondBreakContinueItem>;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct NestedLoopDepth1Recipe {
    pub cond_view: CondBlockView,
    pub body: Option<StmtOnlyBlockRecipe>,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum LoopCondBreakContinueItem {
    /// A normal statement (assignment/local/call/print).
    Stmt(StmtRef),
    /// A program/scope block inside loop body.
    ///
    /// - `stmt_only`: pre-built "stmt-only effects" payload when available (fast path).
    /// - When `stmt_only` is `None`, lowering must fall back to lowering the underlying
    ///   `ASTNode::{Program,ScopeBox}` statement (still under the "no-exit from block" contract).
    ProgramBlock {
        stmt: StmtRef,
        stmt_only: Option<StmtOnlyBlockRecipe>,
    },
    /// Exit-if (break/continue/return) accepted by facts.
    ExitIf {
        if_stmt: StmtRef,
        block: Option<ExitAllowedBlockRecipe>,
    },
    /// if-else with a tail-continue branch and fallthrough branch (recipe-first).
    ContinueIfWithElse {
        if_stmt: StmtRef,
        continue_in_then: bool,
        continue_prelude: Option<NoExitBlockRecipe>,
        fallthrough_body: Option<NoExitBlockRecipe>,
    },
    /// Conditional-update if (assignment/local + optional tail break/continue).
    ConditionalUpdateIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_body: Option<NoExitBlockRecipe>,
        then_exit: Option<ExitKind>,
        else_body: Option<NoExitBlockRecipe>,
        else_exit: Option<ExitKind>,
    },
    /// General if (no exit in branches), represented as a join-bearing RecipeBlock.
    GeneralIf(NoExitBlockRecipe),
    /// Nested loop (depth=1) allowed by facts.
    NestedLoopDepth1 {
        loop_stmt: StmtRef,
        nested: NestedLoopDepth1Recipe,
    },
    /// Tail break at top level.
    TailBreak {
        block: Option<ExitAllowedBlockRecipe>,
    },
    /// If-else where only else has return (then=non-exit, else=return value)
    ElseOnlyReturnIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_no_exit: Option<NoExitBlockRecipe>,
        else_return_stmt: StmtRef,
    },
    /// If-else where only then has return (then=return value, else=non-exit)
    ThenOnlyReturnIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_return_stmt: StmtRef,
        else_no_exit: Option<NoExitBlockRecipe>,
    },
    /// If-else where only else has break (then=non-exit, else=break)
    ElseOnlyBreakIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_no_exit: Option<NoExitBlockRecipe>,
        else_break_stmt: StmtRef,
    },
    /// If-else where only then has break (then=break, else=non-exit)
    ThenOnlyBreakIf {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        then_break_stmt: StmtRef,
        else_no_exit: Option<NoExitBlockRecipe>,
    },
    /// If-else where else has guard breaks (exit-ifs)
    /// Pattern: if cond { non-exit } else { (if guard { break })+ + non-exit }
    /// Recipe-first: both branches are pre-classified recipes.
    ElseGuardBreakIf {
        if_stmt: StmtRef,
        /// Optional payload for the "then" branch when it can be represented as a
        /// join-bearing, no-exit `RecipeBlock`.
        ///
        /// This payload is an optimization to avoid re-walking `then_body` in features.
        /// If `None`, lowering must fall back to the legacy `then_body` recipe.
        then_no_exit: Option<NoExitBlockRecipe>,
        else_exit_allowed: Option<ExitAllowedBlockRecipe>,
        then_body: LoopCondBreakContinueRecipe,
        else_body: LoopCondBreakContinueRecipe,
    },
    /// Exit leaf: break/continue/return（再帰の終端）
    /// Used inside ExitIfTree for recursive exit structure.
    ExitLeaf { kind: ExitKind, stmt: StmtRef },
    /// Exit-if tree: nested if structure where all branches end with exit.
    /// Pattern: if cond { exit-only-recipe } else { exit-only-recipe }
    /// Recursive structure for nested if-in-loop with break/continue/return.
    ExitIfTree {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        mode: IfMode,
        then_body: ExitOnlyBlockRecipe,
        else_body: Option<ExitOnlyBlockRecipe>,
    },
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct ExitIfRef<'a> {
    pub if_stmt: StmtRef,
    pub exit_allowed_block: Option<&'a ExitAllowedBlockRecipe>,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct ElseGuardBreakIfRef<'a> {
    pub if_stmt: StmtRef,
    pub then_no_exit: Option<&'a NoExitBlockRecipe>,
    pub then_recipe: &'a LoopCondBreakContinueRecipe,
    pub else_recipe: &'a LoopCondBreakContinueRecipe,
    pub else_exit_allowed: Option<&'a ExitAllowedBlockRecipe>,
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

    pub fn as_exit_if(&self) -> Option<ExitIfRef<'_>> {
        match self {
            Self::ExitIf { if_stmt, block } => Some(ExitIfRef {
                if_stmt: *if_stmt,
                exit_allowed_block: block.as_ref(),
            }),
            _ => None,
        }
    }

    pub fn tail_break_exit_allowed_block(&self) -> Option<&ExitAllowedBlockRecipe> {
        match self {
            Self::TailBreak { block } => block.as_ref(),
            _ => None,
        }
    }

    pub fn is_tail_break(&self) -> bool {
        matches!(self, Self::TailBreak { .. })
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

    pub fn as_else_guard_break_if(&self) -> Option<ElseGuardBreakIfRef<'_>> {
        match self {
            Self::ElseGuardBreakIf {
                if_stmt,
                then_no_exit,
                else_exit_allowed,
                then_body,
                else_body,
            } => Some(ElseGuardBreakIfRef {
                if_stmt: *if_stmt,
                then_no_exit: then_no_exit.as_ref(),
                then_recipe: then_body,
                else_recipe: else_body,
                else_exit_allowed: else_exit_allowed.as_ref(),
            }),
            _ => None,
        }
    }
}
