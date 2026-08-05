//! Test-only passive provenance product for the Generic carrier handoff.
//!
//! The resolver-issued owner is the only brand issuer.  This module consumes
//! one already-branded handoff and publishes one opaque AST-free witness; it
//! never assigns a Generic key, selects a route, or touches Builder effects.

use super::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedLoopSourceForestV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedCarrierRoleKindV1 {
    NestedWrite,
    PostLoopRead,
    /// Fixture-only value used to prove missing/unsupported role rejection.
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResolvedCarrierRoleV1 {
    kind: ResolvedCarrierRoleKindV1,
    site: SourceExprSiteV1,
    binding: BindingRefV1,
    strict_ancestor: bool,
}

impl ResolvedCarrierRoleV1 {
    pub(crate) fn for_test(
        kind: ResolvedCarrierRoleKindV1,
        site: SourceExprSiteV1,
        binding: BindingRefV1,
        strict_ancestor: bool,
    ) -> Self {
        Self {
            kind,
            site,
            binding,
            strict_ancestor,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BrandedResolvedForestV1 {
    owner: FunctionOwnerIdV1,
    forest: VerifiedResolvedLoopSourceForestV1,
}

impl BrandedResolvedForestV1 {
    /// Test-fixture ingress for a resolver-issued forest.  Production code
    /// has no constructor because this module is `cfg(test)` only.
    pub(crate) fn for_test(
        owner: FunctionOwnerIdV1,
        forest: VerifiedResolvedLoopSourceForestV1,
    ) -> Self {
        Self { owner, forest }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BrandedResolvedFrameV1 {
    owner: FunctionOwnerIdV1,
    frame: LoopExecutionFrameKeyV1,
}

impl BrandedResolvedFrameV1 {
    /// Test-fixture ingress for the matching resolver-issued frame.
    pub(crate) fn for_test(owner: FunctionOwnerIdV1, frame: LoopExecutionFrameKeyV1) -> Self {
        Self { owner, frame }
    }
}

/// One co-sealed handoff.  It is deliberately non-`Clone` so a consumer
/// cannot independently pair forest/frame/role fragments after issuance.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResolvedCarrierHandoffV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    outer_site: SourceStmtSiteV1,
    inner_site: SourceStmtSiteV1,
    forest: BrandedResolvedForestV1,
    frame: BrandedResolvedFrameV1,
    roles: [ResolvedCarrierRoleV1; 2],
}

impl ResolvedCarrierHandoffV1 {
    /// Fixture-only assembly of one resolver-owned handoff.  The product
    /// factory below is the only semantic constructor and consumes this
    /// value as a unit.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn for_test(
        owner: FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        outer_site: SourceStmtSiteV1,
        inner_site: SourceStmtSiteV1,
        forest: BrandedResolvedForestV1,
        frame: BrandedResolvedFrameV1,
        roles: [ResolvedCarrierRoleV1; 2],
    ) -> Self {
        Self {
            owner,
            function_origin,
            source_kind,
            outer_site,
            inner_site,
            forest,
            frame,
            roles,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProvenanceRejectV1 {
    MixedOwnerBrand,
    ForeignBindingOwner,
    MissingNestedWrite,
    MissingPostLoopRead,
    DuplicateRole,
    UnsupportedRole,
    BindingRelation,
    ForestShape,
    ForestIdentity,
    FrameMismatch,
    SourceKindMismatch,
    StrictAncestorMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedResolvedCarrierProvenanceV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    outer_site: SourceStmtSiteV1,
    inner_site: SourceStmtSiteV1,
    forest: BrandedResolvedForestV1,
    frame: BrandedResolvedFrameV1,
    roles: [ResolvedCarrierRoleV1; 2],
    _seal: ProvenanceSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct ProvenanceSealV1;

impl VerifiedResolvedCarrierProvenanceV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn outer_site(&self) -> &SourceStmtSiteV1 {
        &self.outer_site
    }

    pub(crate) fn inner_site(&self) -> &SourceStmtSiteV1 {
        &self.inner_site
    }

    pub(crate) fn role_kinds(&self) -> [ResolvedCarrierRoleKindV1; 2] {
        [self.roles[0].kind, self.roles[1].kind]
    }
}

/// Seal one passive AST-free source witness.  No selector, Generic snapshot,
/// logical key, or Builder/MIR artifact is produced by this function.
pub(crate) fn issue_resolved_carrier_provenance_v1(
    handoff: ResolvedCarrierHandoffV1,
) -> Result<VerifiedResolvedCarrierProvenanceV1, ProvenanceRejectV1> {
    validate_handoff(&handoff)?;
    let ResolvedCarrierHandoffV1 {
        owner,
        function_origin,
        source_kind,
        outer_site,
        inner_site,
        forest,
        frame,
        roles,
    } = handoff;
    Ok(VerifiedResolvedCarrierProvenanceV1 {
        owner,
        function_origin,
        source_kind,
        outer_site,
        inner_site,
        forest,
        frame,
        roles,
        _seal: ProvenanceSealV1,
    })
}

fn validate_handoff(handoff: &ResolvedCarrierHandoffV1) -> Result<(), ProvenanceRejectV1> {
    if handoff.forest.owner != handoff.owner || handoff.frame.owner != handoff.owner {
        return Err(ProvenanceRejectV1::MixedOwnerBrand);
    }
    if handoff.source_kind != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(ProvenanceRejectV1::SourceKindMismatch);
    }
    if handoff
        .roles
        .iter()
        .any(|role| role.kind == ResolvedCarrierRoleKindV1::Unknown)
    {
        return Err(ProvenanceRejectV1::UnsupportedRole);
    }

    let nested = handoff
        .roles
        .iter()
        .filter(|role| role.kind == ResolvedCarrierRoleKindV1::NestedWrite)
        .collect::<Vec<_>>();
    let post = handoff
        .roles
        .iter()
        .filter(|role| role.kind == ResolvedCarrierRoleKindV1::PostLoopRead)
        .collect::<Vec<_>>();
    if nested.len() > 1 || post.len() > 1 {
        return Err(ProvenanceRejectV1::DuplicateRole);
    }
    if nested.is_empty() {
        return Err(ProvenanceRejectV1::MissingNestedWrite);
    }
    if post.is_empty() {
        return Err(ProvenanceRejectV1::MissingPostLoopRead);
    }
    let nested = nested[0];
    let post = post[0];
    if nested.binding.owner() != handoff.owner || post.binding.owner() != handoff.owner {
        return Err(ProvenanceRejectV1::ForeignBindingOwner);
    }
    if nested.binding != post.binding {
        return Err(ProvenanceRejectV1::BindingRelation);
    }
    if !nested.strict_ancestor || post.strict_ancestor {
        return Err(ProvenanceRejectV1::StrictAncestorMismatch);
    }

    let members = handoff.forest.forest.members();
    if members.len() != 2
        || members[0].parent_index().is_some()
        || members[1].parent_index() != Some(0)
    {
        return Err(ProvenanceRejectV1::ForestShape);
    }
    if !members[0].source().matches_identity(
        handoff.function_origin,
        handoff.source_kind,
        &handoff.outer_site,
    ) || !members[1].source().matches_identity(
        handoff.function_origin,
        handoff.source_kind,
        &handoff.inner_site,
    ) {
        return Err(ProvenanceRejectV1::ForestIdentity);
    }
    if !members[0]
        .source()
        .frame_key()
        .matches(&handoff.frame.frame)
    {
        return Err(ProvenanceRejectV1::FrameMismatch);
    }
    Ok(())
}
