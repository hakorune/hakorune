//! Draft/sealed publication boundary for resolved function semantics.

use std::collections::BTreeMap;

use hakorune_mir_core::BindingId;

use super::ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId};
use super::normalized::{build_normalized_graph, NormalizedResolvedFunctionGraphV1};
use super::records::{
    ResolvedAssignmentTargetV1, ResolvedBindingRecordV1, ResolvedExitRecordV1,
    ResolvedLexicalRefV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1,
};
use super::source_site::{
    FunctionOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
};
use super::verifier::{verify_resolved_function, ResolvedFunctionVerificationErrorV1};

#[derive(Debug)]
pub(crate) struct ResolvedFunctionDataV1 {
    pub(crate) owner: FunctionOwnerIdV1,
    pub(crate) function_origin: FunctionOriginV1,
    pub(crate) function_scope: ScopeId,
    pub(crate) function_region: RegionId,
    pub(crate) bindings: BTreeMap<BindingId, ResolvedBindingRecordV1>,
    pub(crate) scopes: BTreeMap<ScopeId, ResolvedScopeRecordV1>,
    pub(crate) regions: BTreeMap<RegionId, ResolvedRegionRecordV1>,
    pub(crate) declarations: BTreeMap<SourceBindingSiteV1, BindingRefV1>,
    pub(crate) variable_uses: BTreeMap<SourceExprSiteV1, ResolvedLexicalRefV1>,
    pub(crate) assignment_targets: BTreeMap<SourceExprSiteV1, ResolvedAssignmentTargetV1>,
    pub(crate) resolved_exits: BTreeMap<ResolvedExitSiteV1, ResolvedExitRecordV1>,
}

/// Mutable construction state. It is never a public consumer input.
#[derive(Debug)]
pub(crate) struct ResolvedFunctionDraftV1 {
    pub(crate) data: ResolvedFunctionDataV1,
}

/// Immutable semantic authority published only after verification.
#[derive(Debug)]
pub struct VerifiedResolvedFunctionV1 {
    data: ResolvedFunctionDataV1,
    normalized: NormalizedResolvedFunctionGraphV1,
}

impl ResolvedFunctionDraftV1 {
    pub(crate) fn seal(
        self,
    ) -> Result<VerifiedResolvedFunctionV1, ResolvedFunctionVerificationErrorV1> {
        verify_resolved_function(&self.data)?;
        let normalized = build_normalized_graph(&self.data);
        Ok(VerifiedResolvedFunctionV1 {
            data: self.data,
            normalized,
        })
    }
}

impl VerifiedResolvedFunctionV1 {
    pub const fn owner(&self) -> FunctionOwnerIdV1 {
        self.data.owner
    }

    pub const fn function_origin(&self) -> FunctionOriginV1 {
        self.data.function_origin
    }

    pub const fn function_scope(&self) -> ScopeId {
        self.data.function_scope
    }

    pub const fn function_region(&self) -> RegionId {
        self.data.function_region
    }

    pub fn binding_ref(&self, id: BindingId) -> Option<BindingRefV1> {
        self.data
            .bindings
            .contains_key(&id)
            .then(|| BindingRefV1::new(self.data.owner, id))
    }

    pub fn binding(&self, id: BindingRefV1) -> Option<&ResolvedBindingRecordV1> {
        (id.owner() == self.data.owner)
            .then(|| self.data.bindings.get(&id.binding()))
            .flatten()
    }

    pub(crate) fn bindings(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, &ResolvedBindingRecordV1)> {
        self.data
            .bindings
            .iter()
            .map(|(binding, record)| (BindingRefV1::new(self.data.owner, *binding), record))
    }

    pub fn scope(&self, id: ScopeId) -> Option<&ResolvedScopeRecordV1> {
        (id.owner() == self.data.owner)
            .then(|| self.data.scopes.get(&id))
            .flatten()
    }

    pub fn region(&self, id: RegionId) -> Option<&ResolvedRegionRecordV1> {
        (id.owner() == self.data.owner)
            .then(|| self.data.regions.get(&id))
            .flatten()
    }

    pub fn declaration_binding(&self, site: &SourceBindingSiteV1) -> Option<BindingRefV1> {
        self.data.declarations.get(site).copied()
    }

    pub(crate) fn declaration_sites(&self) -> impl Iterator<Item = &SourceBindingSiteV1> {
        self.data.declarations.keys()
    }

    pub fn variable_ref(&self, site: &SourceExprSiteV1) -> Option<ResolvedLexicalRefV1> {
        self.data.variable_uses.get(site).copied()
    }

    pub(crate) fn variable_refs(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedLexicalRefV1)> {
        self.data.variable_uses.iter()
    }

    pub fn assignment_target(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&ResolvedAssignmentTargetV1> {
        self.data.assignment_targets.get(site)
    }

    pub(crate) fn assignment_targets(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedAssignmentTargetV1)> {
        self.data.assignment_targets.iter()
    }

    pub fn resolved_exit(&self, site: &ResolvedExitSiteV1) -> Option<&ResolvedExitRecordV1> {
        self.data.resolved_exits.get(site)
    }

    pub fn binding_count(&self) -> usize {
        self.data.bindings.len()
    }

    pub fn scope_count(&self) -> usize {
        self.data.scopes.len()
    }

    pub fn region_count(&self) -> usize {
        self.data.regions.len()
    }

    pub fn normalized_graph(&self) -> &NormalizedResolvedFunctionGraphV1 {
        &self.normalized
    }

    pub(crate) fn exact_scope_containing(
        &self,
        site: &super::source_site::SourceNodeSiteV1,
    ) -> Option<ScopeId> {
        let region = super::verifier::exact_source_region_v1(&self.data, site)?;
        self.data.regions.get(&region)?.lexical_scope()
    }
}
