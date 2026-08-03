//! Resolved, builder-free admission for the DirectAccum whole-function profile.
//!
//! This module joins existing policy/source/facts/Recipe capabilities. It does
//! not select routes, inspect a live Builder, or lower physical MIR.

use crate::mir::loop_recipe_contract::{
    produce_direct_accum_recipe_v1, DirectAccumRecipeProducerRejectV1,
    VerifiedDirectAccumRecipeProductV1,
};
use crate::mir::loop_route_policy::{
    DirectAccumRouteAdmissionRejectV1, VerifiedLoopPolicyWinnerV1,
};
use crate::mir::loop_structural_facts::{
    issue_selected_loop_recipe_demand_v1, DirectAccumBindingEffectEntryV1,
    DirectAccumBindingEffectRoleV1, DirectAccumStructuralShapeV1, SelectedLoopDemandRejectV1,
    VerifiedDirectAccumBindingEffectPlanV1, VerifiedLoopStructuralFactsV1,
};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::BindingRefV1;

use super::direct_accum_projection::{
    issue_direct_accum_facts_from_source_v1, DirectAccumProjectionRejectV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DirectAccumProfileRejectV1 {
    RouteAdmission(DirectAccumRouteAdmissionRejectV1),
    SourceLookup,
    Projection(DirectAccumProjectionRejectV1),
    Demand(SelectedLoopDemandRejectV1),
    MissingStructuralShape,
    Recipe(DirectAccumRecipeProducerRejectV1),
    CompletionOwnerMismatch,
}

/// One resolved DirectAccum profile handoff. The physicalizer consumes the
/// Recipe product and effect plan only after this whole row has been sealed.
#[derive(Debug)]
pub(crate) struct VerifiedDirectAccumProfileV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    recipe: VerifiedDirectAccumRecipeProductV1,
    effect_plan: VerifiedDirectAccumBindingEffectPlanV1,
    completion: VerifiedFunctionCompletionV1,
}

impl<'source> VerifiedDirectAccumProfileV1<'source> {
    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'source>,
        LocatedStmtV1<'source>,
        VerifiedDirectAccumRecipeProductV1,
        VerifiedDirectAccumBindingEffectPlanV1,
        VerifiedFunctionCompletionV1,
    ) {
        (
            self.input,
            self.loop_stmt,
            self.recipe,
            self.effect_plan,
            self.completion,
        )
    }
}

/// Co-seals one already-qualified policy winner with the resolved source,
/// DirectAccum facts, portable Recipe/JoinSig, effect plan, and completion.
pub(crate) fn admit_direct_accum_profile_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    winner: VerifiedLoopPolicyWinnerV1,
    completion: VerifiedFunctionCompletionV1,
) -> Result<VerifiedDirectAccumProfileV1<'source>, DirectAccumProfileRejectV1> {
    if completion.owner() != input.owner() {
        return Err(DirectAccumProfileRejectV1::CompletionOwnerMismatch);
    }
    let admission = winner
        .into_direct_accum_v1()
        .map_err(DirectAccumProfileRejectV1::RouteAdmission)?;
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
    let demand =
        issue_selected_loop_recipe_demand_v1(admission.into_policy_winner(), facts, source)
            .map_err(DirectAccumProfileRejectV1::Demand)?;
    let recipe =
        produce_direct_accum_recipe_v1(demand).map_err(DirectAccumProfileRejectV1::Recipe)?;
    Ok(VerifiedDirectAccumProfileV1 {
        input,
        loop_stmt,
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
        let source = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        let winner = crate::mir::loop_route_policy::issue_policy_winner_for_test_with_frame(
            10,
            &source.frame_key(),
        );
        let profile = admit_direct_accum_profile_v1(input, loop_stmt, winner, completion)
            .expect("DirectAccum profile");
        let (_input, _loop, _recipe, effect_plan, _completion) = profile.into_parts();
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
