//! Source-owned route token for the bounded LoopCond handoff.
//!
//! The planner remains the Facts/Recipe authority.  This module only co-seals
//! its already selected route with the resolver forest projection that the
//! callable scope owns.  It does not lower, allocate Builder state, or select
//! a compatibility fallback.

use crate::mir::builder::control_flow::joinir::route_entry::registry::RecipeFirstRouteSelectionV1;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    VerifiedCallableResultRepresentationV1, VerifiedStaticCallResultPublicationHandoffV1,
};
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
    RouteNotExclusive { routes: Box<[LoopRouteId]> },
    ProjectionMissing,
    ProjectionOwnerMismatch,
    ProjectionIdentityMismatch,
    SourceItemsMissing,
    SourceItemForeign,
    SourceItemOutsideLoop,
    SourceTargetMissing,
    SourceTargetSiteMismatch,
    SourceTargetMultiple,
    SourceTargetRequirementMismatch,
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

/// Resolver-owned source item relation retained for the future LoopCond
/// physical port. This copies only source sites and selector metadata; it
/// does not reopen AST dispatch or issue a Recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceItemBindingV1 {
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    result_site: SourceExprSiteV1,
    selector: Box<str>,
    arity: u32,
}

impl CallableLoopSourceItemBindingV1 {
    pub(in crate::mir::builder) fn from_resolved(
        owner: FunctionOwnerIdV1,
        call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if call.owner() != owner {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        let argument_sites = call
            .arguments()
            .iter()
            .map(|argument| argument.site().clone())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        if argument_sites.len() != call.arity() as usize {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        Ok(Self {
            call_site: call.site().clone(),
            receiver_site: call.receiver_site().clone(),
            argument_sites,
            result_site: call.result_site().clone(),
            selector: call.selector().into(),
            arity: call.arity(),
        })
    }

    pub(in crate::mir::builder) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir::builder) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(in crate::mir::builder) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(in crate::mir::builder) const fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(in crate::mir::builder) fn selector(&self) -> &str {
        &self.selector
    }

    pub(in crate::mir::builder) const fn arity(&self) -> u32 {
        self.arity
    }
}

/// Selected static-result requirement evidence copied from one publication
/// row.  This is a read-only copy of the sealed requirement shape; the owned
/// handoff itself stays with the publication owner for its sole physical
/// consumer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceTargetRequirementV1 {
    representation: VerifiedCallableResultRepresentationV1,
    required_i64_arguments: Box<[u32]>,
}

impl CallableLoopSourceTargetRequirementV1 {
    pub(in crate::mir::builder) fn from_handoff(
        handoff: &VerifiedStaticCallResultPublicationHandoffV1,
    ) -> Self {
        Self {
            representation: handoff.representation().clone(),
            required_i64_arguments: handoff.required_i64_arguments().to_vec().into_boxed_slice(),
        }
    }

    pub(in crate::mir::builder) const fn representation(
        &self,
    ) -> &VerifiedCallableResultRepresentationV1 {
        &self.representation
    }

    pub(in crate::mir::builder) fn required_i64_arguments(&self) -> &[u32] {
        &self.required_i64_arguments
    }
}

/// Exact target relation issued by the same invocation's source-target
/// authority. The route token never derives a target from selector text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceTargetRelationV1 {
    call_site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    requirement: Option<CallableLoopSourceTargetRequirementV1>,
}

impl CallableLoopSourceTargetRelationV1 {
    pub(in crate::mir::builder) fn new(
        call_site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
        requirement: Option<CallableLoopSourceTargetRequirementV1>,
    ) -> Self {
        Self {
            call_site,
            target,
            requirement,
        }
    }

