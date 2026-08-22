//! Source-aware GenericLoop Facts/Recipe issuer.
//!
//! This module only transports one already-located callable Loop into the
//! existing planner/Facts authority. It does not lower, consume a ledger,
//! enter the route registry, or provide a fallback path.  Its only production
//! caller is the Ready branch in `raw_loop_child_entry`.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::registry::{
    select_recipe_first_routes, LocatedGenericLoopV1SelectionErrorV1, RecipeFirstRouteSelectionV1,
    VerifiedLocatedGenericLoopV1SelectionV1,
};
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::plan::features::generic_loop_body;
use crate::mir::builder::control_flow::plan::single_planner::{
    self, CallableLoopFactsPlannerInputV1,
};
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::control_flow::plan::GenericLoopV1Facts;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1, SourcePathSegmentV1};

#[cfg(test)]
use super::control_flow::joinir::structural_port::{
    issue_route_neutral_structural_seed, CallableLoopRouteNeutralStructuralSeedV1,
    CallableLoopSourceBoundStructuralPortV1, CallableLoopStructuralLeaseRejectV1,
};
use super::normal_callable_loop_handoff::{
    CallableSemanticLoopHandoffPreEffectReceiptV1, VerifiedCallableSemanticLoopBindingScheduleV1,
};
use super::raw_invocation_source_transport::RawInvocationSourceContextV1;
use super::raw_loop_child_entry::PreparedCallableGenericLoopSourceFactsPayloadV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsSourceErrorV1 {
    ParentNotLocated,
    ConditionNotLocated,
    BodyNotLocated,
    ForeignRootLineage,
    ParentSiteMismatch,
    ConditionSiteMismatch,
    BodySiteMismatch,
    OwnerMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsRouteErrorV1 {
    GenericLoopV1NotSelected,
    NonGenericOrOverlapping { routes: Box<[LoopRouteId]> },
}

#[derive(Debug)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsDispositionV1<'source> {
    SourceUnavailable(CallableGenericLoopSourceFactsSourceErrorV1),
    FactsAbsent,
    FactsRejected(Box<str>),
    RouteNotFrontSelected(CallableGenericLoopSourceFactsRouteErrorV1),
    Ready(CallableGenericLoopSourceFactsV1<'source>),
}

/// One move-only source-located Facts/Recipe outcome.
///
/// `PlanBuildOutcome` remains the existing Facts/Recipe authority. This
/// aggregate only co-seals it with the already-issued source schedule and the
/// exact route selection; it does not issue a new semantic binding or policy.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsV1<'source> {
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    schedule: VerifiedCallableSemanticLoopBindingScheduleV1,
    policy: GenericLoopFactsPolicyFrameV1,
    debug: bool,
    in_static_box: bool,
    outcome: PlanBuildOutcome,
    selection: RecipeFirstRouteSelectionV1,
    selected: VerifiedLocatedGenericLoopV1SelectionV1,
}

impl<'source> CallableGenericLoopSourceFactsV1<'source> {
    #[cfg(test)]
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn policy(&self) -> GenericLoopFactsPolicyFrameV1 {
        self.policy
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        &self.selection
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        &self.outcome
    }

