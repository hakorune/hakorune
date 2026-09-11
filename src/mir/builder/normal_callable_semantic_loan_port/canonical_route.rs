//! Canonical route probing for the normal callable adapter.
//!
//! This module only selects an already-issued source-bound product. It does
//! not issue source facts, repair targets, or own physical Builder state.

use crate::mir::builder::normal_callable_prepared_operation::PreparedCallableLoopOperationProgramV1;
use crate::mir::builder::normal_callable_semantic_source::{
    PreparedCallableLoopIngressV1, VerifiedNormalCallableSourceIngressReceiptV1,
};
use crate::mir::compiler::callable_single_loop_recipe_coseal::{
    issue_callable_single_loop_recipe_v1, CallableRecipeCoSealRejectV1,
};
use crate::mir::compiler::callable_single_loop_source_map::{
    issue_callable_single_loop_source_map_v1, CallableSourceMapRejectV1,
};
use crate::mir::compiler::callable_single_loop_source_shapes::SourceCallKindV1;
use crate::mir::compiler::callable_single_loop_syntax_facts::{
    issue_callable_single_loop_syntax_facts_from_ledger_v1, CallableSyntaxFactsRejectV1,
};
use crate::mir::compiler::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalGenericG0PlanV1, CanonicalLoweringPreflightV1,
};
use crate::mir::compiler::direct_accum_capability::probe_direct_accum_function_v1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::generic_g0_capability::probe_generic_g0_function_v1;
use crate::mir::loop_route_policy::GenericG0PolicyModeV1;
use crate::mir::compiler::CanonicalLoweringErrorV1;

/// A preflight result for the existing canonical callable consumers.
///
/// The enum is a route observation only; each successful variant is consumed
/// by an existing owner in the parent adapter.
pub(in crate::mir::builder) enum CanonicalCallableRouteV1<'source> {
    Ready(crate::mir::compiler::capability::CanonicalTrivialBindingSsaPlanV1<'source>),
    DirectAccum(crate::mir::compiler::direct_accum_profile::CanonicalDirectAccumPlanV1<'source>),
    CallableSingleLoop(PreparedCallableLoopOperationProgramV1<'source>),
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
    if let Some(program) = try_prepare_callable_single_loop_program_v1(input)? {
        return Ok(CanonicalCallableRouteV1::CallableSingleLoop(program));
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

pub(in crate::mir::builder) fn try_prepare_callable_single_loop_program_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<Option<PreparedCallableLoopOperationProgramV1<'_>>, String> {
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .map_err(|error| format!("[freeze:contract][callable-loop/source-ledger] {error:?}"))?;
    if ledger.loop_sites().count() != 1 {
        return Ok(None);
    }
    let syntax = match issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger) {
        Ok(syntax) => syntax,
        Err(error) if callable_syntax_shape_outside(error) => return Ok(None),
        Err(error) => {
            return Err(format!(
                "[freeze:contract][callable-loop/syntax-facts] {error:?}"
            ))
        }
    };
    let map = match issue_callable_single_loop_source_map_v1(&ledger, syntax) {
        Ok(map) => map,
        Err(error) if callable_source_map_shape_outside(error) => return Ok(None),
        Err(error) => {
            return Err(format!(
                "[freeze:contract][callable-loop/source-map] {error:?}"
            ))
        }
    };
    // This bounded CallableSingleLoop consumer currently has a resolver-issued
    // FreeStatic target, but no declared-instance target owner for Method
    // prefixes. Keep that shape explicitly outside this route.
    if map
        .prefix()
        .target()
        .prefix()
        .is_some_and(|(_, call, _)| matches!(call.kind(), SourceCallKindV1::Method(_)))
    {
        return Ok(None);
    }
    let product = match issue_callable_single_loop_recipe_v1(&ledger, map) {
        Ok(product) => product,
        Err(error) if callable_recipe_shape_outside(&error) => return Ok(None),
        Err(error) => return Err(format!("[freeze:contract][callable-loop/recipe] {error:?}")),
    };
    let source = VerifiedNormalCallableSourceIngressReceiptV1::from_resolved_input_v1(input)?;
    let prepared = PreparedCallableLoopIngressV1::from_source_v1(source, product)
        .map_err(|error| format!("[freeze:contract][callable-loop/source-context] {error:?}"))?
        .prepare_full_demand()
        .map_err(|error| format!("[freeze:contract][callable-loop/semantic-demand] {error:?}"))?;
    Ok(Some(prepared))
}

fn callable_syntax_shape_outside(error: CallableSyntaxFactsRejectV1) -> bool {
    matches!(
        error,
        CallableSyntaxFactsRejectV1::LoopCardinality
            | CallableSyntaxFactsRejectV1::LoopShape
            | CallableSyntaxFactsRejectV1::LoopBodyArity
            | CallableSyntaxFactsRejectV1::InitialCarrierShape
            | CallableSyntaxFactsRejectV1::DuplicateInitialCarrier
            | CallableSyntaxFactsRejectV1::PrefixBoundaryShape
            | CallableSyntaxFactsRejectV1::DuplicatePrefixBoundary
            | CallableSyntaxFactsRejectV1::ConditionShape
            | CallableSyntaxFactsRejectV1::ConditionRhsNotLiteral
            | CallableSyntaxFactsRejectV1::StepShape
            | CallableSyntaxFactsRejectV1::StepRhsNotLiteral
            | CallableSyntaxFactsRejectV1::StepTargetShape
            | CallableSyntaxFactsRejectV1::TailShape
            | CallableSyntaxFactsRejectV1::UnexpectedBodyStatement
    )
}

fn callable_source_map_shape_outside(error: CallableSourceMapRejectV1) -> bool {
    matches!(
        error,
        CallableSourceMapRejectV1::UnsupportedAssignmentTarget
            | CallableSourceMapRejectV1::UnsupportedLiteral(_)
            | CallableSourceMapRejectV1::UnsupportedOperator(_)
    )
}

fn callable_recipe_shape_outside(error: &CallableRecipeCoSealRejectV1) -> bool {
    matches!(
        error,
        CallableRecipeCoSealRejectV1::UnsupportedLiteral(_)
            | CallableRecipeCoSealRejectV1::UnsupportedOperator(_)
    )
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
