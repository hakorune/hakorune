//! Resolver-issued semantic context carried by a neutral Loop demand.
//!
//! This is a move-only transport wrapper. It does not issue source identity,
//! select a Recipe family, or create a physical identity.

use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedScopeRegionPairV1,
    SemanticOwnerSourceKindV1, SourceStmtSiteV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopSemanticContextV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
}

impl VerifiedLoopSemanticContextV1 {
    pub(crate) fn from_parts(
        owner: FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        loop_site: SourceStmtSiteV1,
        frame: LoopExecutionFrameKeyV1,
        scope_region: ResolvedScopeRegionPairV1,
    ) -> Self {
        Self {
            owner,
            origin,
            source_kind,
            loop_site,
            frame,
            scope_region,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn origin(&self) -> FunctionOriginV1 {
        self.origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }
}