    pub(in crate::mir::builder) fn claim_all(
        self,
    ) -> Result<
        CallableGenericLoopSourceFactsReceiptV1<'source>,
        CallableGenericLoopSourceFactsClaimErrorV1,
    > {
        let CallableGenericLoopSourceFactsV1 {
            owner,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            schedule,
            policy,
            debug,
            in_static_box,
            outcome,
            selection,
            selected,
        } = self;
        let parent_site = parent_source
            .site()
            .ok_or(CallableGenericLoopSourceFactsClaimErrorV1::ParentNotLocated)?;
        let condition_site = condition_source
            .site()
            .ok_or(CallableGenericLoopSourceFactsClaimErrorV1::ConditionNotLocated)?;
        let body_site = body_source
            .site()
            .ok_or(CallableGenericLoopSourceFactsClaimErrorV1::BodyNotLocated)?;
        let pre_effect = schedule
            .consume_pre_effect(parent_site, condition_site, body_site)
            .map_err(CallableGenericLoopSourceFactsClaimErrorV1::PreEffectRejected)?;
        Ok(CallableGenericLoopSourceFactsReceiptV1 {
            owner,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            pre_effect,
            policy,
            debug,
            in_static_box,
            outcome,
            selection,
            selected,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsClaimErrorV1 {
    ParentNotLocated,
    ConditionNotLocated,
    BodyNotLocated,
    PreEffectRejected(String),
}

/// One-shot source-facts claim receipt.  The pre-effect receipt is retained
/// here; it is not an observation that may be discarded at the old route.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsReceiptV1<'source> {
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    pre_effect: CallableSemanticLoopHandoffPreEffectReceiptV1,
    policy: GenericLoopFactsPolicyFrameV1,
    debug: bool,
    in_static_box: bool,
    outcome: PlanBuildOutcome,
    selection: RecipeFirstRouteSelectionV1,
    selected: VerifiedLocatedGenericLoopV1SelectionV1,
}

impl<'source> CallableGenericLoopSourceFactsReceiptV1<'source> {
    #[cfg(test)]
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        &self.pre_effect
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn policy(&self) -> GenericLoopFactsPolicyFrameV1 {
        self.policy
    }

    pub(in crate::mir::builder) fn into_semantic_recipe(
        self,
    ) -> Result<
        CallableGenericLoopV1SemanticRecipeV1<'source>,
        CallableGenericLoopV1SemanticRecipeRejectV1,
    > {
        CallableGenericLoopV1SemanticRecipeIssuerV1::issue(self)
    }
}

/// The single source-backed semantic Recipe owner for GenericLoopV1.
///
/// This wrapper owns the already-claimed source Facts receipt.  It does not
/// copy Facts, re-run the planner, select a route, or issue physical IDs.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopV1SemanticRecipeV1<'source> {
    receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopV1SemanticRecipeRejectV1 {
    FactsMissing,
    GenericFactsMissing,
    NestedLoopOutsideFirstCohort,
    BlockExprPreludeOutsideFirstCohort,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableGenericLoopV1SemanticRecipeViewRejectV1 {
    FactsMissing,
    GenericFactsMissing,
}

/// HRTB-bounded semantic view.  The borrowed fields all come from the one
/// claimed receipt and cannot escape the callback with a source lifetime.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopV1SemanticViewV1<'view> {
    owner: FunctionOwnerIdV1,
    loop_site: &'view SourceNodeSiteV1,
    pre_effect: &'view CallableSemanticLoopHandoffPreEffectReceiptV1,
    facts: &'view CanonicalLoopFacts,
    generic: &'view GenericLoopV1Facts,
    selection: &'view RecipeFirstRouteSelectionV1,
    selected: &'view VerifiedLocatedGenericLoopV1SelectionV1,
    debug: bool,
    in_static_box: bool,
}

impl CallableGenericLoopV1SemanticViewV1<'_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn loop_site(&self) -> &SourceNodeSiteV1 {
        self.loop_site
    }

    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        self.pre_effect
    }

    pub(in crate::mir::builder) fn facts(&self) -> &CanonicalLoopFacts {
        self.facts
    }

    pub(in crate::mir::builder) fn generic(&self) -> &GenericLoopV1Facts {
        self.generic
    }

    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        self.selection
    }

    pub(in crate::mir::builder) fn selected(&self) -> &VerifiedLocatedGenericLoopV1SelectionV1 {
        self.selected
    }

    pub(in crate::mir::builder) const fn debug(&self) -> bool {
        self.debug
    }

    pub(in crate::mir::builder) const fn in_static_box(&self) -> bool {
        self.in_static_box
    }
}

impl<'source> CallableGenericLoopV1SemanticRecipeV1<'source> {
    pub(in crate::mir::builder) fn with_view<R>(
        self,
        use_view: impl for<'view> FnOnce(CallableGenericLoopV1SemanticViewV1<'view>) -> R,
    ) -> Result<R, CallableGenericLoopV1SemanticRecipeViewRejectV1> {
        let Self { receipt } = self;
        let Some(facts) = receipt.outcome.facts.as_ref() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::FactsMissing);
        };
        let Some(generic) = facts.facts.generic_loop_v1() else {
            return Err(CallableGenericLoopV1SemanticRecipeViewRejectV1::GenericFactsMissing);
        };
        let view = CallableGenericLoopV1SemanticViewV1 {
            owner: receipt.owner,
            loop_site: receipt.pre_effect.loop_site(),
            pre_effect: &receipt.pre_effect,
            facts,
            generic,
            selection: &receipt.selection,
            selected: &receipt.selected,
            debug: receipt.debug,
            in_static_box: receipt.in_static_box,
        };
        Ok(use_view(view))
    }
}

pub(in crate::mir::builder) struct CallableGenericLoopV1SemanticRecipeIssuerV1;

