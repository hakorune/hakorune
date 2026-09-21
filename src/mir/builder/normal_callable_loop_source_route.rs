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
    target: Option<CanonicalSameModuleCallableKeyV1>,
    requirement: Option<CallableLoopSourceTargetRequirementV1>,
    core_methods: Box<[CallableLoopSourceItemBindingV1]>,
}

impl CallableLoopSourceTargetRelationV1 {
    pub(in crate::mir::builder) fn new(
        call_site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
        requirement: Option<CallableLoopSourceTargetRequirementV1>,
    ) -> Self {
        Self {
            call_site,
            target: Some(target),
            requirement,
            core_methods: Box::new([]),
        }
    }

    pub(in crate::mir::builder) fn core_methods(
        methods: Box<[CallableLoopSourceItemBindingV1]>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let Some(first) = methods.first() else {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        };
        Ok(Self {
            call_site: first.call_site().clone(),
            target: None,
            requirement: None,
            core_methods: methods,
        })
    }

    pub(in crate::mir::builder) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir::builder) fn target(&self) -> Option<&CanonicalSameModuleCallableKeyV1> {
        self.target.as_ref()
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

    pub(in crate::mir::builder) fn core_method_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        &self.core_methods
    }
}

/// One source-order disposition for a resolver-issued method item.  Static
/// publication and bound CoreMethod rows share one batch, while their
/// authorities remain distinct and are validated by their existing owners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopSourceItemDispositionV1 {
    SelectedStatic(CallableLoopSourceTargetRelationV1),
    CoreMethod(CallableLoopSourceItemBindingV1),
}

