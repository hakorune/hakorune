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

/// One pair of complete S2 products.  The repeat observer accepts this pair
/// as a single unit so no downstream caller can re-pair its private parts.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ProvenanceRepeatInputV1 {
    left: VerifiedResolvedCarrierProvenanceV1,
    right: VerifiedResolvedCarrierProvenanceV1,
}

impl ProvenanceRepeatInputV1 {
    #[cfg(test)]
    pub(crate) fn for_test(
        left: VerifiedResolvedCarrierProvenanceV1,
        right: VerifiedResolvedCarrierProvenanceV1,
    ) -> Self {
        Self { left, right }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProvenanceRepeatRejectV1 {
    ReusedOrEqualOwnerBrand,
    FunctionOriginMismatch,
    SourceKindMismatch,
    OuterInnerSiteMismatch,
    ForestTopologyMismatch,
    RoleKindOrSiteMismatch,
    BindingRelationMismatch,
    StrictAncestorMismatch,
    FrameCoordinateMismatch,
    MissingOrDetachedProduct,
    MixedOrForeignOwnerBrand,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ProvenanceRepeatObservationV1 {
    source_topology_equal: bool,
    outer_inner_sites_equal: bool,
    roles_equal: bool,
    strict_ancestor_equal: bool,
    owner_brands_distinct: bool,
    raw_frame_coordinates_equal: bool,
    _seal: ProvenanceRepeatSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct ProvenanceRepeatSealV1;

impl ProvenanceRepeatObservationV1 {
    pub(crate) const fn source_topology_equal(&self) -> bool {
        self.source_topology_equal
    }

    pub(crate) const fn outer_inner_sites_equal(&self) -> bool {
        self.outer_inner_sites_equal
    }

    pub(crate) const fn roles_equal(&self) -> bool {
        self.roles_equal
    }

    pub(crate) const fn strict_ancestor_equal(&self) -> bool {
        self.strict_ancestor_equal
    }

    pub(crate) const fn owner_brands_distinct(&self) -> bool {
        self.owner_brands_distinct
    }

    pub(crate) const fn raw_frame_coordinates_equal(&self) -> bool {
        self.raw_frame_coordinates_equal
    }
}

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

/// Observe one repeat across fresh resolver products.  This is deliberately a
/// passive sink: it consumes complete products, performs no source scan, and
/// never creates Generic selection or Builder state.
pub(crate) fn issue_provenance_repeat_audit_v1(
    input: ProvenanceRepeatInputV1,
) -> Result<ProvenanceRepeatObservationV1, ProvenanceRepeatRejectV1> {
    let ProvenanceRepeatInputV1 { left, right } = input;
    if left.owner == right.owner {
        return Err(ProvenanceRepeatRejectV1::ReusedOrEqualOwnerBrand);
    }
    if left.function_origin != right.function_origin {
        return Err(ProvenanceRepeatRejectV1::FunctionOriginMismatch);
    }
    if left.source_kind != right.source_kind {
        return Err(ProvenanceRepeatRejectV1::SourceKindMismatch);
    }
    if left.outer_site != right.outer_site || left.inner_site != right.inner_site {
        return Err(ProvenanceRepeatRejectV1::OuterInnerSiteMismatch);
    }
    if left.forest.forest != right.forest.forest {
        return Err(ProvenanceRepeatRejectV1::ForestTopologyMismatch);
    }
    if left.forest.owner != left.owner
        || right.forest.owner != right.owner
        || left.frame.owner != left.owner
        || right.frame.owner != right.owner
    {
        return Err(ProvenanceRepeatRejectV1::MixedOrForeignOwnerBrand);
    }
    for (left_role, right_role) in left.roles.iter().zip(right.roles.iter()) {
        if left_role.kind != right_role.kind || left_role.site != right_role.site {
            return Err(ProvenanceRepeatRejectV1::RoleKindOrSiteMismatch);
        }
        if left_role.binding.binding() != right_role.binding.binding() {
            return Err(ProvenanceRepeatRejectV1::BindingRelationMismatch);
        }
        if left_role.strict_ancestor != right_role.strict_ancestor {
            return Err(ProvenanceRepeatRejectV1::StrictAncestorMismatch);
        }
    }
    let raw_frame_coordinates_equal = left.frame.frame == right.frame.frame;
    if !raw_frame_coordinates_equal {
        return Err(ProvenanceRepeatRejectV1::FrameCoordinateMismatch);
    }
    Ok(ProvenanceRepeatObservationV1 {
        source_topology_equal: true,
        outer_inner_sites_equal: true,
        roles_equal: true,
        strict_ancestor_equal: true,
        owner_brands_distinct: true,
        raw_frame_coordinates_equal,
        _seal: ProvenanceRepeatSealV1,
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
