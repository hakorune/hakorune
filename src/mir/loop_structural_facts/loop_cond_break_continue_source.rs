//! AST-free source products for the bounded LoopCond branch shape.
//!
//! The compiler projector is the syntax observer and constructs these
//! products. Policy consumes them as opaque resolver-branded facts. This
//! first slice deliberately excludes the legacy LoopCond recipe variants.

use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedControlTransferV1,
    ResolvedExitOriginV1, ResolvedExitRecordV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceStmtSiteV1,
};

use super::VerifiedLoopSourceForestBindingV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondBreakContinueSourceShapeV1 {
    pub(crate) loop_site: SourceStmtSiteV1,
    pub(crate) loop_condition_site: SourceExprSiteV1,
    pub(crate) branch_site: SourceStmtSiteV1,
    pub(crate) branch_condition_site: SourceExprSiteV1,
    pub(crate) then_exit_site: SourceStmtSiteV1,
    pub(crate) then_exit_origin: ResolvedExitOriginV1,
    pub(crate) then_exit_transfer: ResolvedControlTransferV1,
    pub(crate) else_exit_site: SourceStmtSiteV1,
    pub(crate) else_exit_origin: ResolvedExitOriginV1,
    pub(crate) else_exit_transfer: ResolvedControlTransferV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondBreakContinueSourceProjectionV1 {
    owner: FunctionOwnerIdV1,
    shape: VerifiedLoopCondBreakContinueSourceShapeV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_frame_key: LoopExecutionFrameKeyV1,
}

impl VerifiedLoopCondBreakContinueSourceProjectionV1 {
    pub(crate) fn new(
        owner: FunctionOwnerIdV1,
        shape: VerifiedLoopCondBreakContinueSourceShapeV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        root_frame_key: LoopExecutionFrameKeyV1,
    ) -> Self {
        Self {
            owner,
            shape,
            function_origin,
            source_kind,
            root_frame_key,
        }
    }

    pub(crate) fn shape(&self) -> &VerifiedLoopCondBreakContinueSourceShapeV1 {
        &self.shape
    }

    pub(crate) const fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn matches_source_identity(
        &self,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        self.function_origin == function_origin
            && self.source_kind == source_kind
            && &self.shape.loop_site == site
    }
}

/// One resolver-owned exit row retained by the bounded source forest handoff.
///
/// The source site and its transfer record stay paired so a later consumer
/// cannot join an exit by ordinal or reconstruct its target from syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondSourceExitV1 {
    site: SourceStmtSiteV1,
    record: ResolvedExitRecordV1,
}

impl VerifiedLoopCondSourceExitV1 {
    pub(crate) fn new(site: SourceStmtSiteV1, record: ResolvedExitRecordV1) -> Self {
        Self { site, record }
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn record(&self) -> &ResolvedExitRecordV1 {
        &self.record
    }
}

/// AST-free co-seal for one nested LoopCond source forest.
///
/// The resolver-issued forest binding owns member order and parentage. The
/// exit rows are collected from the same resolved function and are retained
/// with their exact source sites. This product has no Recipe, route, Builder,
/// MIR, or physical identity and is consumed exactly once by a later owner.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
    owner: FunctionOwnerIdV1,
    forest_binding: VerifiedLoopSourceForestBindingV1,
    member_sites: Box<[SourceStmtSiteV1]>,
    exits: Box<[VerifiedLoopCondSourceExitV1]>,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_frame_key: LoopExecutionFrameKeyV1,
}

impl VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
    pub(crate) fn new(
        owner: FunctionOwnerIdV1,
        forest_binding: VerifiedLoopSourceForestBindingV1,
        member_sites: Box<[SourceStmtSiteV1]>,
        exits: Box<[VerifiedLoopCondSourceExitV1]>,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        root_frame_key: LoopExecutionFrameKeyV1,
    ) -> Self {
        Self {
            owner,
            forest_binding,
            member_sites,
            exits,
            function_origin,
            source_kind,
            root_frame_key,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn forest_binding(&self) -> &VerifiedLoopSourceForestBindingV1 {
        &self.forest_binding
    }

    pub(crate) fn member_sites(&self) -> &[SourceStmtSiteV1] {
        &self.member_sites
    }

    pub(crate) fn exits(&self) -> &[VerifiedLoopCondSourceExitV1] {
        &self.exits
    }

    pub(crate) const fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn matches_source_identity(
        &self,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        root_site: &SourceStmtSiteV1,
    ) -> bool {
        self.function_origin == function_origin
            && self.source_kind == source_kind
            && self.member_sites.first() == Some(root_site)
    }
}
