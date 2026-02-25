//! Shape-only reference types (meaning-neutral).
//!
//! These types represent structural shapes without semantic meaning.
//! Semantic vocabulary (ExitIf, GeneralIf, etc.) stays in each box's enum.

use super::{StmtIdx, StmtRange};

/// Single statement reference (shape-only, no semantic meaning)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct StmtRef {
    pub idx: StmtIdx,
}

impl StmtRef {
    pub(in crate::mir::builder) fn new(idx: usize) -> Self {
        Self { idx: StmtIdx(idx) }
    }

    pub(in crate::mir::builder) fn index(self) -> usize {
        self.idx.0
    }
}

/// Pair of statement references (shape-only)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct StmtPair {
    pub a: StmtIdx,
    pub b: StmtIdx,
}

impl StmtPair {
    pub(in crate::mir::builder) fn new(a: usize, b: usize) -> Self {
        Self { a: StmtIdx(a), b: StmtIdx(b) }
    }
}

/// Span of statements (shape-only)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct StmtSpan {
    pub range: StmtRange,
}

impl StmtSpan {
    pub(in crate::mir::builder) fn new(start: usize, end: usize) -> Self {
        Self {
            range: StmtRange::new(start, end),
        }
    }

    pub(in crate::mir::builder) fn start_index(self) -> usize {
        self.range.start_index()
    }

    pub(in crate::mir::builder) fn end_index(self) -> usize {
        self.range.end_index()
    }

    pub(in crate::mir::builder) fn indices(self) -> (usize, usize) {
        self.range.indices()
    }
}
