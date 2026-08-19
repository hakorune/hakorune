//! Draft/sealed publication boundary for resolved function semantics.
use std::collections::{BTreeMap, BTreeSet};

use hakorune_mir_core::BindingId;

use super::body_shape::VerifiedResolvedMethodCallSourceV1;
use super::brand_source_relation::VerifiedBrandCallSourceRelationV1;
use super::direct_call::ResolvedDirectCallTargetV1;
use super::enum_variant_demand::EnumVariantAdmissionV1;
use super::explicit_extern_call::ResolvedExplicitExternCallV1;
use super::expression_source::ResolvedExpressionSourceInventoryV1;
use super::function_root::ResolvedFunctionLoweringRootsV1;
use super::ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId};
use super::if_region::ResolvedIfRegionIndexV1;
use super::loop_region::ResolvedLoopRegionIndexV1;
use super::normalized::{build_normalized_graph, NormalizedResolvedFunctionGraphV1};
use super::owner_root_profile::SemanticOwnerRootProfileV1;
use super::records::{
    BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedExitRecordV1, ResolvedLexicalRefV1, ResolvedRegionRecordV1,
    ResolvedScopeRecordV1, ScopeKindV1, ScopeOriginV1,
};
use super::source_site::{
    FunctionOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
};
use super::source_site_inventory::{
    seal_source_site_inventory_v1, ResolvedSourceSiteInventoryDraftV1,
    VerifiedResolvedSourceSiteInventoryV1,
};
use super::verifier::{verify_resolved_function, ResolvedFunctionVerificationErrorV1};

#[derive(Debug)]
pub(crate) struct ResolvedFunctionDataV1 {
    pub(crate) owner: FunctionOwnerIdV1,
    pub(crate) function_origin: FunctionOriginV1,
    pub(crate) root_profile: SemanticOwnerRootProfileV1,
    pub(crate) function_scope: ScopeId,
    pub(crate) function_region: RegionId,
    pub(crate) bindings: BTreeMap<BindingId, ResolvedBindingRecordV1>,
    pub(crate) scopes: BTreeMap<ScopeId, ResolvedScopeRecordV1>,
    pub(crate) regions: BTreeMap<RegionId, ResolvedRegionRecordV1>,
    pub(crate) declarations: BTreeMap<SourceBindingSiteV1, BindingRefV1>,
    pub(crate) variable_uses: BTreeMap<SourceExprSiteV1, ResolvedLexicalRefV1>,
    pub(crate) assignment_targets: BTreeMap<SourceExprSiteV1, ResolvedAssignmentTargetV1>,
    pub(crate) direct_call_targets: BTreeMap<SourceExprSiteV1, ResolvedDirectCallTargetV1>,
    pub(crate) brand_call_relations: BTreeMap<SourceExprSiteV1, VerifiedBrandCallSourceRelationV1>,
    pub(crate) explicit_extern_calls: BTreeMap<SourceExprSiteV1, ResolvedExplicitExternCallV1>,
    pub(crate) method_calls: BTreeMap<SourceExprSiteV1, VerifiedResolvedMethodCallSourceV1>,
    pub(crate) expression_source: ResolvedExpressionSourceInventoryV1,
    pub(crate) resolved_exits: BTreeMap<ResolvedExitSiteV1, ResolvedExitRecordV1>,
}

/// Mutable construction state. It is never a public consumer input.
#[derive(Debug)]
pub(crate) struct ResolvedFunctionDraftV1 {
    pub(crate) data: ResolvedFunctionDataV1,
}

/// Root-neutral immutable semantic authority published after verification.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedOwnerCoreV1 {
    data: ResolvedFunctionDataV1,
    source_sites: VerifiedResolvedSourceSiteInventoryV1,
    normalized: NormalizedResolvedFunctionGraphV1,
    lowering_roots: ResolvedFunctionLoweringRootsV1,
    pub(crate) if_regions: ResolvedIfRegionIndexV1,
    pub(crate) loop_regions: ResolvedLoopRegionIndexV1,
}

impl VerifiedResolvedOwnerCoreV1 {
    pub(crate) const fn data(&self) -> &ResolvedFunctionDataV1 {
        &self.data
    }

    pub(crate) const fn normalized_graph(&self) -> &NormalizedResolvedFunctionGraphV1 {
        &self.normalized
    }
}

