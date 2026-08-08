//! Parser-private structural paths for top-level build-gate source transport.
//!
//! This module issues no semantic receipt and performs no branch selection. It
//! only preserves the parser-owned source coordinate until a later postpass.

use super::source_authority::ParserInvocationBrandV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct SourceBuildGateIdV1(u32);

impl SourceBuildGateIdV1 {
    pub(super) fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub(super) fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceBuildGateBranchV1 {
    Then,
    Else,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SourceBoxPathSegmentV1 {
    RootStatement {
        ordinal: u32,
    },
    BuildGate {
        gate_id: SourceBuildGateIdV1,
        branch: SourceBuildGateBranchV1,
        child_ordinal: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceBoxDeclarationPathV1 {
    brand: ParserInvocationBrandV1,
    segments: Box<[SourceBoxPathSegmentV1]>,
}

impl SourceBoxDeclarationPathV1 {
    pub(super) fn root(brand: ParserInvocationBrandV1, ordinal: u32) -> Self {
        Self {
            brand,
            segments: vec![SourceBoxPathSegmentV1::RootStatement { ordinal }].into_boxed_slice(),
        }
    }

    pub(super) fn child(
        &self,
        gate_id: SourceBuildGateIdV1,
        branch: SourceBuildGateBranchV1,
        child_ordinal: u32,
    ) -> Self {
        let mut segments = self.segments.to_vec();
        segments.push(SourceBoxPathSegmentV1::BuildGate {
            gate_id,
            branch,
            child_ordinal,
        });
        Self {
            brand: self.brand.clone(),
            segments: segments.into_boxed_slice(),
        }
    }

    pub(super) fn brand(&self) -> &ParserInvocationBrandV1 {
        &self.brand
    }

    pub(super) fn segments(&self) -> &[SourceBoxPathSegmentV1] {
        &self.segments
    }

    pub(super) fn root_statement_ordinal(&self) -> Option<u32> {
        match self.segments.first() {
            Some(SourceBoxPathSegmentV1::RootStatement { ordinal }) => Some(*ordinal),
            _ => None,
        }
    }
}

/// Parser-owned cursor used while descending a build-gate branch. It only
/// issues structural child paths; it does not select or prune a branch.
#[derive(Debug, Clone)]
pub(super) struct SourceBoxPathCursorV1 {
    parent: SourceBoxDeclarationPathV1,
    gate_id: SourceBuildGateIdV1,
    branch: SourceBuildGateBranchV1,
    next_child_ordinal: u32,
}

impl SourceBoxPathCursorV1 {
    pub(super) fn new(
        parent: SourceBoxDeclarationPathV1,
        gate_id: SourceBuildGateIdV1,
        branch: SourceBuildGateBranchV1,
    ) -> Self {
        Self {
            parent,
            gate_id,
            branch,
            next_child_ordinal: 0,
        }
    }

    pub(super) fn next_child(&mut self) -> Option<SourceBoxDeclarationPathV1> {
        let ordinal = self.next_child_ordinal;
        self.next_child_ordinal = self.next_child_ordinal.checked_add(1)?;
        Some(self.parent.child(self.gate_id, self.branch, ordinal))
    }
}
