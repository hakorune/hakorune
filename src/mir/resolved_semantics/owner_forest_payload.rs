//! Shared forest payload boundary.
//!
//! The payload enum lets the shared forest carry Function/Lambda and the
//! first narrow Script product without creating a second forest authority or
//! changing the existing Function API.

use super::direct_call::ResolvedDirectCallTargetV1;
use super::ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId};
use super::product::{
    VerifiedResolvedFunctionV1, VerifiedResolvedOwnerCoreV1, VerifiedResolvedScriptV1,
};
use super::records::{
    ResolvedAssignmentTargetV1, ResolvedBindingRecordV1, ResolvedExitRecordV1,
    ResolvedLexicalRefV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1,
};
use super::source_site::{
    FunctionOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
};
use super::{NormalizedResolvedFunctionGraphV1, SemanticOwnerSourceKindV1};

#[derive(Debug)]
pub(crate) enum VerifiedSemanticOwnerProductV1 {
    Function(VerifiedResolvedFunctionV1),
    Script(VerifiedResolvedScriptV1),
}

impl VerifiedSemanticOwnerProductV1 {
    pub(crate) const fn core(&self) -> &VerifiedResolvedOwnerCoreV1 {
        match self {
            Self::Function(product) => product.core(),
            Self::Script(product) => product.core(),
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.core().data().owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.core().data().function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.core().data().root_profile.source_kind()
    }

    pub(crate) const fn root_profile(
        &self,
    ) -> super::owner_root_profile::SemanticOwnerRootProfileV1 {
        self.core().data().root_profile
    }

    pub(crate) const fn function_scope(&self) -> ScopeId {
        self.core().data().function_scope
    }

    pub(crate) const fn function_region(&self) -> RegionId {
        self.core().data().function_region
    }

    pub(crate) fn binding(&self, id: BindingRefV1) -> Option<&ResolvedBindingRecordV1> {
        (id.owner() == self.owner())
            .then(|| self.core().data().bindings.get(&id.binding()))
            .flatten()
    }

    pub(crate) fn bindings(
        &self,
    ) -> impl Iterator<Item = (BindingRefV1, &ResolvedBindingRecordV1)> {
        self.core()
            .data()
            .bindings
            .iter()
            .map(|(binding, record)| (BindingRefV1::new(self.owner(), *binding), record))
    }

    pub(crate) fn scope(&self, id: ScopeId) -> Option<&ResolvedScopeRecordV1> {
        (id.owner() == self.owner())
            .then(|| self.core().data().scopes.get(&id))
            .flatten()
    }

    pub(crate) fn scopes(&self) -> impl Iterator<Item = (ScopeId, &ResolvedScopeRecordV1)> {
        self.core()
            .data()
            .scopes
            .iter()
            .map(|(scope, record)| (*scope, record))
    }

    pub(crate) fn region(&self, id: RegionId) -> Option<&ResolvedRegionRecordV1> {
        (id.owner() == self.owner())
            .then(|| self.core().data().regions.get(&id))
            .flatten()
    }

    pub(crate) fn regions(&self) -> impl Iterator<Item = (RegionId, &ResolvedRegionRecordV1)> {
        self.core()
            .data()
            .regions
            .iter()
            .map(|(region, record)| (*region, record))
    }

    pub(crate) fn declaration_binding(&self, site: &SourceBindingSiteV1) -> Option<BindingRefV1> {
        self.core().data().declarations.get(site).copied()
    }

    pub(crate) fn declaration_sites(&self) -> impl Iterator<Item = &SourceBindingSiteV1> {
        self.core().data().declarations.keys()
    }

    pub(crate) fn variable_refs(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedLexicalRefV1)> {
        self.core().data().variable_uses.iter()
    }

    pub(crate) fn assignment_targets(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedAssignmentTargetV1)> {
        self.core().data().assignment_targets.iter()
    }

    pub(crate) fn direct_call_targets(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, ResolvedDirectCallTargetV1)> {
        self.core()
            .data()
            .direct_call_targets
            .iter()
            .map(|(site, target)| (site, *target))
    }

    pub(crate) fn resolved_exits(
        &self,
    ) -> impl Iterator<Item = (&ResolvedExitSiteV1, &ResolvedExitRecordV1)> {
        self.core().data().resolved_exits.iter()
    }

    pub(crate) fn exact_scope_containing(&self, site: &SourceNodeSiteV1) -> Option<ScopeId> {
        let region = super::verifier::exact_source_region_v1(self.core().data(), site)?;
        self.core().data().regions.get(&region)?.lexical_scope()
    }

    pub(crate) fn normalized_graph(&self) -> &NormalizedResolvedFunctionGraphV1 {
        self.core().normalized_graph()
    }

    pub(crate) fn into_function(self) -> Option<VerifiedResolvedFunctionV1> {
        match self {
            Self::Function(product) => Some(product),
            Self::Script(_) => None,
        }
    }

    pub(crate) fn as_function(&self) -> Option<&VerifiedResolvedFunctionV1> {
        match self {
            Self::Function(product) => Some(product),
            Self::Script(_) => None,
        }
    }
}
