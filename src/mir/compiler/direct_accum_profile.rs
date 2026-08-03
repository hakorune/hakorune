//! Resolved, builder-free admission for the DirectAccum whole-function profile.
//!
//! This module joins existing policy/source/facts/Recipe capabilities. It does
//! not select routes, inspect a live Builder, or lower physical MIR.

use crate::mir::loop_recipe_contract::{
    produce_direct_accum_recipe_v1, DirectAccumRecipeProducerRejectV1,
    VerifiedDirectAccumRecipeProductV1,
};
use crate::mir::loop_route_policy::{
    DirectAccumRouteAdmissionRejectV1, VerifiedDirectAccumPolicyHandoffV1,
    VerifiedDirectAccumPolicyReceiptV1, VerifiedLoopPolicyWinnerV1,
};
use crate::mir::loop_structural_facts::{
    issue_selected_loop_recipe_demand_v1, DirectAccumBindingEffectEntryV1,
    DirectAccumBindingEffectRoleV1, DirectAccumStructuralShapeV1, SelectedLoopDemandRejectV1,
    VerifiedDirectAccumBindingEffectPlanV1, VerifiedLoopStructuralFactsV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::BindingRefV1;

use super::direct_accum_prefix::{
    issue_direct_accum_prefix_input_v1, DirectAccumPrefixRejectV1, VerifiedDirectAccumPrefixInputV1,
};
use super::direct_accum_projection::{
    issue_direct_accum_facts_from_source_v1, DirectAccumProjectionRejectV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DirectAccumProfileRejectV1 {
    RouteAdmission(DirectAccumRouteAdmissionRejectV1),
    PolicyAdmission(DirectAccumRouteAdmissionRejectV1),
    SourceLookup,
    Projection(DirectAccumProjectionRejectV1),
    Demand(SelectedLoopDemandRejectV1),
    Prefix(DirectAccumPrefixRejectV1),
    MissingStructuralShape,
    Recipe(DirectAccumRecipeProducerRejectV1),
    CompletionOwnerMismatch,
}

/// Issue the resolved DirectAccum plan from one exact source loop. Source
/// projection, singleton observation, policy admission, and plan sealing all
/// happen once before any canonical session is opened.
pub(crate) fn issue_direct_accum_plan_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    completion: VerifiedFunctionCompletionV1,
) -> Result<CanonicalDirectAccumPlanV1<'source>, DirectAccumProfileRejectV1> {
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .map_err(|_| DirectAccumProfileRejectV1::SourceLookup)?;
    let prefix = issue_direct_accum_prefix_input_v1(input, &loop_stmt)
        .map_err(DirectAccumProfileRejectV1::Prefix)?;
    let facts = issue_direct_accum_facts_from_source_v1(input, &loop_stmt, &source)
        .map_err(DirectAccumProfileRejectV1::Projection)?;
    let observation = facts
        .into_direct_accum_singleton_observation_v1(source)
        .map_err(|_| DirectAccumProfileRejectV1::SourceLookup)?;
    let handoff = crate::mir::loop_route_policy::issue_direct_accum_route_admission_v1(observation)
        .map_err(DirectAccumProfileRejectV1::PolicyAdmission)?;
    issue_direct_accum_plan_from_handoff_v1(input, loop_stmt, handoff, prefix, completion)
}

/// One resolved DirectAccum whole-function plan. The physicalizer consumes the
/// Recipe product and effect plan only after this whole row has been sealed.
#[derive(Debug)]
pub(crate) struct CanonicalDirectAccumPlanV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    policy_receipt: VerifiedDirectAccumPolicyReceiptV1,
    prefix: VerifiedDirectAccumPrefixInputV1,
    recipe: VerifiedDirectAccumRecipeProductV1,
    effect_plan: VerifiedDirectAccumBindingEffectPlanV1,
    completion: VerifiedFunctionCompletionV1,
}

impl<'source> CanonicalDirectAccumPlanV1<'source> {
    pub(crate) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'source> {
        self.input
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'source>,
        LocatedStmtV1<'source>,
        VerifiedDirectAccumPolicyReceiptV1,
        VerifiedDirectAccumPrefixInputV1,
        VerifiedDirectAccumRecipeProductV1,
        VerifiedDirectAccumBindingEffectPlanV1,
        VerifiedFunctionCompletionV1,
    ) {
        (
            self.input,
            self.loop_stmt,
            self.policy_receipt,
            self.prefix,
            self.recipe,
            self.effect_plan,
            self.completion,
        )
    }
}

