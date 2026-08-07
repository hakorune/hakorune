//! Resolver-owned, AST-free source rows for one callable owner.
//!
//! This view is deliberately a query surface over an already sealed resolver
//! product.  It does not copy ValueIds, inspect AST nodes, issue Loop policy,
//! or lower a Recipe.  Each source family keeps its typed key and has one
//! explicit query instead of being flattened into a route-local map.

use super::direct_call::ResolvedDirectCallTargetV1;
use super::ordered_capture::OrderedCaptureDemandV1;
use super::owner_forest::VerifiedSemanticOwnerForestV1;
use super::records::{
    ResolvedAssignmentTargetV1, ResolvedBindingRecordV1, ResolvedExitRecordV1, ResolvedLexicalRefV1,
};
use super::source_site::{
    FunctionOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};
use super::{
    FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ResolvedLoopRegionLookupErrorV1,
    ResolvedScopeRegionPairV1, SemanticOwnerSourceKindV1, VerifiedResolvedFunctionV1,
    VerifiedResolvedLoopSourceV1, VerifiedResolvedSourceSiteInventoryV1,
};

/// The source families intentionally exposed by the first callable ledger.
///
/// This is an inventory vocabulary, not a Loop selector.  Keeping the family
/// names explicit makes an omitted resolver row visible at the API boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum CallableSourceRowFamilyV1 {
    Declaration,
    LexicalRef,
    AssignmentTarget,
    DirectCall,
    Exit,
    LambdaCapture,
    LoopMembership,
}

impl CallableSourceRowFamilyV1 {
    pub(crate) const ALL: [Self; 7] = [
        Self::Declaration,
        Self::LexicalRef,
        Self::AssignmentTarget,
        Self::DirectCall,
        Self::Exit,
        Self::LambdaCapture,
        Self::LoopMembership,
    ];
}

/// Explicit presence state for one resolver-owned row family.
///
/// `Empty` means the sealed owner has no row in that family. It is not an
/// implicit fallback or an unsupported-source claim; unsupported/opaque
/// source dispositions belong to the later source-to-Recipe map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallableSourceRowDispositionV1 {
    Published(usize),
    Empty,
}

impl CallableSourceRowDispositionV1 {
    pub(crate) const fn count(self) -> usize {
        match self {
            Self::Published(count) => count,
            Self::Empty => 0,
        }
    }
}

/// Construction rejection for a callable view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallableSourceLedgerRejectV1 {
    MissingOwner(FunctionOwnerIdV1),
    NonCallableOwner(FunctionOwnerIdV1),
}

/// A resolver-issued Loop source token paired with its opaque execution frame.
///
/// The frame is derived from the non-`Clone` source token.  No route id or raw
/// source suffix can mint either value.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallableLoopMembershipV1 {
    source: VerifiedResolvedLoopSourceV1,
    frame: LoopExecutionFrameKeyV1,
    scope_region: ResolvedScopeRegionPairV1,
}

impl VerifiedCallableLoopMembershipV1 {
    pub(crate) fn source(&self) -> &VerifiedResolvedLoopSourceV1 {
        &self.source
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) const fn scope_region(&self) -> ResolvedScopeRegionPairV1 {
        self.scope_region
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedResolvedLoopSourceV1,
        LoopExecutionFrameKeyV1,
        ResolvedScopeRegionPairV1,
    ) {
        (self.source, self.frame, self.scope_region)
    }
}

/// Immutable borrowed view over one resolver-owned callable product.
#[derive(Debug)]
pub(crate) struct CallableSemanticSourceLedgerView<'a> {
    forest: &'a VerifiedSemanticOwnerForestV1,
    function: &'a VerifiedResolvedFunctionV1,
    owner: FunctionOwnerIdV1,
}

impl<'a> CallableSemanticSourceLedgerView<'a> {
    pub(crate) fn from_forest(
        forest: &'a VerifiedSemanticOwnerForestV1,
        owner: FunctionOwnerIdV1,
    ) -> Result<Self, CallableSourceLedgerRejectV1> {
        let Some(product) = forest.semantic_owner(owner) else {
            return Err(CallableSourceLedgerRejectV1::MissingOwner(owner));
        };
        let Some(function) = product.as_function() else {
            return Err(CallableSourceLedgerRejectV1::NonCallableOwner(owner));
        };
        Ok(Self {
            forest,
            function,
            owner,
        })
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function.function_origin()
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.function.source_kind()
    }