/// Immutable declared-function/Lambda authority. The public wrapper remains
/// stable while the forest evolves to hold additional root profiles.
#[derive(Debug)]
pub struct VerifiedResolvedFunctionV1 {
    pub(crate) core: VerifiedResolvedOwnerCoreV1,
}

/// Script wrapper carried by the shared forest for the literal-only producer.
/// Its public surface stays narrower than the declared-function wrapper.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedScriptV1 {
    core: VerifiedResolvedOwnerCoreV1,
    record_literal_demands: BTreeMap<SourceExprSiteV1, u32>,
    enum_variant_demands: BTreeMap<SourceExprSiteV1, EnumVariantAdmissionV1>,
    enum_match_demands: BTreeSet<SourceExprSiteV1>,
    qmark_propagation_sites: BTreeSet<SourceExprSiteV1>,
    match_control_sites: BTreeSet<SourceExprSiteV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedScopeRegionPairV1 {
    scope: ScopeId,
    region: RegionId,
}

impl ResolvedScopeRegionPairV1 {
    pub(super) const fn from_verified(scope: ScopeId, region: RegionId) -> Self {
        Self { scope, region }
    }

    pub(crate) const fn scope(self) -> ScopeId {
        self.scope
    }

    pub(crate) const fn region(self) -> RegionId {
        self.region
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedScopeRegionLookupErrorV1 {
    ForeignOwner,
    MissingExactPair,
    PairContractMismatch,
}

impl ResolvedFunctionDraftV1 {
    #[cfg(test)]
    pub(crate) fn seal(
        self,
    ) -> Result<VerifiedResolvedFunctionV1, ResolvedFunctionVerificationErrorV1> {
        let source_sites =
            ResolvedSourceSiteInventoryDraftV1::covering_existing_indexes(&self.data);
        self.seal_with_source_sites(source_sites)
    }

    pub(crate) fn seal_with_source_sites(
        self,
        source_sites: ResolvedSourceSiteInventoryDraftV1,
    ) -> Result<VerifiedResolvedFunctionV1, ResolvedFunctionVerificationErrorV1> {
        Ok(VerifiedResolvedFunctionV1 {
            core: seal_owner_core(self.data, source_sites)?,
        })
    }
}

fn seal_owner_core(
    data: ResolvedFunctionDataV1,
    source_sites: ResolvedSourceSiteInventoryDraftV1,
) -> Result<VerifiedResolvedOwnerCoreV1, ResolvedFunctionVerificationErrorV1> {
    let source_sites = seal_source_site_inventory_v1(source_sites, &data)
        .map_err(ResolvedFunctionVerificationErrorV1::SourceSiteInventory)?;
    let derived = verify_resolved_function(&data)?;
    let normalized = build_normalized_graph(&data);
    Ok(VerifiedResolvedOwnerCoreV1 {
        data,
        source_sites,
        normalized,
        lowering_roots: derived.lowering_roots,
        if_regions: derived.if_regions,
        loop_regions: derived.loop_regions,
    })
}

impl VerifiedResolvedFunctionV1 {
    pub(crate) const fn core(&self) -> &VerifiedResolvedOwnerCoreV1 {
        &self.core
    }

    pub const fn owner(&self) -> FunctionOwnerIdV1 {
        self.core.data.owner
    }

    pub const fn function_origin(&self) -> FunctionOriginV1 {
        self.core.data.function_origin
    }

    pub const fn source_kind(&self) -> super::SemanticOwnerSourceKindV1 {
        self.core.data.root_profile.source_kind()
    }

    pub const fn source_site_inventory(&self) -> &VerifiedResolvedSourceSiteInventoryV1 {
        &self.core.source_sites
    }

    pub(crate) const fn root_profile(&self) -> SemanticOwnerRootProfileV1 {
        self.core.data.root_profile
    }

    pub const fn function_scope(&self) -> ScopeId {
        self.core.data.function_scope
    }

    pub const fn function_region(&self) -> RegionId {
        self.core.data.function_region
    }

    pub(crate) const fn lowering_roots(&self) -> ResolvedFunctionLoweringRootsV1 {
        self.core.lowering_roots
    }

    pub fn binding_ref(&self, id: BindingId) -> Option<BindingRefV1> {
        self.core
            .data
            .bindings
            .contains_key(&id)
            .then(|| BindingRefV1::new(self.core.data.owner, id))
    }

    pub fn binding(&self, id: BindingRefV1) -> Option<&ResolvedBindingRecordV1> {
        (id.owner() == self.core.data.owner)
            .then(|| self.core.data.bindings.get(&id.binding()))
            .flatten()
    }

    pub(crate) fn bindings(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, &ResolvedBindingRecordV1)> {
        self.core
            .data
            .bindings
            .iter()
            .map(|(binding, record)| (BindingRefV1::new(self.core.data.owner, *binding), record))
    }

    pub fn scope(&self, id: ScopeId) -> Option<&ResolvedScopeRecordV1> {
        (id.owner() == self.core.data.owner)
            .then(|| self.core.data.scopes.get(&id))
            .flatten()
    }

    pub(crate) fn scopes(&self) -> impl Iterator<Item = (ScopeId, &ResolvedScopeRecordV1)> {
        self.core
            .data
            .scopes
            .iter()
            .map(|(scope, record)| (*scope, record))
    }

    pub fn region(&self, id: RegionId) -> Option<&ResolvedRegionRecordV1> {
        (id.owner() == self.core.data.owner)
            .then(|| self.core.data.regions.get(&id))
            .flatten()
    }

    pub(crate) fn regions(&self) -> impl Iterator<Item = (RegionId, &ResolvedRegionRecordV1)> {
        self.core
            .data
            .regions
            .iter()
            .map(|(region, record)| (*region, record))
    }

    pub fn declaration_binding(&self, site: &SourceBindingSiteV1) -> Option<BindingRefV1> {
        self.core.data.declarations.get(site).copied()
    }

    pub(crate) fn declaration_sites(&self) -> impl Iterator<Item = &SourceBindingSiteV1> {
        self.core.data.declarations.keys()
    }

    pub fn variable_ref(&self, site: &SourceExprSiteV1) -> Option<ResolvedLexicalRefV1> {
        self.core.data.variable_uses.get(site).copied()
    }

    pub(crate) fn variable_refs(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedLexicalRefV1)> {
        self.core.data.variable_uses.iter()
    }

    pub fn assignment_target(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&ResolvedAssignmentTargetV1> {
        self.core.data.assignment_targets.get(site)
    }

    pub(crate) fn assignment_targets(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedAssignmentTargetV1)> {
        self.core.data.assignment_targets.iter()
    }

    pub(crate) fn direct_call_target(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<ResolvedDirectCallTargetV1> {
        self.core.data.direct_call_targets.get(site).copied()
    }

    pub(crate) fn direct_call_targets(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, ResolvedDirectCallTargetV1)> {
        self.core
            .data
            .direct_call_targets
            .iter()
            .map(|(site, target)| (site, *target))
    }

    pub(crate) fn brand_call_relation(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedBrandCallSourceRelationV1> {
        self.core.data.brand_call_relations.get(site)
    }

    pub(crate) fn brand_call_relations(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &VerifiedBrandCallSourceRelationV1)> {
        self.core.data.brand_call_relations.iter()
    }

    pub(crate) fn expression_sites(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.core.source_sites.expression_sites()
    }

    pub(crate) fn explicit_extern_call(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&ResolvedExplicitExternCallV1> {
        self.core.data.explicit_extern_calls.get(site)
    }

    pub(crate) fn explicit_extern_calls(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedExplicitExternCallV1)> {
        self.core.data.explicit_extern_calls.iter()
    }

    pub(crate) fn method_calls(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &VerifiedResolvedMethodCallSourceV1)> {
        self.core.data.method_calls.iter()
    }

    pub(crate) const fn expression_source(&self) -> &ResolvedExpressionSourceInventoryV1 {
        &self.core.data.expression_source
    }

    pub fn resolved_exit(&self, site: &ResolvedExitSiteV1) -> Option<&ResolvedExitRecordV1> {
        self.core.data.resolved_exits.get(site)
    }

    pub(crate) fn resolved_exits(
        &self,
    ) -> impl Iterator<Item = (&ResolvedExitSiteV1, &ResolvedExitRecordV1)> {
        self.core.data.resolved_exits.iter()
    }

    pub fn binding_count(&self) -> usize {
        self.core.data.bindings.len()
    }

    pub fn scope_count(&self) -> usize {
        self.core.data.scopes.len()
    }

    pub fn region_count(&self) -> usize {
        self.core.data.regions.len()
    }

    pub fn normalized_graph(&self) -> &NormalizedResolvedFunctionGraphV1 {
        &self.core.normalized
    }

    pub(crate) fn exact_scope_containing(
        &self,
        site: &super::source_site::SourceNodeSiteV1,
    ) -> Option<ScopeId> {
        let region = super::verifier::exact_source_region_v1(&self.core.data, site)?;
        self.core.data.regions.get(&region)?.lexical_scope()
    }

    pub(crate) fn block_expr_scope_region_pair(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> Result<ResolvedScopeRegionPairV1, ResolvedScopeRegionLookupErrorV1> {
        if owner != self.core.data.owner {
            return Err(ResolvedScopeRegionLookupErrorV1::ForeignOwner);
        }
        let path = SourcePathV1::from_node(site.node());
        let origin = path.child(SourcePathSegmentV1::BlockExprPreludeRoot).node();
        let matching_regions = self
            .core
            .data
            .regions
            .iter()
            .filter(|(_, record)| {
                record.kind() == RegionKindV1::BlockExpr
                    && matches!(
                        record.origin(),
                        RegionOriginV1::Source(actual) if actual == &origin
                    )
            })
            .collect::<Vec<_>>();
        if matching_regions.is_empty() {
            return Err(ResolvedScopeRegionLookupErrorV1::MissingExactPair);
        }
        if matching_regions.len() != 1 {
            return Err(ResolvedScopeRegionLookupErrorV1::PairContractMismatch);
        }
        let (&region, region_record) = matching_regions[0];
        let scope = region_record
            .lexical_scope()
            .ok_or(ResolvedScopeRegionLookupErrorV1::PairContractMismatch)?;
        let scope_record = self
            .scope(scope)
            .ok_or(ResolvedScopeRegionLookupErrorV1::PairContractMismatch)?;
        if region_record.kind() != RegionKindV1::BlockExpr
            || scope_record.kind() != ScopeKindV1::BlockExpr
            || scope_record.owner_region() != region
            || scope_record.origin() != &ScopeOriginV1::Source(origin)
        {
            return Err(ResolvedScopeRegionLookupErrorV1::PairContractMismatch);
        }
        Ok(ResolvedScopeRegionPairV1 { scope, region })
    }
}

impl VerifiedResolvedScriptV1 {
    pub(crate) fn from_canonical_data(
        data: ResolvedFunctionDataV1,
        source_sites: ResolvedSourceSiteInventoryDraftV1,
        record_literal_demands: BTreeMap<SourceExprSiteV1, u32>,
        enum_variant_demands: BTreeMap<SourceExprSiteV1, EnumVariantAdmissionV1>,
        enum_match_demands: BTreeSet<SourceExprSiteV1>,
        qmark_propagation_sites: BTreeSet<SourceExprSiteV1>,
        match_control_sites: BTreeSet<SourceExprSiteV1>,
    ) -> Result<Self, ResolvedFunctionVerificationErrorV1> {
        Ok(Self {
            core: seal_owner_core(data, source_sites)?,
            record_literal_demands,
            enum_variant_demands,
            enum_match_demands,
            qmark_propagation_sites,
            match_control_sites,
        })
    }

    pub(crate) const fn core(&self) -> &VerifiedResolvedOwnerCoreV1 {
        &self.core
    }

    pub(crate) fn declaration_binding(&self, site: &SourceBindingSiteV1) -> Option<BindingRefV1> {
        self.core.data.declarations.get(site).copied()
    }

    pub(crate) fn brand_call_relation(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedBrandCallSourceRelationV1> {
        self.core.data.brand_call_relations.get(site)
    }

    pub(crate) fn brand_call_relations(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &VerifiedBrandCallSourceRelationV1)> {
        self.core.data.brand_call_relations.iter()
    }

    pub(crate) fn expression_sites(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.core.source_sites.expression_sites()
    }

    pub(crate) fn record_literal_demands(&self) -> impl Iterator<Item = (&SourceExprSiteV1, u32)> {
        self.record_literal_demands
            .iter()
            .map(|(site, count)| (site, *count))
    }

    pub(crate) fn enum_variant_demands(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &EnumVariantAdmissionV1)> {
        self.enum_variant_demands.iter()
    }

    pub(crate) fn enum_match_demands(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.enum_match_demands.iter()
    }

    pub(crate) fn qmark_propagation_sites(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.qmark_propagation_sites.iter()
    }

    pub(crate) fn match_control_sites(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.match_control_sites.iter()
    }
}
