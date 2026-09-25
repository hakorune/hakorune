//! Source-aware callable loop Facts/Recipe issuer.
//!
//! This module only transports one already-located callable Loop into the
//! existing planner/Facts authority. It does not lower, consume a ledger,
//! select a route, or provide a fallback path.  Its only production caller is
//! the Ready branch in `raw_loop_child_entry`.
//!
//! The retained callable families are LoopCond and LoopTrue. The ordered
//! registry's GenericLoop arm retired with the route scheduler; unmatched
//! profiles end at `RouteNotFrontSelected` instead of a second lowering path.

use crate::mir::builder::control_flow::plan::single_planner::{
    self, CallableLoopFactsPlannerInputV1,
};
use crate::mir::builder::normal_callable_loop_handoff::CallableLoopReadyBodyOnlyProductV1;
use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopRouteMatchV1, CallableLoopSoleFamilyV1,
};
use crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceRouteRejectV1;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::builder::raw_loop_child_entry::PreparedCallableGenericLoopSourceFactsPayloadV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1, SourcePathSegmentV1};

use super::loop_cond;
use super::loop_cond::CallableLoopCondSourceFactsV1;
#[allow(unused_imports)]
pub(in crate::mir::builder) use super::loop_cond::SourceLoopCondPhysicalInputV1;
use super::loop_true;
use super::loop_true::CallableLoopTrueSourceFactsV1;
#[allow(unused_imports)]
pub(in crate::mir::builder) use super::loop_true::SourceLoopTruePhysicalInputV1;

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
    NonGenericOrOverlapping {
        routes: Box<[LoopRouteId]>,
    },
    VariableAccumRecurrenceSourceUnavailable(Box<str>),
    VariableAccumRecurrenceSiteMismatch,
    VariableAccumRecurrenceUnresolved(
        crate::mir::loop_structural_facts::VariableAccumRecurrenceSourceUnresolvedV1,
    ),
    VariableAccumRecurrenceRejected(
        crate::mir::loop_structural_facts::VariableAccumRecurrenceSourceRejectV1,
    ),
    VariableAccumRecurrenceOverlap {
        routes: Box<[LoopRouteId]>,
    },
    VariableAccumRecurrenceProducer(Box<str>),
    LoopCondRouteRejected(CallableLoopSourceRouteRejectV1),
    LoopTrueRouteRejected(CallableLoopSourceRouteRejectV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) enum CallableGenericLoopSourceFactsDispositionV1<'source> {
    SourceUnavailable(CallableGenericLoopSourceFactsSourceErrorV1),
    FactsAbsent,
    FactsRejected(Box<str>),
    RouteNotFrontSelected(CallableGenericLoopSourceFactsRouteErrorV1),
    VariableAccumRecurrenceReady(
        crate::mir::loop_recipe_contract::VerifiedVariableAccumRecurrenceRecipeProductV1,
    ),
    LoopCondReady(CallableLoopCondSourceFactsV1<'source>),
    LoopTrueReady(CallableLoopTrueSourceFactsV1<'source>),
}

#[path = "generic/issuer.rs"]
mod issuer;
pub(in crate::mir::builder) use issuer::issue_callable_variable_accum_recurrence;
#[allow(unused_imports)]
pub(in crate::mir::builder) use issuer::CallableGenericLoopSourceFactsIssuerV1;

fn validate_source_input(
    parent_source: &RawInvocationSourceContextV1,
    condition_source: &RawInvocationSourceContextV1,
    body_source: &RawInvocationSourceContextV1,
    owner: FunctionOwnerIdV1,
    binding_product: &CallableLoopReadyBodyOnlyProductV1,
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
    if binding_product.loop_site() != parent_site {
        return Err(CallableGenericLoopSourceFactsSourceErrorV1::ParentSiteMismatch);
    }
    if binding_product.owner() != owner {
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
