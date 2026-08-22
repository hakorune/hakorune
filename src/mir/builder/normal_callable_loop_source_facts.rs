//! Caller-zero source-aware GenericLoop Facts/Recipe issuer.
//!
//! This module only transports one already-located callable Loop into the
//! existing planner/Facts authority. It does not lower, consume a ledger,
//! enter the route registry, or provide a fallback path.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::joinir::route_entry::registry::{
    select_recipe_first_routes, LocatedGenericLoopV1SelectionErrorV1, RecipeFirstRouteSelectionV1,
    VerifiedLocatedGenericLoopV1SelectionV1,
};
use crate::mir::builder::control_flow::plan::single_planner::{
    self, CallableLoopFactsPlannerInputV1,
};
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1, SourcePathSegmentV1};

use super::normal_callable_loop_handoff::VerifiedCallableSemanticLoopBindingScheduleV1;
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
    Ready(CallableGenericLoopSourceFactsReadyV1<'source>),
}

/// One move-only source-located Facts/Recipe outcome.
///
/// `PlanBuildOutcome` remains the existing Facts/Recipe authority. This
/// aggregate only co-seals it with the already-issued source schedule and the
/// exact route selection; it does not issue a new semantic binding or policy.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsReadyV1<'source> {
    owner: FunctionOwnerIdV1,
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    schedule: VerifiedCallableSemanticLoopBindingScheduleV1,
    policy: GenericLoopFactsPolicyFrameV1,
    outcome: PlanBuildOutcome,
    selection: RecipeFirstRouteSelectionV1,
    selected: VerifiedLocatedGenericLoopV1SelectionV1,
}

impl<'source> CallableGenericLoopSourceFactsReadyV1<'source> {
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
}

/// A terminal-only state transition for the caller-zero seam.
///
/// This keeps only the already-issued source schedule and exact route seal. The
/// AST, Facts, and Recipe are intentionally dropped at this terminal; no
/// later normalizer, registry suffix, or physical consumer can observe them
/// through this product.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsConsumedV1 {
    schedule: VerifiedCallableSemanticLoopBindingScheduleV1,
    selected: VerifiedLocatedGenericLoopV1SelectionV1,
}

impl CallableGenericLoopSourceFactsConsumedV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.schedule.owner()
    }
}

/// Sole named consumer for the terminal-only P0.
pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsTerminalConsumerV1;

impl CallableGenericLoopSourceFactsTerminalConsumerV1 {
    pub(in crate::mir::builder) fn consume<'source>(
        ready: CallableGenericLoopSourceFactsReadyV1<'source>,
    ) -> CallableGenericLoopSourceFactsConsumedV1 {
        let CallableGenericLoopSourceFactsReadyV1 {
            schedule, selected, ..
        } = ready;
        CallableGenericLoopSourceFactsConsumedV1 { schedule, selected }
    }
}

pub(in crate::mir::builder) struct CallableGenericLoopSourceFactsIssuerV1;

impl CallableGenericLoopSourceFactsIssuerV1 {
    /// Issue exactly one source-aware planner outcome. This is intentionally a
    /// disconnected P0 seam; no production caller may invoke it yet.
    pub(in crate::mir::builder) fn issue_once<'source>(
        payload: PreparedCallableGenericLoopSourceFactsPayloadV1<'source>,
    ) -> CallableGenericLoopSourceFactsDispositionV1<'source> {
        let (
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            owner,
            schedule,
            function_name,
            debug,
            _in_static_box,
            policy,
        ) = payload.into_parts();

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

        CallableGenericLoopSourceFactsDispositionV1::Ready(CallableGenericLoopSourceFactsReadyV1 {
            owner,
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            schedule,
            policy,
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
