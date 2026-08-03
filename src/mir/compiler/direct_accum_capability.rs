//! Direct-only resolved preflight for the first Loop plan capability.
//!
//! This ingress is deliberately separate from the ordinary first-family
//! verifier. It accepts one exact source loop, seals the closed pilot prefix
//! and completion contract, then delegates to the source/policy plan issuer.
//! It never probes the route registry or falls through to Trivial/A+.

use crate::ast::{ASTNode, LiteralValue};

use super::capability::CanonicalFirstFamilyPlanV1;
use super::direct_accum_profile::{
    issue_direct_accum_plan_v1, CanonicalDirectAccumPlanV1, DirectAccumProfileRejectV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use crate::mir::resolved_control_flow::verify_function_completion_v1;

/// Source-unit probe result for the one production DirectAccum admission.
/// `NotCandidate` preserves the ordinary first-family verifier for unrelated
/// functions; once the closed `local; loop` source shape is present, any
/// DirectAccum failure is a typed terminal rejection rather than a fallback.
pub(crate) enum DirectAccumSourceUnitProbeV1<'source> {
    NotCandidate(ResolvedFunctionLoweringInputV1<'source>),
    Candidate(CanonicalFirstFamilyPlanV1<'source>),
}

pub(crate) fn probe_direct_accum_source_unit_v1<'source>(
    unit: &'source VerifiedResolvedSourceUnitV1,
) -> Result<DirectAccumSourceUnitProbeV1<'source>, CanonicalLoweringErrorV1> {
    let function = unit.root_function_input()?;
    let root = function.source().root();
    if !matches!(root, ASTNode::FunctionDeclaration { .. }) {
        return Ok(DirectAccumSourceUnitProbeV1::NotCandidate(function));
    }
    let body = function.source().root_body().map_err(source_navigation)?;
    if body.statements().len() != 2 {
        return Ok(DirectAccumSourceUnitProbeV1::NotCandidate(function));
    }
    let local = function
        .source()
        .body_stmt(&body, 0)
        .map_err(source_navigation)?;
    let loop_stmt = function
        .source()
        .body_stmt(&body, 1)
        .map_err(source_navigation)?;
    if !matches!(local.node(), ASTNode::Local { .. })
        || !matches!(loop_stmt.node(), ASTNode::Loop { .. })
    {
        return Ok(DirectAccumSourceUnitProbeV1::NotCandidate(function));
    }
    verify_direct_accum_first_family_function_v1(function, loop_stmt)
        .map(DirectAccumSourceUnitProbeV1::Candidate)
}

pub(crate) fn verify_direct_accum_function_v1<'source>(
    function: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
) -> Result<CanonicalDirectAccumPlanV1<'source>, CanonicalLoweringErrorV1> {
    let ASTNode::FunctionDeclaration {
        name,
        params,
        param_decls,
        uses,
        contracts,
        is_static,
        is_override,
        attrs,
        return_type_name,
        ..
    } = function.source().root()
    else {
        return unsupported("root", function.source().root(), "root_is_not_function");
    };
    if function.forest().owner_count() != 1
        || !function.forest().upvars().is_empty()
        || !*is_static
        || *is_override
        || name == "main"
        || !params.is_empty()
        || !param_decls.is_empty()
        || return_type_name.is_some()
        || !uses.is_empty()
        || !contracts.is_empty()
        || !attrs.is_empty()
    {
        return unsupported(
            "root",
            function.source().root(),
            "direct_accum_header_not_closed",
        );
    }
    if loop_stmt.owner() != function.owner() {
        return unsupported("loop", loop_stmt.node(), "direct_accum_loop_owner_mismatch");
    }
    let body = function.source().root_body().map_err(source_navigation)?;
    if body.statements().len() != 2 {
        return unsupported(
            "root_body",
            function.source().root(),
            "direct_accum_body_not_closed",
        );
    }
    let expected_loop = function
        .source()
        .body_stmt(&body, 1)
        .map_err(source_navigation)?;
    if expected_loop.site() != loop_stmt.site() || !matches!(loop_stmt.node(), ASTNode::Loop { .. })
    {
        return unsupported(
            "root_body/1",
            loop_stmt.node(),
            "direct_accum_loop_site_not_exact",
        );
    }
    let local = function
        .source()
        .body_stmt(&body, 0)
        .map_err(source_navigation)?;
    let ASTNode::Local {
        variables,
        initial_values,
        declared_type_names,
        ..
    } = local.node()
    else {
        return unsupported("root_body/0", local.node(), "direct_accum_local_required");
    };
    if variables.len() != 2
        || initial_values.len() != 2
        || declared_type_names.len() != 2
        || declared_type_names.iter().any(Option::is_some)
        || initial_values.iter().any(|initial| {
            !matches!(
                initial.as_deref(),
                Some(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    ..
                })
            )
        })
    {
        return unsupported(
            "root_body/0",
            local.node(),
            "direct_accum_local_not_zero_pair",
        );
    }
    let completion = verify_function_completion_v1(function).map_err(|error| {
        CanonicalLoweringErrorV1::ResolvedFunctionCompletion {
            detail: format!("{error:?}"),
        }
    })?;
    issue_direct_accum_plan_v1(function, loop_stmt, completion).map_err(|error| {
        let reason = match error {
            DirectAccumProfileRejectV1::CompletionOwnerMismatch => {
                "direct_accum_completion_owner_mismatch"
            }
            _ => "direct_accum_plan_rejected",
        };
        CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            site: "direct_accum".to_string(),
            actual: function.source().root().node_type(),
            reason,
        }
    })
}

/// Wrap the sealed direct-only plan in the compiler's single whole-unit
/// family sum. Ordinary first-family verification remains unchanged and does
/// not probe this Loop profile.
pub(crate) fn verify_direct_accum_first_family_function_v1<'source>(
    function: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
) -> Result<CanonicalFirstFamilyPlanV1<'source>, CanonicalLoweringErrorV1> {
    verify_direct_accum_function_v1(function, loop_stmt)
        .map(CanonicalFirstFamilyPlanV1::DirectAccum)
}

fn source_navigation(error: impl ToString) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::SourceNavigation {
        detail: error.to_string(),
    }
}

fn unsupported<T>(
    site: impl Into<String>,
    node: &ASTNode,
    reason: &'static str,
) -> Result<T, CanonicalLoweringErrorV1> {
    Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
        site: site.into(),
        actual: node.node_type(),
        reason,
    })
}
