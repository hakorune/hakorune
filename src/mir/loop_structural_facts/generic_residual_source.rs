//! AST-free source product for the bounded Generic residual profile.
//!
//! The compiler projector is the syntax observer and constructs this
//! product. Policy consumes it as an opaque resolver-branded fact. The
//! bounded profile is the single-induction integer-progression
//! `GenericLoopV1` source shape; it owns no Recipe, route, Builder, MIR,
//! or physical identity.

use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceExprSiteV1, SourceStmtSiteV1,
};

/// One ordered top-level loop-body statement kept as a resolver site.
///
/// `LocalDeclaration` is a `local` statement inside the loop body;
/// `Rebind` is an `Assignment` whose target is a `Variable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericResidualBodyStatementKindV1 {
    LocalDeclaration,
    Rebind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericResidualBodyStatementV1 {
    pub(crate) site: SourceStmtSiteV1,
    pub(crate) kind: GenericResidualBodyStatementKindV1,
}

/// Bounded single-induction integer-progression source shape. The last
/// `body_statements` entry is the terminal carrier step and is always
/// `Rebind`; every earlier entry is `LocalDeclaration` or `Rebind`.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericResidualSourceShapeV1 {
    pub(crate) loop_site: SourceStmtSiteV1,
    pub(crate) loop_condition_site: SourceExprSiteV1,
    pub(crate) body_statements: Box<[GenericResidualBodyStatementV1]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericResidualSourceProjectionV1 {
    owner: FunctionOwnerIdV1,
    shape: VerifiedGenericResidualSourceShapeV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    root_frame_key: LoopExecutionFrameKeyV1,
}

impl VerifiedGenericResidualSourceProjectionV1 {
    pub(crate) fn new(
        owner: FunctionOwnerIdV1,
        shape: VerifiedGenericResidualSourceShapeV1,
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

    pub(crate) fn shape(&self) -> &VerifiedGenericResidualSourceShapeV1 {
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
