//! Resolved source-bound admission for the bounded Nested Predicate pilot.
//!
//! This is the first-family plan issuer only. It consumes the existing
//! caller-zero projection, Recipe/JoinSig/topology, and resolver effect claims
//! once, then leaves physical MIR ownership to the later canonical adapter.

use crate::ast::ASTNode;
use crate::mir::resolved_control_flow::verify_function_completion_v1;

use super::capability::{
    CanonicalFirstFamilyPlanBrandV1, CanonicalFirstFamilyPlanV1, ResolvedOwnerHeaderFamilyV1,
    ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::nested_predicate_effect_plan::{
    issue_nested_binding_execution_claims_v1, NestedBindingExecutionClaimsRejectV1,
    VerifiedNestedBindingExecutionClaimsV1,
};
use super::nested_predicate_producer::{
    produce_nested_predicate_recipe_v1, NestedPredicateRecipeProducerRejectV1,
};
use super::nested_predicate_projection::{
    issue_nested_predicate_source_projection_v1, NestedPredicateProjectionRejectV1,
};
use super::nested_predicate_topology::{
    issue_nested_predicate_physical_emission_input_v1, NestedPhysicalTopologyRejectV1,
    VerifiedNestedPhysicalEmissionInputV1,
};

/// One sealed resolved Nested Predicate plan. It carries no Builder, MIR ID,
/// SSA, PHI, or retry authority.
#[derive(Debug)]
pub(crate) struct CanonicalNestedPredicatePlanV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    claims: VerifiedNestedBindingExecutionClaimsV1,
    emission: VerifiedNestedPhysicalEmissionInputV1,
    completion: crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
}

impl<'source> CanonicalNestedPredicatePlanV1<'source> {
    pub(crate) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'source> {
        self.input
    }

    pub(crate) fn loop_stmt(&self) -> &LocatedStmtV1<'source> {
        &self.loop_stmt
    }

    pub(crate) fn claims(&self) -> &VerifiedNestedBindingExecutionClaimsV1 {
        &self.claims
    }

    pub(crate) fn emission(&self) -> &VerifiedNestedPhysicalEmissionInputV1 {
        &self.emission
    }

    pub(crate) fn completion(
        &self,
    ) -> &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1 {
        &self.completion
    }

    pub(crate) fn seal_resolved_owner_header_v1(
        &self,
    ) -> Result<VerifiedResolvedOwnerHeaderV1, ResolvedOwnerHeaderSealErrorV1> {
        VerifiedResolvedOwnerHeaderV1::seal_input(
            CanonicalFirstFamilyPlanBrandV1::from_family(
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa,
            ),
            self.input,
        )
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'source>,
        LocatedStmtV1<'source>,
        VerifiedNestedBindingExecutionClaimsV1,
        VerifiedNestedPhysicalEmissionInputV1,
        crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    ) {
        (
            self.input,
            self.loop_stmt,
            self.claims,
            self.emission,
            self.completion,
        )
    }
}

/// Result of the bounded source-unit probe. `NotCandidate` deliberately keeps
/// the existing DirectAccum probe available for clearly non-Nested shapes.
pub(crate) enum NestedPredicateSourceUnitProbeV1<'source> {
    NotCandidate,
    Candidate(CanonicalFirstFamilyPlanV1<'source>),
}

