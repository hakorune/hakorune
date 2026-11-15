//! Canonical route probing for the normal callable adapter.
//!
//! This module only selects an already-issued source-bound product. It does
//! not issue source facts, repair targets, or own physical Builder state.

use crate::mir::compiler::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalGenericG0PlanV1, CanonicalLoweringPreflightV1,
};
use crate::mir::compiler::direct_accum_capability::probe_direct_accum_function_v1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::generic_g0_capability::probe_generic_g0_function_v1;
use crate::mir::compiler::CanonicalLoweringErrorV1;
use crate::mir::loop_route_policy::GenericG0PolicyModeV1;

/// A preflight result for the existing canonical callable consumers.
///
/// The enum is a route observation only; each successful variant is consumed
/// by an existing owner in the parent adapter.
pub(in crate::mir::builder) enum CanonicalCallableRouteV1<'source> {
    Ready(crate::mir::compiler::capability::CanonicalTrivialBindingSsaPlanV1<'source>),
    DirectAccum(crate::mir::compiler::direct_accum_profile::CanonicalDirectAccumPlanV1<'source>),
    GenericG0(CanonicalGenericG0PlanV1<'source>),
    Outside,
}

pub(in crate::mir::builder) fn classify_canonical_callable_route(
    input: ResolvedFunctionLoweringInputV1<'_>,
    mode: Option<GenericG0PolicyModeV1>,
) -> Result<CanonicalCallableRouteV1<'_>, String> {
    if let Some(plan) = probe_direct_accum_function_v1(input).map_err(|error| {
        format!("[freeze:contract][mir/callable-direct-accum-preflight] {error:?}")
    })? {
        if let CanonicalFirstFamilyPlanV1::Loop(
            crate::mir::compiler::capability::CanonicalLoopFamilyPlanV1::DirectAccum(plan),
        ) = plan
        {
            return Ok(CanonicalCallableRouteV1::DirectAccum(plan));
        }
        return Err(
            "[freeze:contract][mir/callable-direct-accum-preflight] unexpected plan family"
                .to_owned(),
        );
    }
    if let Some(route) = try_classify_generic_g0_route_v1(input, mode)? {
        return Ok(route);
    }
    match CanonicalLoweringPreflightV1::verify_function(input) {
        Ok(CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan)) => {
            Ok(CanonicalCallableRouteV1::Ready(plan))
        }
        Ok(_) => Ok(CanonicalCallableRouteV1::Outside),
        Err(error) if is_canonical_shape_outside(&error) => Ok(CanonicalCallableRouteV1::Outside),
        Err(error) => Err(format!(
            "[freeze:contract][mir/callable-canonical-preflight] {error:?}"
        )),
    }
}

pub(in crate::mir::builder) fn try_classify_generic_g0_route_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    mode: Option<GenericG0PolicyModeV1>,
) -> Result<Option<CanonicalCallableRouteV1<'_>>, String> {
    probe_generic_g0_function_v1(input, mode)
        .map(|plan| plan.map(CanonicalCallableRouteV1::GenericG0))
        .map_err(|error: CanonicalLoweringErrorV1| {
            format!("[freeze:contract][mir/callable-generic-g0-preflight] {error:?}")
        })
}

fn is_canonical_shape_outside(error: &CanonicalLoweringErrorV1) -> bool {
    matches!(
        error,
        CanonicalLoweringErrorV1::UnsupportedCanonicalOwnerKind
            | CanonicalLoweringErrorV1::UnsupportedCanonicalSyntaxKind
            | CanonicalLoweringErrorV1::UnsupportedCanonicalControlRoute
            | CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape { .. }
    )
}
