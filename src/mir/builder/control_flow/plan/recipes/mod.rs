//! Recipe base types (SSOT).
//!
//! Goal: keep Facts→Lower contracts crisp and prevent "acceptance drift".
//! A box that adopts recipe-first should:
//! - own the original body as `RecipeBody` (no borrowed lifetimes across layers)
//! - reference statements by `StmtIdx` / `StmtRange` (not raw usize)
//! - let Facts build a recipe; let Lower consume the recipe only (no re-validation)
//!
//! See also:
//! - docs/development/current/main/design/boxcount-new-box-addition-checklist-ssot.md
//! - docs/development/current/main/design/feature-helper-cross-pipeline-map.md

pub mod refs;

use crate::ast::ASTNode;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct RecipeBody {
    pub body: Vec<ASTNode>,
}

impl RecipeBody {
    pub fn new(body: Vec<ASTNode>) -> Self {
        Self { body }
    }

    pub fn get(&self, idx: StmtIdx) -> Option<&ASTNode> {
        self.body.get(idx.0)
    }

    pub fn get_ref(&self, stmt: refs::StmtRef) -> Option<&ASTNode> {
        self.body.get(stmt.index())
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

impl Deref for RecipeBody {
    type Target = [ASTNode];

    fn deref(&self) -> &Self::Target {
        self.body.as_slice()
    }
}

impl AsRef<[ASTNode]> for RecipeBody {
    fn as_ref(&self) -> &[ASTNode] {
        self.body.as_slice()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(in crate::mir::builder) struct StmtIdx(pub usize);

impl StmtIdx {
    pub(in crate::mir::builder) fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct StmtRange {
    pub start: StmtIdx,
    pub end: StmtIdx,
}

impl StmtRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: StmtIdx(start),
            end: StmtIdx(end),
        }
    }

    pub(in crate::mir::builder) fn start_index(self) -> usize {
        self.start.index()
    }

    pub(in crate::mir::builder) fn end_index(self) -> usize {
        self.end.index()
    }

    pub(in crate::mir::builder) fn indices(self) -> (usize, usize) {
        (self.start.index(), self.end.index())
    }
}