impl CallableGenericLoopV1SemanticRecipeIssuerV1 {
    pub(in crate::mir::builder) fn issue<'source>(
        receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    ) -> Result<
        CallableGenericLoopV1SemanticRecipeV1<'source>,
        CallableGenericLoopV1SemanticRecipeRejectV1,
    > {
        let facts = receipt
            .outcome
            .facts
            .as_ref()
            .ok_or(CallableGenericLoopV1SemanticRecipeRejectV1::FactsMissing)?;
        let generic = facts
            .facts
            .generic_loop_v1()
            .ok_or(CallableGenericLoopV1SemanticRecipeRejectV1::GenericFactsMissing)?;
        if facts.nested_loop {
            return Err(CallableGenericLoopV1SemanticRecipeRejectV1::NestedLoopOutsideFirstCohort);
        }
        if generic_loop_body::body_has_blockexpr_prelude_loop(&generic.body.body) {
            return Err(
                CallableGenericLoopV1SemanticRecipeRejectV1::BlockExprPreludeOutsideFirstCohort,
            );
        }
        Ok(CallableGenericLoopV1SemanticRecipeV1 { receipt })
    }
}

/// Move-only source/structural handoff.  The seed is transport-only and the
/// source receipt remains the sole Facts/Recipe authority.
#[cfg(test)]
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableLoopStructuralHandoffV1<'source> {
    receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    seed: CallableLoopRouteNeutralStructuralSeedV1,
}

/// Opaque view borrowed only for one callback invocation.
#[cfg(test)]
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopReadyStructuralViewV1<'view> {
    owner: FunctionOwnerIdV1,
    loop_site: &'view SourceNodeSiteV1,
    pre_effect: &'view CallableSemanticLoopHandoffPreEffectReceiptV1,
    outcome: &'view PlanBuildOutcome,
    selection: &'view RecipeFirstRouteSelectionV1,
    selected: &'view VerifiedLocatedGenericLoopV1SelectionV1,
    port: CallableLoopSourceBoundStructuralPortV1<'view>,
}

#[cfg(test)]
impl CallableLoopReadyStructuralViewV1<'_> {
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn loop_site(&self) -> &SourceNodeSiteV1 {
        self.loop_site
    }

    pub(in crate::mir::builder) fn pre_effect(
        &self,
    ) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        self.pre_effect
    }

    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        self.outcome
    }

    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        self.selection
    }

    pub(in crate::mir::builder) fn selected(&self) -> &VerifiedLocatedGenericLoopV1SelectionV1 {
        self.selected
    }

    pub(in crate::mir::builder) fn structural_port(
        &self,
    ) -> &CallableLoopSourceBoundStructuralPortV1<'_> {
        &self.port
    }
}

/// Sole issuer for the caller-zero route-neutral structural lease.
#[cfg(test)]
pub(in crate::mir::builder) struct CallableLoopStructuralLeaseIssuerV1;

#[cfg(test)]
impl CallableLoopStructuralLeaseIssuerV1 {
    pub(in crate::mir::builder) fn prepare<'source>(
        receipt: CallableGenericLoopSourceFactsReceiptV1<'source>,
    ) -> Result<PreparedCallableLoopStructuralHandoffV1<'source>, CallableLoopStructuralLeaseRejectV1>
    {
        let seed = issue_route_neutral_structural_seed(
            receipt.owner,
            &receipt.parent_source,
            &receipt.condition_source,
            &receipt.body_source,
            &receipt.pre_effect,
        )?;
        Ok(PreparedCallableLoopStructuralHandoffV1 { receipt, seed })
    }
}

#[cfg(test)]
impl<'source> PreparedCallableLoopStructuralHandoffV1<'source> {
    /// Consume the handoff exactly once; the borrowed view cannot escape this
    /// higher-ranked callback and no physical effect occurs here.
    pub(in crate::mir::builder) fn with_view<R>(
        self,
        use_view: impl for<'view> FnOnce(CallableLoopReadyStructuralViewV1<'view>) -> R,
    ) -> R {
        let Self { receipt, seed } = self;
        let view = CallableLoopReadyStructuralViewV1 {
            owner: receipt.owner,
            loop_site: receipt.pre_effect.loop_site(),
            pre_effect: &receipt.pre_effect,
            outcome: &receipt.outcome,
            selection: &receipt.selection,
            selected: &receipt.selected,
            port: CallableLoopSourceBoundStructuralPortV1::from_seed(&seed),
        };
        use_view(view)
    }
}

pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsIssuerV1;

