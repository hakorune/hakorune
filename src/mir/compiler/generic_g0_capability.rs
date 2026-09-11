//! Production Generic G0 source capability issuer.
//!
//! This seam is the only compiler caller that turns the verified source-unit
//! marker into a Generic G0 observation and source parent. It does not lower
//! or publish anything; the later physical terminal must consume the exact
//! `CanonicalGenericG0PlanV1` without re-projecting source facts.

use crate::ast::ASTNode;
use crate::mir::loop_route_policy::{
    issue_generic_g0_candidate_v1, GenericG0PolicyContextV1, GenericG0PolicyModeV1,
    GenericG0PolicyOutcomeV1, GenericG0PolicyProfileV1,
};
use crate::mir::numeric_substrate::NumericTarget;

use super::capability::{CanonicalGenericG0PlanV1, CanonicalFirstFamilyPlanV1};
use super::direct_accum_capability::{
    probe_direct_accum_source_unit_v1, DirectAccumSourceUnitProbeV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_projection::handoff::{
    issue_generic_g0_policy_handoff_with_window_v1, GenericG0PolicyHandoffIssueV1,
};
use super::generic_g0_source_parent::{
    issue_generic_g0_source_parent_from_observation_v1, GenericG0SourceParentRejectV1,
};
use super::located::LocatedStmtV1;
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::nested_predicate_profile::{
    probe_nested_predicate_source_unit_v1, NestedPredicateSourceUnitProbeV1,
};

#[derive(Debug)]
pub(crate) enum GenericG0SourceUnitProbe<'source> {
    NotCandidate(ResolvedFunctionLoweringInputV1<'source>),
    Candidate(CanonicalFirstFamilyPlanV1<'source>),
}

pub(crate) fn verify_with_generic_g0_mode_v1(
    unit: &VerifiedResolvedSourceUnitV1,
    mode: Option<GenericG0PolicyModeV1>,
) -> Result<CanonicalFirstFamilyPlanV1<'_>, CanonicalLoweringErrorV1> {
    match probe_nested_predicate_source_unit_v1(unit)? {
        NestedPredicateSourceUnitProbeV1::Candidate(plan) => return Ok(plan),
        NestedPredicateSourceUnitProbeV1::NotCandidate => {}
    }
    match probe_direct_accum_source_unit_v1(unit)? {
        DirectAccumSourceUnitProbeV1::Candidate(plan) => Ok(plan),
        DirectAccumSourceUnitProbeV1::NotCandidate(_) => {
            match probe_generic_g0_source_unit_v1(unit, mode)? {
                GenericG0SourceUnitProbe::Candidate(plan) => Ok(plan),
                GenericG0SourceUnitProbe::NotCandidate(function) => {
                    super::capability::CanonicalLoweringPreflightV1::verify_function(function)
                }
            }
        }
    }
}

/// Probe the exact production G0 source shape after the existing nested and
/// direct loop probes have declined. A marked shape is never handed back to a
/// generic fallback: every failure below is a typed canonical rejection.
pub(crate) fn probe_generic_g0_source_unit_v1(
    unit: &VerifiedResolvedSourceUnitV1,
    mode: Option<GenericG0PolicyModeV1>,
) -> Result<GenericG0SourceUnitProbe<'_>, CanonicalLoweringErrorV1> {
    let input = unit.root_function_input()?;
    let Some((root_loop, _tail)) = generic_g0_root_marker(input)? else {
        return Ok(GenericG0SourceUnitProbe::NotCandidate(input));
    };
    let Some(mode) = mode else {
        return Err(CanonicalLoweringErrorV1::CapabilityNotActivated {
            boundary: "generic_g0_policy_mode",
        });
    };
    let window_lease = input
        .function()
        .issue_loop_family_window_lease_v1(root_loop.site())
        .map_err(|error| resolved_region_error(format!("window_lease={error:?}")))?;
    let handoff = issue_generic_g0_policy_handoff_with_window_v1(
        input,
        &window_lease,
        NumericTarget::host(),
    )
    .map_err(map_handoff_error)?;
    let context = GenericG0PolicyContextV1::from_observation(
        input.owner(),
        GenericG0PolicyProfileV1::G0,
        mode,
        crate::mir::loop_route_policy::GenericG0CoverageV1::Complete,
    );
    let observation = match issue_generic_g0_candidate_v1(handoff, context) {
        GenericG0PolicyOutcomeV1::Candidate(observation) => observation,
        GenericG0PolicyOutcomeV1::Unresolved(reason) => {
            return Err(resolved_region_error(format!("policy_unresolved={reason:?}")))
        }
        GenericG0PolicyOutcomeV1::Rejected(reason) => {
            return Err(resolved_region_error(format!("policy_rejected={reason:?}")))
        }
    };
    let parent = issue_generic_g0_source_parent_from_observation_v1(
        input,
        observation,
        window_lease,
    )
    .map_err(map_source_parent_error)?;
    Ok(GenericG0SourceUnitProbe::Candidate(
        CanonicalFirstFamilyPlanV1::Loop(
            super::capability::CanonicalLoopFamilyPlanV1::GenericG0(
                CanonicalGenericG0PlanV1::new(parent),
            ),
        ),
    ))
}

fn generic_g0_root_marker(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<Option<(LocatedStmtV1<'_>, LocatedStmtV1<'_>)>, CanonicalLoweringErrorV1> {
    let body = input
        .source()
        .root_body()
        .map_err(|error| resolved_region_error(format!("root_body={error:?}")))?;
    if body.statements().len() != 2 {
        return Ok(None);
    }
    let root_loop = input
        .source()
        .body_stmt(&body, 0)
        .map_err(|error| resolved_region_error(format!("root_loop={error:?}")))?;
    let tail = input
        .source()
        .body_stmt(&body, 1)
        .map_err(|error| resolved_region_error(format!("tail={error:?}")))?;
    if matches!(root_loop.node(), ASTNode::Loop { .. })
        && matches!(tail.node(), ASTNode::Return { .. })
    {
        Ok(Some((root_loop, tail)))
    } else {
        Ok(None)
    }
}

fn map_handoff_error(error: GenericG0PolicyHandoffIssueV1) -> CanonicalLoweringErrorV1 {
    resolved_region_error(format!("source_handoff={error:?}"))
}

fn map_source_parent_error(error: GenericG0SourceParentRejectV1) -> CanonicalLoweringErrorV1 {
    resolved_region_error(format!("source_parent={error:?}"))
}

fn resolved_region_error(detail: String) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::ResolvedRegionFlow { detail }
}
