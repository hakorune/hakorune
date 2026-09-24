//! Source-owned route token for the bounded LoopCond handoff.
//!
//! The planner remains the Facts/Recipe authority.  This module only co-seals
//! its already selected route with the resolver forest projection that the
//! callable scope owns.  It does not lower, allocate Builder state, or select
//! a compatibility fallback.

use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionOriginV1, FunctionOwnerIdV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopSourceRouteRejectV1 {
    FactsMissing,
    LoopCondFactsMissing,
    RouteNotExclusive {
        routes: Box<[LoopRouteId]>,
    },
    ProjectionMissing,
    ProjectionOwnerMismatch,
    ProjectionIdentityMismatch,
    SourceItemsMissing,
    SourceItemForeign,
    SourceItemOutsideLoop,
    SourceTargetMissing,
    SourceTargetSiteMismatch,
    SourceTargetMultiple,
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
    SourceItemDispositionMissing {
        call_site: SourceExprSiteV1,
    },
    SourceItemDispositionResidual {
        call_sites: Box<[SourceExprSiteV1]>,
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
    SourceCallOutsideSelectedFamily {
        call_sites: Box<[SourceExprSiteV1]>,
    },
    SourceIdentityMissing,
    SourceParentMissing,
}

/// Source-item binding products live in the sibling `items` module; they are
/// re-exported here so existing `normal_callable_loop_source_route::Type`
/// paths keep resolving.
#[path = "normal_callable_loop_source_route_items.rs"]
mod items;

pub(in crate::mir::builder) use items::*;

/// Data-only callable route match issued from canonical loop facts.
///
/// This product records which non-generic route predicates the retained
/// policy surface reports for one loop. It is not a scheduler: it owns no
/// entry order, execution, or fallback — consumers compare the matched set
/// against their sole accepted route. The GenericLoop routes are absent by
/// construction; their fact arms retired with the ordered registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopRouteMatchV1 {
    matched: Box<[LoopRouteId]>,
}

impl CallableLoopRouteMatchV1 {
    /// Evaluate the retained route predicates in canonical entry order.
    ///
    /// Suppression is unnecessary for the exclusivity checks the callable
    /// arms perform: a suppressed route's suppressor is itself a matched
    /// route, so `matched == [route]` holds exactly when the route is the
    /// sole surviving candidate.
    pub(in crate::mir::builder) fn issue(facts: &CanonicalLoopFacts) -> Self {
        use crate::mir::builder::control_flow::joinir::route_entry::registry::predicates::*;
        let mut matched = Vec::new();
        if pred_loop_break_recipe(facts) {
            matched.push(LoopRouteId::LoopBreakRecipe);
        }
        if pred_if_phi_join(facts) {
            matched.push(LoopRouteId::IfPhiJoin);
        }
        if pred_loop_continue_only(facts) {
            matched.push(LoopRouteId::LoopContinueOnly);
        }
        if pred_loop_true_early_exit(facts) {
            matched.push(LoopRouteId::LoopTrueEarlyExit);
        }
        if pred_loop_simple_while(facts) {
            matched.push(LoopRouteId::LoopSimpleWhile);
        }
        if pred_loop_char_map(facts) {
            matched.push(LoopRouteId::LoopCharMap);
        }
        if pred_loop_array_join(facts) {
            matched.push(LoopRouteId::LoopArrayJoin);
        }
        if pred_scan_with_init(facts) {
            matched.push(LoopRouteId::ScanWithInit);
        }
        if pred_split_scan(facts) {
            matched.push(LoopRouteId::SplitScan);
        }
        if pred_bool_predicate_scan(facts) {
            matched.push(LoopRouteId::BoolPredicateScan);
        }
        if pred_accum_const_loop(facts) {
            matched.push(LoopRouteId::AccumConstLoop);
        }
        if pred_nested_loop_minimal(facts) {
            matched.push(LoopRouteId::NestedLoopMinimal);
        }
        if pred_loop_true_break_continue(facts) {
            matched.push(LoopRouteId::LoopTrueBreakContinue);
        }
        if pred_loop_cond_break_continue(facts) {
            matched.push(LoopRouteId::LoopCondBreakContinue);
        }
        if pred_loop_cond_continue_only(facts) {
            matched.push(LoopRouteId::LoopCondContinueOnly);
        }
        if pred_loop_cond_continue_with_return(facts) {
            matched.push(LoopRouteId::LoopCondContinueWithReturn);
        }
        if pred_loop_cond_return_in_body(facts) {
            matched.push(LoopRouteId::LoopCondReturnInBody);
        }
        Self {
            matched: matched.into_boxed_slice(),
        }
    }

    /// Matched route candidates in canonical entry order.
    pub(in crate::mir::builder) fn matched_routes(&self) -> &[LoopRouteId] {
        &self.matched
    }
}