/// Consume the sealed source/policy handoff without reselecting or
/// reprojecting the loop. This is the only production-shaped plan ingress;
/// the winner-based helper below remains test-only parity evidence.
pub(crate) fn issue_direct_accum_plan_from_handoff_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    handoff: VerifiedDirectAccumPolicyHandoffV1,
    prefix: VerifiedDirectAccumPrefixInputV1,
    completion: VerifiedFunctionCompletionV1,
) -> Result<CanonicalDirectAccumPlanV1<'source>, DirectAccumProfileRejectV1> {
    if completion.owner() != input.owner() || loop_stmt.owner() != input.owner() {
        return Err(DirectAccumProfileRejectV1::CompletionOwnerMismatch);
    }
    let (admission, observation) = handoff.into_parts();
    let (winner, policy_receipt) = admission.into_parts();
    let (facts, source) = observation.into_parts();
    if !source.matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    ) {
        return Err(DirectAccumProfileRejectV1::SourceLookup);
    }
    let shape = facts
        .direct_accum_shape()
        .ok_or(DirectAccumProfileRejectV1::MissingStructuralShape)?;
    let effect_plan =
        VerifiedDirectAccumBindingEffectPlanV1::issue(input.owner(), source.frame_key(), shape);
    let demand = issue_selected_loop_recipe_demand_v1(winner, facts, source)
        .map_err(DirectAccumProfileRejectV1::Demand)?;
    let recipe =
        produce_direct_accum_recipe_v1(demand).map_err(DirectAccumProfileRejectV1::Recipe)?;
    Ok(CanonicalDirectAccumPlanV1 {
        input,
        loop_stmt,
        policy_receipt,
        prefix,
        recipe,
        effect_plan,
        completion,
    })
}

/// Co-seals one already-qualified policy winner with the resolved source,
/// DirectAccum facts, portable Recipe/JoinSig, effect plan, and completion.
#[cfg(test)]
pub(crate) fn admit_direct_accum_profile_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    winner: VerifiedLoopPolicyWinnerV1,
    completion: VerifiedFunctionCompletionV1,
) -> Result<CanonicalDirectAccumPlanV1<'source>, DirectAccumProfileRejectV1> {
    if completion.owner() != input.owner() {
        return Err(DirectAccumProfileRejectV1::CompletionOwnerMismatch);
    }
    let admission = winner
        .into_direct_accum_v1()
        .map_err(DirectAccumProfileRejectV1::RouteAdmission)?;
    let (winner, policy_receipt) = admission.into_parts();
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .map_err(|_| DirectAccumProfileRejectV1::SourceLookup)?;
    let facts = issue_direct_accum_facts_from_source_v1(input, &loop_stmt, &source)
        .map_err(DirectAccumProfileRejectV1::Projection)?;
    let shape = facts
        .direct_accum_shape()
        .ok_or(DirectAccumProfileRejectV1::MissingStructuralShape)?;
    let effect_plan =
        VerifiedDirectAccumBindingEffectPlanV1::issue(input.owner(), source.frame_key(), shape);
    let demand = issue_selected_loop_recipe_demand_v1(winner, facts, source)
        .map_err(DirectAccumProfileRejectV1::Demand)?;
    let recipe =
        produce_direct_accum_recipe_v1(demand).map_err(DirectAccumProfileRejectV1::Recipe)?;
    let prefix = issue_direct_accum_prefix_input_v1(input, &loop_stmt)
        .map_err(DirectAccumProfileRejectV1::Prefix)?;
    Ok(CanonicalDirectAccumPlanV1 {
        input,
        loop_stmt,
        policy_receipt,
        prefix,
        recipe,
        effect_plan,
        completion,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::loop_recipe_contract::LoopBindingKeyV1;
    use crate::mir::resolved_control_flow::verify_function_completion_v1;

    #[test]
    fn direct_accum_profile_seals_effect_witness() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
            .expect("DirectAccum fixture resolves");
        let input = unit.root_function_input().expect("function input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let completion = verify_function_completion_v1(input).expect("completion");
        let profile =
            issue_direct_accum_plan_v1(input, loop_stmt, completion).expect("DirectAccum plan");
        let (_input, _loop, _receipt, _prefix, _recipe, effect_plan, _completion) =
            profile.into_parts();
        assert_eq!(effect_plan.entries().len(), 5);
        let roles = effect_plan
            .entries()
            .iter()
            .map(DirectAccumBindingEffectEntryV1::role)
            .collect::<Vec<_>>();
        assert_eq!(
            roles.as_slice(),
            DirectAccumBindingEffectRoleV1::ALL.as_slice()
        );
        assert_eq!(
            effect_plan
                .entry(DirectAccumBindingEffectRoleV1::ConditionInductionRead)
                .recipe_binding(),
            LoopBindingKeyV1::new(0)
        );
        assert_eq!(
            effect_plan
                .entry(DirectAccumBindingEffectRoleV1::UpdateAccumulatorWrite)
                .recipe_binding(),
            LoopBindingKeyV1::new(1)
        );
    }

    #[test]
    fn direct_accum_profile_rejects_non_accum_policy_winner() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_accum_function_for_test())
            .expect("DirectAccum fixture resolves");
        let input = unit.root_function_input().expect("function input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let completion = verify_function_completion_v1(input).expect("completion");
        let source = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        let winner = crate::mir::loop_route_policy::issue_policy_winner_for_test_with_frame(
            4,
            &source.frame_key(),
        );
        assert!(matches!(
            admit_direct_accum_profile_v1(input, loop_stmt, winner, completion),
            Err(DirectAccumProfileRejectV1::RouteAdmission(
                DirectAccumRouteAdmissionRejectV1::WrongWinnerCursor { .. }
            ))
        ));
    }
}