impl CallableLoopSourceItemDispositionV1 {
    pub(in crate::mir::builder) fn call_site(&self) -> &SourceExprSiteV1 {
        match self {
            Self::SelectedStatic(relation) => relation.call_site(),
            Self::CoreMethod(item) => item.call_site(),
        }
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
    /// Resolver-issued CoreMethod rows that cover the loop's method items.
    /// This is a distinct source family from static publication evidence.
    core_methods: Box<[CallableLoopSourceItemBindingV1]>,
}

impl CallableLoopSourceTargetProbeV1 {
    /// Empty probe: the loop is outside the selected family because the
    /// caller is not cataloged, the site is missing, or no item rows exist.
    pub(in crate::mir::builder) fn empty() -> Self {
        Self {
            selected: Box::new([]),
            uncovered: Box::new([]),
            requirement_mismatch: false,
            core_methods: Box::new([]),
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_parts(
        selected: Box<[CallableLoopSourceTargetRelationV1]>,
        uncovered: Box<[SourceExprSiteV1]>,
        requirement_mismatch: bool,
    ) -> Self {
        Self::from_parts_with_core_methods(selected, uncovered, requirement_mismatch, Box::new([]))
    }

    pub(in crate::mir::builder) fn from_parts_with_core_methods(
        selected: Box<[CallableLoopSourceTargetRelationV1]>,
        uncovered: Box<[SourceExprSiteV1]>,
        requirement_mismatch: bool,
        core_methods: Box<[CallableLoopSourceItemBindingV1]>,
    ) -> Self {
        Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
        }
    }

    /// Resolve the classification into the single co-sealed target relation.
    /// Evidence gaps are checked before selection arity so a dropped
    /// required row can never reclassify as out of scope.
    pub(in crate::mir::builder) fn into_selected_relation(
        self,
        source_items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<CallableLoopSourceTargetRelationV1, CallableLoopSourceRouteRejectV1> {
        let Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
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
        if let Some(relation) = selected.pop() {
            return Ok(relation);
        }
        if !core_methods.is_empty() {
            let covered = core_methods
                .iter()
                .map(|item| item.call_site())
                .collect::<std::collections::BTreeSet<_>>();
            let uncovered = source_items
                .iter()
                .filter(|item| !covered.contains(item.call_site()))
                .map(|item| item.call_site().clone())
                .collect::<Vec<_>>();
            if !uncovered.is_empty() {
                return Err(
                    CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                        call_sites: uncovered.into_boxed_slice(),
                    },
                );
            }
            return CallableLoopSourceTargetRelationV1::core_methods(core_methods);
        }
        Err(
            CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                call_sites: source_items
                    .iter()
                    .map(|item| item.call_site().clone())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
        )
    }

    /// Consume the complete ordered selected set for a composite source
    /// Recipe.  The direct route keeps `into_selected_relation`'s singleton
    /// contract; this batch route only accepts one selected handoff for every
    /// resolver-issued source item, in the same order.
    #[cfg(test)]
    pub(in crate::mir::builder) fn into_selected_relations(
        self,
        source_items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<Box<[CallableLoopSourceTargetRelationV1]>, CallableLoopSourceRouteRejectV1> {
        let Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
        } = self;
        if requirement_mismatch {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
        }
        if !uncovered.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetUnselected {
                call_sites: uncovered,
            });
        }
        if !core_methods.is_empty() {
            return Err(
                CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                    call_sites: core_methods
                        .iter()
                        .map(|item| item.call_site().clone())
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                },
            );
        }
        if selected.len() != source_items.len() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetCardinality {
                expected: source_items.len(),
                actual: selected.len(),
            });
        }
        for (item, relation) in source_items.iter().zip(selected.iter()) {
            if item.call_site() != relation.call_site() {
                return Err(CallableLoopSourceRouteRejectV1::SourceTargetOrderMismatch {
                    expected: item.call_site().clone(),
                    actual: relation.call_site().clone(),
                });
            }
            if !relation.has_exact_i64_requirement(&[1]) {
                return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
            }
        }
        Ok(selected)
    }

    /// Consume the complete source-order disposition batch for a structured
    /// source Recipe. Every resolver item is covered exactly once by either
    /// the selected publication owner or the existing CoreMethod owner.
    pub(in crate::mir::builder) fn into_item_dispositions(
        self,
        source_items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<Box<[CallableLoopSourceItemDispositionV1]>, CallableLoopSourceRouteRejectV1> {
        use std::collections::BTreeMap;

        let Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
        } = self;
        if requirement_mismatch {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
        }
        if !uncovered.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetUnselected {
                call_sites: uncovered,
            });
        }

        let mut selected_by_site = BTreeMap::new();
        for relation in selected {
            let call_site = relation.call_site().clone();
            if selected_by_site
                .insert(call_site.clone(), relation)
                .is_some()
            {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site });
            }
        }
        let mut core_by_site = BTreeMap::new();
        for item in core_methods {
            let call_site = item.call_site().clone();
            if core_by_site.insert(call_site.clone(), item).is_some() {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site });
            }
        }

        let mut seen_items = BTreeMap::new();
        let mut dispositions = Vec::with_capacity(source_items.len());
        for item in source_items {
            if seen_items.insert(item.call_site().clone(), ()).is_some() {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate {
                    call_site: item.call_site().clone(),
                });
            }
            if let Some(relation) = selected_by_site.remove(item.call_site()) {
                if !relation.has_exact_i64_requirement(&[1]) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
                }
                dispositions.push(CallableLoopSourceItemDispositionV1::SelectedStatic(
                    relation,
                ));
            } else if let Some(core_method) = core_by_site.remove(item.call_site()) {
                dispositions.push(CallableLoopSourceItemDispositionV1::CoreMethod(core_method));
            } else {
                return Err(
                    CallableLoopSourceRouteRejectV1::SourceItemDispositionMissing {
                        call_site: item.call_site().clone(),
                    },
                );
            }
        }

        let mut residual = selected_by_site
            .into_keys()
            .chain(core_by_site.into_keys())
            .collect::<Vec<_>>();
        if !residual.is_empty() {
            residual.sort();
            return Err(
                CallableLoopSourceRouteRejectV1::SourceItemDispositionResidual {
                    call_sites: residual.into_boxed_slice(),
                },
            );
        }
        Ok(dispositions.into_boxed_slice())
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
