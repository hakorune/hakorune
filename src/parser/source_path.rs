//! Parser-private structural paths for top-level build-gate source transport.
//!
//! This module issues no semantic receipt and performs no branch selection. It
//! only preserves the parser-owned source coordinate until a later postpass.

use super::common::ParserUtils;
use super::source_authority::ParserInvocationBrandV1;
use super::{NyashParser, ParseError};
use crate::ast::{BuildPredicate, Span};

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

/// Program-wide declaration path used by callable source products.
///
/// `SourceBoxDeclarationPathV1` predates callable coverage and remains the
/// compatibility carrier used by Box source code.  This wrapper prevents that
/// historical name from becoming the durable contract for free functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceProgramDeclarationPathV1(SourceBoxDeclarationPathV1);

impl SourceProgramDeclarationPathV1 {
    pub(super) fn from_parser_path(path: SourceBoxDeclarationPathV1) -> Self {
        Self(path)
    }

    pub(super) fn brand(&self) -> &ParserInvocationBrandV1 {
        self.0.brand()
    }

    pub(super) fn compatibility_box_path(&self) -> &SourceBoxDeclarationPathV1 {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SourceProgramMemberGateStepV1 {
    gate_member_ordinal: u32,
    branch: SourceBuildGateBranchV1,
}

impl SourceProgramMemberGateStepV1 {
    pub(super) fn new(gate_member_ordinal: u32, branch: SourceBuildGateBranchV1) -> Self {
        Self {
            gate_member_ordinal,
            branch,
        }
    }

    pub(super) fn gate_member_ordinal(&self) -> u32 {
        self.gate_member_ordinal
    }

    pub(super) fn branch(&self) -> SourceBuildGateBranchV1 {
        self.branch
    }
}

/// Exact as-written source coordinate for one direct callable declaration.
/// Placement is intentionally distinct from the opaque callable anchor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SourceProgramCallablePathV1 {
    TopLevel {
        declaration: SourceProgramDeclarationPathV1,
    },
    BoxMethod {
        declaration: SourceProgramDeclarationPathV1,
        gate_path: Box<[SourceProgramMemberGateStepV1]>,
        member_ordinal: u32,
    },
}

impl SourceProgramCallablePathV1 {
    pub(super) fn top_level(path: SourceBoxDeclarationPathV1) -> Self {
        Self::TopLevel {
            declaration: SourceProgramDeclarationPathV1::from_parser_path(path),
        }
    }

    pub(super) fn box_method(
        declaration: SourceProgramDeclarationPathV1,
        gate_path: Box<[SourceProgramMemberGateStepV1]>,
        member_ordinal: u32,
    ) -> Self {
        Self::BoxMethod {
            declaration,
            gate_path,
            member_ordinal,
        }
    }

    pub(super) fn declaration(&self) -> &SourceProgramDeclarationPathV1 {
        match self {
            Self::TopLevel { declaration } | Self::BoxMethod { declaration, .. } => declaration,
        }
    }

    pub(super) fn box_method_parts(
        &self,
    ) -> Option<(
        &SourceProgramDeclarationPathV1,
        &[SourceProgramMemberGateStepV1],
        u32,
    )> {
        match self {
            Self::TopLevel { .. } => None,
            Self::BoxMethod {
                declaration,
                gate_path,
                member_ordinal,
            } => Some((declaration, gate_path, *member_ordinal)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceBuildGateScopeV1 {
    Closed,
    TopLevelItem,
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

/// Gate-node identity is deliberately distinct from a Box declaration path.
/// The coordinates are analogous, but a gate is not a Box source declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SourceBuildGatePathV1 {
    brand: ParserInvocationBrandV1,
    segments: Box<[SourceBuildGatePathSegmentV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SourceBuildGatePathSegmentV1 {
    RootTopLevel {
        statement_ordinal: u32,
    },
    BranchChild {
        parent_gate_id: SourceBuildGateIdV1,
        branch: SourceBuildGateBranchV1,
        child_ordinal: u32,
    },
}

impl SourceBuildGatePathV1 {
    pub(super) fn root(brand: ParserInvocationBrandV1, ordinal: u32) -> Self {
        Self {
            brand,
            segments: vec![SourceBuildGatePathSegmentV1::RootTopLevel {
                statement_ordinal: ordinal,
            }]
            .into_boxed_slice(),
        }
    }

    pub(super) fn from_box_path(path: &SourceBoxDeclarationPathV1) -> Option<Self> {
        Self::from_box_prefix(path, path.segments().len())
    }

    pub(super) fn from_box_prefix(
        path: &SourceBoxDeclarationPathV1,
        prefix_len: usize,
    ) -> Option<Self> {
        if prefix_len == 0 || prefix_len > path.segments().len() {
            return None;
        }
        let mut segments = Vec::with_capacity(prefix_len);
        for segment in path.segments().iter().take(prefix_len) {
            match segment {
                SourceBoxPathSegmentV1::RootStatement { ordinal } => {
                    segments.push(SourceBuildGatePathSegmentV1::RootTopLevel {
                        statement_ordinal: *ordinal,
                    });
                }
                SourceBoxPathSegmentV1::BuildGate {
                    gate_id,
                    branch,
                    child_ordinal,
                } => segments.push(SourceBuildGatePathSegmentV1::BranchChild {
                    parent_gate_id: *gate_id,
                    branch: *branch,
                    child_ordinal: *child_ordinal,
                }),
            }
        }
        Some(Self {
            brand: path.brand().clone(),
            segments: segments.into_boxed_slice(),
        })
    }

    pub(super) fn child(
        &self,
        parent_gate_id: SourceBuildGateIdV1,
        branch: SourceBuildGateBranchV1,
        child_ordinal: u32,
    ) -> Self {
        let mut segments = self.segments.to_vec();
        segments.push(SourceBuildGatePathSegmentV1::BranchChild {
            parent_gate_id,
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

    pub(super) fn segments(&self) -> &[SourceBuildGatePathSegmentV1] {
        &self.segments
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct PreparedBuildGateSourceRecordV1 {
    pub(super) brand: ParserInvocationBrandV1,
    pub(super) gate_id: SourceBuildGateIdV1,
    pub(super) gate_path: SourceBuildGatePathV1,
    pub(super) scope: SourceBuildGateScopeV1,
    pub(super) predicate: BuildPredicate,
    pub(super) span: Span,
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

impl NyashParser {
    pub(super) fn source_invocation_brand(&self) -> ParserInvocationBrandV1 {
        self.source_invocation_brand.clone()
    }

    pub(super) fn active_source_statement_ordinal(&self) -> Option<u32> {
        self.active_source_statement_ordinal
    }

    pub(super) fn active_source_declaration_path(&self) -> Option<&SourceBoxDeclarationPathV1> {
        self.active_source_declaration_path.as_ref()
    }

    pub(super) fn issue_source_build_gate_id(&mut self) -> Result<SourceBuildGateIdV1, ParseError> {
        let raw = self.next_source_build_gate_id;
        self.next_source_build_gate_id =
            self.next_source_build_gate_id
                .checked_add(1)
                .ok_or_else(|| ParseError::BuildCfg {
                    message: "parser source build-gate id exceeds u32".to_owned(),
                    line: self.current_token().line,
                })?;
        Ok(SourceBuildGateIdV1::from_raw(raw))
    }

    pub(super) fn register_prepared_source_seal(
        &mut self,
        prepared: super::source_authority::PreparedBoxSourceSealV1,
    ) {
        self.prepared_source_seals.push(prepared);
    }
}
