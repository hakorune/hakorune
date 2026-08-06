//! AST-free source products for the bounded LoopCond branch shape.
//!
//! The compiler projector is the syntax observer and constructs these
//! products. Policy consumes them as opaque resolver-branded facts. This
//! first slice deliberately excludes the legacy LoopCond recipe variants.

use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedControlTransferV1,
    ResolvedExitOriginV1, SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
};

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