impl CallableGenericLoopSourceFactsIssuerV1 {
    /// Issue exactly one source-aware planner outcome.  The raw Ready branch
    /// is the sole production caller; no old-route fallback is owned here.
    pub(in crate::mir::builder) fn issue_once<'source>(
        payload: PreparedCallableGenericLoopSourceFactsPayloadV1<'source>,
    ) -> CallableGenericLoopSourceFactsDispositionV1<'source> {
        let PreparedCallableGenericLoopSourceFactsPayloadV1 {
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            owner,
            schedule,
            function_name,
            debug,
            in_static_box,
            policy,
        } = payload;

        if let Err(error) = validate_source_input(
            parent_source,
            &condition_source,
            &body_source,
            owner,
            &schedule,
        ) {
            return CallableGenericLoopSourceFactsDispositionV1::SourceUnavailable(error);
        }

        let planner_input =
            CallableLoopFactsPlannerInputV1::new(&condition, &body, policy, function_name, debug);
        let outcome = match single_planner::try_build_source_outcome(planner_input) {
            Ok(outcome) => outcome,
            Err(error) => {
                return CallableGenericLoopSourceFactsDispositionV1::FactsRejected(
                    error.into_boxed_str(),
                )
            }
        };
        if outcome.facts.is_none() {
            return CallableGenericLoopSourceFactsDispositionV1::FactsAbsent;
        }

        let selection = select_recipe_first_routes(outcome.facts.as_ref());
        let selected = match selection.verify_located_generic_loop_v1() {
            Ok(selected) => selected,
            Err(error) => {
                return CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
                    route_error(error),
                )
            }
        };

        CallableGenericLoopSourceFactsDispositionV1::Ready(CallableGenericLoopSourceFactsV1 {
            owner,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            schedule,
            policy,
            debug,
            in_static_box,
            outcome,
            selection,
            selected,
        })
    }
}

fn validate_source_input(
    parent_source: &RawInvocationSourceContextV1,
    condition_source: &RawInvocationSourceContextV1,
    body_source: &RawInvocationSourceContextV1,
    owner: FunctionOwnerIdV1,
    schedule: &VerifiedCallableSemanticLoopBindingScheduleV1,
) -> Result<(), CallableGenericLoopSourceFactsSourceErrorV1> {
    let parent_site = parent_source
        .site()
        .ok_or(CallableGenericLoopSourceFactsSourceErrorV1::ParentNotLocated)?;
    let condition_site = condition_source
        .site()
        .ok_or(CallableGenericLoopSourceFactsSourceErrorV1::ConditionNotLocated)?;
    let body_site = body_source
        .site()
        .ok_or(CallableGenericLoopSourceFactsSourceErrorV1::BodyNotLocated)?;
    if !parent_source.shares_root_lineage(condition_source)
        || !parent_source.shares_root_lineage(body_source)
    {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::ForeignRootLineage);
    }
    if schedule.loop_site() != parent_site {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::ParentSiteMismatch);
    }
    if schedule.owner() != owner {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::OwnerMismatch);
    }
    if !condition_source.is_exact_loop_condition()
        || !is_direct_child(
            parent_site,
            condition_site,
            SourcePathSegmentV1::LoopCondition,
        )
    {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::ConditionSiteMismatch);
    }
    if !body_source.is_exact_loop_body_root()
        || !is_direct_child(parent_site, body_site, SourcePathSegmentV1::LoopBodyRoot)
    {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::BodySiteMismatch);
    }
    Ok(())
}

fn is_direct_child(
    parent: &SourceNodeSiteV1,
    child: &SourceNodeSiteV1,
    expected: SourcePathSegmentV1,
) -> bool {
    let parent_segments = parent.segments();
    let child_segments = child.segments();
    child_segments.len() == parent_segments.len() + 1
        && child_segments.starts_with(parent_segments)
        && child_segments.last() == Some(&expected)
}

fn route_error(
    error: LocatedGenericLoopV1SelectionErrorV1,
) -> CallableGenericLoopSourceFactsRouteErrorV1 {
    match error {
        LocatedGenericLoopV1SelectionErrorV1::GenericLoopV1NotSelected => {
            CallableGenericLoopSourceFactsRouteErrorV1::GenericLoopV1NotSelected
        }
        LocatedGenericLoopV1SelectionErrorV1::NonGenericOrOverlappingSelection {
            raw_execution_routes,
        } => CallableGenericLoopSourceFactsRouteErrorV1::NonGenericOrOverlapping {
            routes: raw_execution_routes,
        },
    }
}

#[cfg(test)]
#[path = "normal_callable_loop_source_facts_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "normal_callable_loop_structural_lease_tests.rs"]
mod structural_lease_tests;
