#![allow(dead_code)]
//! RecipeBlock: body-referencing recipe tree with arena (M5m scaffold).
//!
//! Body is NOT embedded in the tree: `BodyId` indexes into `RecipeBodies`.
//! `RecipeItem::IfV2` holds `if_stmt: StmtRef` (the If node itself).
//! `RecipeItem::Exit` uses existing `ExitKind`.

use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::plan::recipes::RecipeBody;
use super::common::{ExitKind, IfMode};

/// Contract kind for `RecipeItem` if-vocabulary unification (M19).
///
/// This is meaning-level *contract* only; the structural shape stays the same.
///
/// NOTE: This enum is introduced first (no behavior change). A follow-up step
/// adds a unified-if `RecipeItem` variant and migrates producers gradually.
#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) enum IfContractKind {
    ExitOnly { mode: IfMode },
    /// then=fallthrough, else=exit-only (ElseOnlyExit pattern)
    ExitAllowed { mode: IfMode },
    Join,
}

/// Contract for a `RecipeBlock` used as a loop body.
///
/// This is a structural/contract hint only; it must not encode CFG/SSA details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum BlockContractKind {
    StmtOnly,
    NoExit,
    ExitAllowed,
    ExitOnly,
}

/// Loop kind for `RecipeItem::LoopV0` (structure-only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopKindV0 {
    WhileLike,
    Infinite,
}

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::mir::builder) struct LoopV0Features;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct BodyId(pub usize);

#[derive(Debug, Clone, Default)]
pub(in crate::mir::builder) struct RecipeBodies {
    bodies: Vec<RecipeBody>,
}

impl RecipeBodies {
    pub(in crate::mir::builder) fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub(in crate::mir::builder) fn register(&mut self, body: RecipeBody) -> BodyId {
        let id = BodyId(self.bodies.len());
        self.bodies.push(body);
        id
    }

    pub(in crate::mir::builder) fn get(&self, id: BodyId) -> Option<&RecipeBody> {
        self.bodies.get(id.0)
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct RecipeBlock {
    pub body_id: BodyId,
    pub items: Vec<RecipeItem>,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum RecipeItem {
    Stmt(StmtRef),

    /// Loop structure vocabulary (structure-only).
    ///
    /// CFG/SSA payload 禁止:
    /// - BB/PHI/Frag/ValueId/CFG などの “下ろしの都合” を recipe payload に持ち込まない。
    /// - RecipeItem は構造語彙のみを追加対象とする（acceptance policy は Facts/Verifier 側で固定する）。
    LoopV0 {
        loop_stmt: StmtRef,
        kind: LoopKindV0,
        cond_view: CondBlockView,
        body_block: Box<RecipeBlock>,
        body_contract: BlockContractKind,
        features: LoopV0Features,
    },

    /// Unified-if vocabulary (M19): contract distinguishes exit-only vs join-bearing.
    ///
    /// NOTE: Producers will migrate gradually. Consumers must treat unknown items
    /// as `[freeze:contract][recipe]` (see M19-1c/M19-1d).
    IfV2 {
        if_stmt: StmtRef,
        cond_view: CondBlockView,
        contract: IfContractKind,
        then_block: Box<RecipeBlock>,
        else_block: Option<Box<RecipeBlock>>,
    },

    /// Exit kind uses existing ExitKind (Break/Continue include depth).
    Exit {
        kind: ExitKind,
        stmt: StmtRef,
    },
}

impl RecipeBlock {
    pub fn new(body_id: BodyId, items: Vec<RecipeItem>) -> Self {
        Self { body_id, items }
    }
}
