//! Passive records stored in a sealed resolved-function arena.

use super::ids::{BindingRefV1, RegionId, ScopeId, UpvarRefV1};
use super::source_site::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BindingKindV1 {
    Receiver,
    Parameter { index: u32 },
    Local { ordinal: u32 },
    Outbox { ordinal: u32 },
    Nowait,
    LoopBinder,
    CatchBinder { ordinal: u32 },
    PatternBinder { ordinal: u32 },
    CompilerSynthetic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntheticBindingKindV1 {
    DeclaredContractSlot,
}

/// Diagnostic/parity provenance for an arena-owned binding entity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BindingOriginV1 {
    Source(SourceBindingSiteV1),
    Synthetic {
        owner: SourceNodeSiteV1,
        kind: SyntheticBindingKindV1,
        ordinal: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResolvedLexicalRefV1 {
    Local(BindingRefV1),
    Upvar(UpvarRefV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedBindingRecordV1 {
    diagnostic_name: Box<str>,
    kind: BindingKindV1,
    owner_scope: ScopeId,
    origin: BindingOriginV1,
}

impl ResolvedBindingRecordV1 {
    pub(crate) fn new(
        diagnostic_name: impl Into<Box<str>>,
        kind: BindingKindV1,
        owner_scope: ScopeId,
        origin: BindingOriginV1,
    ) -> Self {
        Self {
            diagnostic_name: diagnostic_name.into(),
            kind,
            owner_scope,
            origin,
        }
    }

    pub fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub const fn kind(&self) -> BindingKindV1 {
        self.kind
    }

    pub const fn owner_scope(&self) -> ScopeId {
        self.owner_scope
    }

    pub fn origin(&self) -> &BindingOriginV1 {
        &self.origin
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeKindV1 {
    Function,
    LexicalBlock,
    IfThen,
    IfElse,
    LoopBody,
    Catch,
    PatternArm,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeOriginV1 {
    Function(FunctionOriginV1),
    Source(SourceNodeSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedScopeRecordV1 {
    kind: ScopeKindV1,
    parent: Option<ScopeId>,
    owner_region: RegionId,
    declarations: Box<[BindingRefV1]>,
    origin: ScopeOriginV1,
}

impl ResolvedScopeRecordV1 {
    pub(crate) fn new(
        kind: ScopeKindV1,
        parent: Option<ScopeId>,
        owner_region: RegionId,
        declarations: Vec<BindingRefV1>,
        origin: ScopeOriginV1,
    ) -> Self {
        Self {
            kind,
            parent,
            owner_region,
            declarations: declarations.into_boxed_slice(),
            origin,
        }
    }

    pub const fn kind(&self) -> ScopeKindV1 {
        self.kind
    }

    pub const fn parent(&self) -> Option<ScopeId> {
        self.parent
    }

    pub const fn owner_region(&self) -> RegionId {
        self.owner_region
    }

    pub fn declarations(&self) -> &[BindingRefV1] {
        &self.declarations
    }

    pub fn origin(&self) -> &ScopeOriginV1 {
        &self.origin
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegionKindV1 {
    Function,
    Sequence,
    LexicalScope,
    If,
    IfThen,
    IfElse,
    Loop,
    Try,
    Catch,
    Finally,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegionOriginV1 {
    Function(FunctionOriginV1),
    Source(SourceNodeSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRegionRecordV1 {
    kind: RegionKindV1,
    parent: Option<RegionId>,
    lexical_scope: Option<ScopeId>,
    origin: RegionOriginV1,
}

impl ResolvedRegionRecordV1 {
    pub(crate) fn new(
        kind: RegionKindV1,
        parent: Option<RegionId>,
        lexical_scope: Option<ScopeId>,
        origin: RegionOriginV1,
    ) -> Self {
        Self {
            kind,
            parent,
            lexical_scope,
            origin,
        }
    }

    pub const fn kind(&self) -> RegionKindV1 {
        self.kind
    }

    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    pub const fn lexical_scope(&self) -> Option<ScopeId> {
        self.lexical_scope
    }

    pub fn origin(&self) -> &RegionOriginV1 {
        &self.origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedAssignmentTargetV1 {
    BindingRebind(BindingRefV1),
    UpvarRebind(UpvarRefV1),
    FieldWrite { receiver: SourceExprSiteV1 },
    IndexWrite { receiver: SourceExprSiteV1 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResolvedExitOriginV1 {
    ExplicitContinue,
    ExplicitBreak,
    ExplicitReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResolvedControlTransferV1 {
    Continue { target_loop: RegionId },
    Break { target_loop: RegionId },
    Return { target_function: RegionId },
}

/// One atomic source-to-control correspondence.
///
/// Keeping the containing source region and transfer together removes the
/// parallel-map update order that previously existed in the sealed product.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedExitRecordV1 {
    source_region: RegionId,
    origin: ResolvedExitOriginV1,
    transfer: ResolvedControlTransferV1,
}

impl ResolvedExitRecordV1 {
    pub(crate) const fn new(
        source_region: RegionId,
        origin: ResolvedExitOriginV1,
        transfer: ResolvedControlTransferV1,
    ) -> Self {
        Self {
            source_region,
            origin,
            transfer,
        }
    }

    pub const fn source_region(&self) -> RegionId {
        self.source_region
    }

    pub const fn origin(&self) -> ResolvedExitOriginV1 {
        self.origin
    }

    pub const fn transfer(&self) -> ResolvedControlTransferV1 {
        self.transfer
    }
}