    pub(crate) const fn source_site_inventory(&self) -> &VerifiedResolvedSourceSiteInventoryV1 {
        self.function.source_site_inventory()
    }

    pub(crate) fn declaration_sites(&self) -> impl Iterator<Item = &SourceBindingSiteV1> {
        self.function.declaration_sites()
    }

    pub(crate) fn declaration_binding(
        &self,
        site: &SourceBindingSiteV1,
    ) -> Option<super::BindingRefV1> {
        self.function.declaration_binding(site)
    }

    pub(crate) fn bindings(
        &self,
    ) -> impl Iterator<Item = (super::BindingRefV1, &ResolvedBindingRecordV1)> {
        self.function.bindings()
    }

    pub(crate) fn variable_refs(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedLexicalRefV1)> {
        self.function.variable_refs()
    }

    pub(crate) fn assignment_targets(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedAssignmentTargetV1)> {
        self.function.assignment_targets()
    }

    pub(crate) fn direct_call_targets(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, ResolvedDirectCallTargetV1)> {
        self.function.direct_call_targets()
    }

    pub(crate) fn resolved_exits(
        &self,
    ) -> impl Iterator<Item = (&ResolvedExitSiteV1, &ResolvedExitRecordV1)> {
        self.function.resolved_exits()
    }

    /// Returns the resolver's existing capture boundary for this owner.
    pub(crate) fn capture_demands(&self) -> &[OrderedCaptureDemandV1] {
        self.forest.ordered_capture_demands(self.owner)
    }

    pub(crate) fn family_count(&self, family: CallableSourceRowFamilyV1) -> usize {
        match family {
            CallableSourceRowFamilyV1::Declaration => self.declaration_sites().count(),
            CallableSourceRowFamilyV1::LexicalRef => self.variable_refs().count(),
            CallableSourceRowFamilyV1::AssignmentTarget => self.assignment_targets().count(),
            CallableSourceRowFamilyV1::DirectCall => self.direct_call_targets().count(),
            CallableSourceRowFamilyV1::Exit => self.resolved_exits().count(),
            CallableSourceRowFamilyV1::LambdaCapture => self.capture_demands().len(),
            CallableSourceRowFamilyV1::LoopMembership => self.function.loop_region_bundle_count(),
        }
    }

    pub(crate) fn family_disposition(
        &self,
        family: CallableSourceRowFamilyV1,
    ) -> CallableSourceRowDispositionV1 {
        match self.family_count(family) {
            0 => CallableSourceRowDispositionV1::Empty,
            count => CallableSourceRowDispositionV1::Published(count),
        }
    }

    /// Issue exact Loop membership and frame identity from the sealed index.
    pub(crate) fn only_loop_site(
        &self,
    ) -> Result<VerifiedCallableLoopMembershipV1, ResolvedLoopRegionLookupErrorV1> {
        let site = self.function.only_loop_site()?;
        let (source, scope_region) = self.function.resolved_loop_source_context(&site)?;
        let frame = source.frame_key();
        Ok(VerifiedCallableLoopMembershipV1 {
            source,
            frame,
            scope_region,
        })
    }

    /// Issue exact Loop membership and frame identity from the sealed index.
    pub(crate) fn resolved_loop_source(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Result<VerifiedCallableLoopMembershipV1, ResolvedLoopRegionLookupErrorV1> {
        let (source, scope_region) = self.function.resolved_loop_source_context(site)?;
        let frame = source.frame_key();
        Ok(VerifiedCallableLoopMembershipV1 {
            source,
            frame,
            scope_region,
        })
    }
}

impl VerifiedSemanticOwnerForestV1 {
    pub(crate) fn callable_source_ledger(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<CallableSemanticSourceLedgerView<'_>, CallableSourceLedgerRejectV1> {
        CallableSemanticSourceLedgerView::from_forest(self, owner)
    }
}