/// One move-only source route token.  The planner outcome and resolver forest
/// are consumed together; no later consumer may pair them by AST shape or
/// source line.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopSourceRouteTokenV1 {
    owner: FunctionOwnerIdV1,
    parent_site: SourceNodeSiteV1,
    outcome: PlanBuildOutcome,
    selection: CallableLoopRouteMatchV1,
    projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target: Option<CallableLoopSourceTargetRelationV1>,
}

impl CallableLoopSourceRouteTokenV1 {
    pub(in crate::mir::builder) fn issue(
        owner: FunctionOwnerIdV1,
        parent_site: SourceNodeSiteV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        outcome: PlanBuildOutcome,
        selection: CallableLoopRouteMatchV1,
        projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let facts = outcome
            .facts
            .as_ref()
            .ok_or(CallableLoopSourceRouteRejectV1::FactsMissing)?;
        if facts.facts.loop_cond_break_continue().is_none() {
            return Err(CallableLoopSourceRouteRejectV1::LoopCondFactsMissing);
        }
        if selection.matched_routes() != [LoopRouteId::LoopCondBreakContinue] {
            return Err(CallableLoopSourceRouteRejectV1::RouteNotExclusive {
                routes: selection.matched_routes().into(),
            });
        }
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
            outcome,
            selection,
            projection,
            source_items: Box::new([]),
            source_target: None,
        })
    }

    pub(in crate::mir::builder) fn issue_with_source_relations(
        owner: FunctionOwnerIdV1,
        parent_site: SourceNodeSiteV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        outcome: PlanBuildOutcome,
        selection: CallableLoopRouteMatchV1,
        projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target_probe: CallableLoopSourceTargetProbeV1,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if source_items.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        }
        if source_items
            .iter()
            .any(|item| !is_under_parent(item.call_site().node(), &parent_site))
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemOutsideLoop);
        }
        let source_target = source_target_probe.into_selected_relation(&source_items)?;
        if !source_items
            .iter()
            .any(|item| item.call_site() == source_target.call_site())
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetSiteMismatch);
        }
        let mut token = Self::issue(
            owner,
            parent_site,
            function_origin,
            source_kind,
            outcome,
            selection,
            projection,
        )?;
        token.source_items = source_items;
        token.source_target = Some(source_target);
        Ok(token)
    }

    /// Collect source item relations from the resolver ledger for one exact
    /// loop root. The target relation is intentionally supplied separately by
    /// the same-invocation target authority.
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

    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn parent_site(&self) -> &SourceNodeSiteV1 {
        &self.parent_site
    }

    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        &self.outcome
    }

    pub(in crate::mir::builder) fn selection(&self) -> &CallableLoopRouteMatchV1 {
        &self.selection
    }

    pub(in crate::mir::builder) fn projection(
        &self,
    ) -> &VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
        &self.projection
    }

    pub(in crate::mir::builder) fn source_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        &self.source_items
    }

    pub(in crate::mir::builder) fn source_target(
        &self,
    ) -> Option<&CallableLoopSourceTargetRelationV1> {
        self.source_target.as_ref()
    }

    /// Move the already co-sealed route product into its sole physical
    /// consumer.  The Facts/Recipe remains owned by `outcome`; callers must
    /// consume this tuple exactly once and may not reconstruct it from AST or
    /// source names.
    pub(in crate::mir::builder) fn into_physical_parts(
        self,
    ) -> Result<
        (
            FunctionOwnerIdV1,
            SourceNodeSiteV1,
            PlanBuildOutcome,
            CallableLoopRouteMatchV1,
            VerifiedLoopCondBreakContinueSourceForestProjectionV1,
            Box<[CallableLoopSourceItemBindingV1]>,
            CallableLoopSourceTargetRelationV1,
        ),
        CallableLoopSourceRouteRejectV1,
    > {
        let Self {
            owner,
            parent_site,
            outcome,
            selection,
            projection,
            source_items,
            source_target,
        } = self;
        let source_target =
            source_target.ok_or(CallableLoopSourceRouteRejectV1::SourceTargetMissing)?;
        if source_items.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        }
        if selection.matched_routes() != [LoopRouteId::LoopCondBreakContinue] {
            return Err(CallableLoopSourceRouteRejectV1::RouteNotExclusive {
                routes: selection.matched_routes().into(),
            });
        }
        Ok((
            owner,
            parent_site,
            outcome,
            selection,
            projection,
            source_items,
            source_target,
        ))
    }
}

fn is_under_parent(site: &SourceNodeSiteV1, parent: &SourceNodeSiteV1) -> bool {
    site.segments().starts_with(parent.segments())
}

#[cfg(test)]
#[path = "normal_callable_loop_source_route_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "normal_callable_loop_scalar_result_tests.rs"]
mod scalar_result_tests;
