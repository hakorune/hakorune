//! Source-owned route token for the bounded LoopCond handoff.
//!
//! The V2 token binds the checked recipe to the resolver's loop projection
//! and exact source items before the writer consumes it.

use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionOriginV1, FunctionOwnerIdV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopSourceRouteRejectV1 {
    ProjectionMissing,
    ProjectionOwnerMismatch,
    ProjectionIdentityMismatch,
    #[cfg(test)]
    SourceItemsMissing,
    SourceItemForeign,
    SourceTargetMissing,
    #[cfg(test)]
    SourceTargetCardinality {
        expected: usize,
        actual: usize,
    },
    #[cfg(test)]
    SourceTargetOrderMismatch {
        expected: SourceExprSiteV1,
        actual: SourceExprSiteV1,
    },
    SourceTargetRequirementMismatch,
    SourceItemDuplicate {
        call_site: SourceExprSiteV1,
    },
    /// An exact same-module static target was resolved for the listed sites
    /// but its selected publication row is absent — target-only, missing,
    /// or already consumed evidence all stop here instead of reclassifying
    /// as out of scope.
    SourceTargetUnselected {
        call_sites: Box<[SourceExprSiteV1]>,
    },
    /// No bound item site belongs to the selected same-module static
    /// publication family, so no target relation can be co-sealed. Those
    /// calls need their own consumer contract; this is the unsupported
    /// family terminal, not a missing-evidence one.
    #[cfg(test)]
    SourceCallOutsideSelectedFamily {
        call_sites: Box<[SourceExprSiteV1]>,
    },
    /// A call-free coverage proof was issued by a different owner than the
    /// route token; foreign proof never satisfies the local inventory.
    SourceCoverageForeign,
    /// A call-free coverage proof binds a different loop site than the
    /// route token's exact parent.
    SourceCoverageSiteMismatch,
    /// Call evidence remains after a CallFree coverage claim — a residual
    /// probe row or a non-empty item inventory contradicts the proof.
    SourceCallResidualEvidence,
    SourceIdentityMissing,
    SourceParentMissing,
}

/// Source-item binding products live in the sibling `items` module; they are
/// re-exported here so existing `normal_callable_loop_source_route::Type`
/// paths keep resolving.
#[path = "normal_callable_loop_source_route_items.rs"]
mod items;

pub(in crate::mir::builder) use items::*;

/// One move-only source route token for the checked product and resolver forest.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopSourceRouteTokenV1 {
    owner: FunctionOwnerIdV1,
    parent_site: SourceNodeSiteV1,
    projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_coverage: Option<CallableLoopSourceCallCoverageV1>,
}

impl CallableLoopSourceRouteTokenV1 {
    pub(in crate::mir::builder) fn issue_checked_recipe_v2(
        owner: FunctionOwnerIdV1, parent_site: SourceNodeSiteV1, function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
        product: std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        items: Box<[CallableLoopSourceItemBindingV1]>, probe: CallableLoopSourceTargetProbeV1,
        call_free: Option<VerifiedCallableLoopCallFreeCoverageV1>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if !product.source.source.matches_identity_v1(owner, &parent_site, function_origin, source_kind) {
            return Err(CallableLoopSourceRouteRejectV1::ProjectionIdentityMismatch);
        }
        let mut token = Self::issue_source(owner, parent_site, function_origin, source_kind, projection)?;
        if call_free.as_ref().is_some_and(|coverage| !items.is_empty()
            || coverage.owner() != owner || coverage.loop_site().node() != &token.parent_site)
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceCallResidualEvidence);
        }
        CallableLoopSourceCallCoverageV1::validate_checked_recipe_v2(&product, owner, &token.parent_site, &items)?;
        probe.consume_for_checked_recipe_v2(&items, &product)?;
        token.source_items = items;
        token.source_coverage = Some(CallableLoopSourceCallCoverageV1::CheckedRecipe(product));
        Ok(token)
    }

    fn issue_source(owner: FunctionOwnerIdV1, parent_site: SourceNodeSiteV1, function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let projection = projection.ok_or(CallableLoopSourceRouteRejectV1::ProjectionMissing)?;
        if projection.owner() != owner {
            return Err(CallableLoopSourceRouteRejectV1::ProjectionOwnerMismatch);
        }
        let parent_stmt_site = SourceStmtSiteV1::from_node(parent_site.clone());
        if !projection.matches_source_identity(function_origin, source_kind, &parent_stmt_site) {
            return Err(CallableLoopSourceRouteRejectV1::ProjectionIdentityMismatch);
        }
        Ok(Self {
            owner,
            parent_site,
            projection,
            source_items: Box::new([]),
            source_coverage: None,
        })
    }

    /// Collect source item relations from the resolver ledger for one exact
    /// loop root. The target relation is intentionally supplied separately by
    /// the same-invocation target authority.
    #[cfg(test)]
    pub(in crate::mir::builder) fn source_items_for_loop(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        owner: FunctionOwnerIdV1,
        parent_site: &SourceNodeSiteV1,
    ) -> Result<Box<[CallableLoopSourceItemBindingV1]>, CallableLoopSourceRouteRejectV1> {
        let items = ledger
            .method_calls()
            .filter(|(site, _)| is_under_parent(site.node(), parent_site))
            .map(|(_, call)| CallableLoopSourceItemBindingV1::from_resolved(owner, call))
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        if items.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        }
        Ok(items)
    }

    /// Move the already co-sealed route product into its sole physical
    /// consumer. The retained recipe supplies the original call reach.
    pub(in crate::mir::builder) fn into_physical_parts(
        self,
    ) -> Result<
        (
            FunctionOwnerIdV1,
            SourceNodeSiteV1,
            VerifiedLoopCondBreakContinueSourceForestProjectionV1,
            Box<[CallableLoopSourceItemBindingV1]>,
        ),
        CallableLoopSourceRouteRejectV1,
    > {
        let Self {
            owner,
            parent_site,
            projection,
            source_items,
            source_coverage,
        } = self;
        let source_coverage =
            source_coverage.ok_or(CallableLoopSourceRouteRejectV1::SourceTargetMissing)?;
        match &source_coverage {
            CallableLoopSourceCallCoverageV1::CheckedRecipe(product) => {
                CallableLoopSourceCallCoverageV1::validate_checked_recipe_v2(product, owner, &parent_site, &source_items)?;
            }
        }
        Ok((
            owner,
            parent_site,
            projection,
            source_items,
        ))
    }
}

#[cfg(test)]
fn is_under_parent(site: &SourceNodeSiteV1, parent: &SourceNodeSiteV1) -> bool {
    site.segments().starts_with(parent.segments())
}

#[cfg(test)]
#[path = "normal_callable_loop_source_route_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "normal_callable_loop_scalar_result_tests.rs"]
mod scalar_result_tests;