    pub(in crate::mir::builder) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir::builder) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(in crate::mir::builder) fn requirement(
        &self,
    ) -> Option<&CallableLoopSourceTargetRequirementV1> {
        self.requirement.as_ref()
    }

    /// Whether the selected publication row for this site carries the exact
    /// i64 representation with the given required-argument ordinals.
    pub(in crate::mir::builder) fn has_exact_i64_requirement(&self, ordinals: &[u32]) -> bool {
        self.requirement.as_ref().is_some_and(|requirement| {
            requirement.representation() == &VerifiedCallableResultRepresentationV1::ExactI64
                && requirement.required_i64_arguments() == ordinals
        })
    }
}

/// Immutable obligation/evidence classification for one armed loop's source
/// items, produced by the module-port probe before route selection.
///
/// `ModuleLoweringPortV1::target_for_source` reads the inventory-derived
/// exact-target map — the non-consumable "this site is an exact same-module
/// static call" requirement fact. `selected_static_result_handoff_for_source`
/// only peeks the consumable selected row. The probe classifies and never
/// decides fatality: `issue_with_source_relations` maps this product to the
/// named LoopCond terminals, and every other route ignores it.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopSourceTargetProbeV1 {
    /// Exact-target sites whose selected publication row is still present,
    /// in item order.
    selected: Box<[CallableLoopSourceTargetRelationV1]>,
    /// Exact-target sites whose selected publication row is absent —
    /// target-only, missing, or already consumed obligations.
    uncovered: Box<[SourceExprSiteV1]>,
    /// A published handoff disagreed with its exact target relation.
    requirement_mismatch: bool,
}

impl CallableLoopSourceTargetProbeV1 {
    /// Empty probe: the loop is outside the selected family because the
    /// caller is not cataloged, the site is missing, or no item rows exist.
    pub(in crate::mir::builder) fn empty() -> Self {
        Self {
            selected: Box::new([]),
            uncovered: Box::new([]),
            requirement_mismatch: false,
        }
    }

    pub(in crate::mir::builder) fn from_parts(
        selected: Box<[CallableLoopSourceTargetRelationV1]>,
        uncovered: Box<[SourceExprSiteV1]>,
        requirement_mismatch: bool,
    ) -> Self {
        Self {
            selected,
            uncovered,
            requirement_mismatch,
        }
    }

    /// Resolve the classification into the single co-sealed target relation.
    /// Evidence gaps are checked before selection arity so a dropped
    /// required row can never reclassify as out of scope.
    fn into_selected_relation(
        self,
        source_items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<CallableLoopSourceTargetRelationV1, CallableLoopSourceRouteRejectV1> {
        let Self {
            selected,
            uncovered,
            requirement_mismatch,
        } = self;
        if requirement_mismatch {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
        }
        if !uncovered.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetUnselected {
                call_sites: uncovered,
            });
        }
        if selected.len() > 1 {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetMultiple);
        }
        let mut selected = Vec::from(selected);
        let Some(relation) = selected.pop() else {
            return Err(
                CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                    call_sites: source_items
                        .iter()
                        .map(|item| item.call_site().clone())
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                },
            );
        };
        Ok(relation)
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
    selection: RecipeFirstRouteSelectionV1,
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
        selection: RecipeFirstRouteSelectionV1,
        projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let facts = outcome
            .facts
            .as_ref()
            .ok_or(CallableLoopSourceRouteRejectV1::FactsMissing)?;
        if facts.facts.loop_cond_break_continue().is_none() {
            return Err(CallableLoopSourceRouteRejectV1::LoopCondFactsMissing);
        }
        if selection.raw_execution_routes() != [LoopRouteId::LoopCondBreakContinue] {
            return Err(CallableLoopSourceRouteRejectV1::RouteNotExclusive {
                routes: selection.raw_execution_routes().into(),
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
        selection: RecipeFirstRouteSelectionV1,
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

    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
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
            RecipeFirstRouteSelectionV1,
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
        if selection.raw_execution_routes() != [LoopRouteId::LoopCondBreakContinue] {
            return Err(CallableLoopSourceRouteRejectV1::RouteNotExclusive {
                routes: selection.raw_execution_routes().into(),
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
