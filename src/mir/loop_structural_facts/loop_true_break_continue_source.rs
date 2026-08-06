//! AST-free source products for the bounded LoopTrue branch shape.
//!
//! The compiler projector is the syntax observer and constructs these
//! products. Policy consumes them as opaque resolver-branded facts.

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::VerifiedLoopRootSourceV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinueSourceShapeV1 {
    pub(crate) loop_site: SourceStmtSiteV1,
    pub(crate) loop_condition_site: SourceExprSiteV1,
    pub(crate) branch_site: SourceStmtSiteV1,
    pub(crate) branch_condition_site: SourceExprSiteV1,
    pub(crate) branch_condition_lhs_site: SourceExprSiteV1,
    pub(crate) branch_condition_rhs_site: SourceExprSiteV1,
    pub(crate) branch_condition_binding: BindingRefV1,
    pub(crate) branch_condition_bound: i64,
    pub(crate) then_exit_site: SourceStmtSiteV1,
    pub(crate) else_exit_site: SourceStmtSiteV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueBreakContinueSourceProjectionV1 {
    source_binding: VerifiedLoopRootSourceV1,
    shape: VerifiedLoopTrueBreakContinueSourceShapeV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_frame_key: LoopExecutionFrameKeyV1,
}

impl VerifiedLoopTrueBreakContinueSourceProjectionV1 {
    pub(crate) fn new(
        source_binding: VerifiedLoopRootSourceV1,
        shape: VerifiedLoopTrueBreakContinueSourceShapeV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        root_frame_key: LoopExecutionFrameKeyV1,
    ) -> Self {
        Self {
            source_binding,
            shape,
            function_origin,
            source_kind,
            root_frame_key,
        }
    }

    pub(crate) fn source_binding(&self) -> &VerifiedLoopRootSourceV1 {
        &self.source_binding
    }

    pub(crate) fn shape(&self) -> &VerifiedLoopTrueBreakContinueSourceShapeV1 {
        &self.shape
    }

    pub(crate) const fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.shape.branch_condition_binding.owner()
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

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopRootSourceV1,
        VerifiedLoopTrueBreakContinueSourceShapeV1,
        LoopExecutionFrameKeyV1,
    ) {
        (self.source_binding, self.shape, self.root_frame_key)
    }
}