/// Probe Nested before DirectAccum. Both pilots share the top-level
/// `Local + Loop` envelope, so a Nested sentinel must win before DirectAccum
/// can reject or claim the source.
pub(crate) fn probe_nested_predicate_source_unit_v1<'source>(
    unit: &'source VerifiedResolvedSourceUnitV1,
) -> Result<NestedPredicateSourceUnitProbeV1<'source>, CanonicalLoweringErrorV1> {
    let input = unit.root_function_input()?;
    let body = input
        .source()
        .root_body()
        .map_err(|error| source_navigation(format!("{error:?}")))?;
    if body.statements().len() != 2 {
        return Ok(NestedPredicateSourceUnitProbeV1::NotCandidate);
    }
    let root_loop = input
        .source()
        .body_stmt(&body, 1)
        .map_err(|error| source_navigation(format!("{error:?}")))?;
    if !matches!(root_loop.node(), ASTNode::Loop { .. }) {
        return Ok(NestedPredicateSourceUnitProbeV1::NotCandidate);
    }

    // The resolved forest is the structural Nested sentinel. DirectAccum has
    // the same surface envelope but only one loop member.
    let Ok(forest) = input
        .function()
        .resolved_loop_source_forest(root_loop.site())
    else {
        return Ok(NestedPredicateSourceUnitProbeV1::NotCandidate);
    };
    if forest.members().len() != 2
        || forest.members()[0].parent_index().is_some()
        || forest.members()[1].parent_index() != Some(0)
    {
        return Ok(NestedPredicateSourceUnitProbeV1::NotCandidate);
    }

    let plan = issue_nested_predicate_plan_v1(input, root_loop)?;
    Ok(NestedPredicateSourceUnitProbeV1::Candidate(
        CanonicalFirstFamilyPlanV1::Loop(
            super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
        ),
    ))
}

fn issue_nested_predicate_plan_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
) -> Result<CanonicalNestedPredicatePlanV1<'source>, CanonicalLoweringErrorV1> {
    let projection = issue_nested_predicate_source_projection_v1(input, &loop_stmt)
        .map_err(|error| nested_projection_error(input, error))?;
    let product = produce_nested_predicate_recipe_v1(projection)
        .map_err(|error| nested_recipe_error(input, error))?;
    let claims =
        issue_nested_binding_execution_claims_v1(input.function(), product.source_handoff())
            .map_err(|error| nested_claim_error(input, error))?;
    let emission = issue_nested_predicate_physical_emission_input_v1(product)
        .map_err(|error| nested_topology_error(input, error))?;
    let completion = verify_function_completion_v1(input).map_err(|error| {
        CanonicalLoweringErrorV1::ResolvedFunctionCompletion {
            detail: format!("nested_predicate={error:?}"),
        }
    })?;
    if input.owner() != loop_stmt.owner()
        || completion.owner() != input.owner()
        || claims.prefix().owner() != input.owner()
        || claims.effect_plan().owner() != input.owner()
        || emission.topology().owner() != input.owner()
        || claims.prefix().frame_key() != emission.topology().root_frame_key()
        || claims.effect_plan().frame_key() != emission.topology().root_frame_key()
    {
        return Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            site: "nested_predicate".into(),
            actual: input.source().root().node_type(),
            reason: "nested_owner_or_frame_mismatch",
        });
    }
    Ok(CanonicalNestedPredicatePlanV1 {
        input,
        loop_stmt,
        claims,
        emission,
        completion,
    })
}

fn source_navigation(detail: impl Into<String>) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::SourceNavigation {
        detail: detail.into(),
    }
}

fn nested_projection_error(
    input: ResolvedFunctionLoweringInputV1<'_>,
    error: NestedPredicateProjectionRejectV1,
) -> CanonicalLoweringErrorV1 {
    unsupported(input, "nested_projection_rejected", format!("{error:?}"))
}

fn nested_recipe_error(
    input: ResolvedFunctionLoweringInputV1<'_>,
    error: NestedPredicateRecipeProducerRejectV1,
) -> CanonicalLoweringErrorV1 {
    unsupported(input, "nested_recipe_rejected", format!("{error:?}"))
}

fn nested_claim_error(
    input: ResolvedFunctionLoweringInputV1<'_>,
    error: NestedBindingExecutionClaimsRejectV1,
) -> CanonicalLoweringErrorV1 {
    unsupported(input, "nested_effect_claims_rejected", format!("{error:?}"))
}

fn nested_topology_error(
    input: ResolvedFunctionLoweringInputV1<'_>,
    error: NestedPhysicalTopologyRejectV1,
) -> CanonicalLoweringErrorV1 {
    unsupported(input, "nested_topology_rejected", format!("{error:?}"))
}

fn unsupported(
    input: ResolvedFunctionLoweringInputV1<'_>,
    reason: &'static str,
    detail: String,
) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::BuilderContract {
        detail: format!(
            "nested_predicate/{reason}: {detail}; root={}",
            input.source().root().node_type()
        ),
    }
}
