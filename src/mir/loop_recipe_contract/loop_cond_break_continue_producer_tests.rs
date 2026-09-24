#![cfg(test)]

use super::ids::{LoopItemKeyV1, LoopValueKeyV1};
use super::join_sig::{LoopJoinBranchArmV1, LoopJoinEdgeRoleV1};
use super::loop_cond_break_continue_producer::{
    produce_loop_cond_break_continue_recipe_v1, VerifiedLoopCondBreakContinueRecipeProductV1,
};
use super::schema::{LoopCompareI64OpV1, LoopConditionV1, LoopOperationV1, LoopRecipeItemV1};
use crate::mir::compiler::loop_cond_break_continue_projection::issue_loop_cond_break_continue_source_projection_v1;
use crate::mir::compiler::loop_cond_break_continue_typed_map::VerifiedLoopCondBreakContinueTypedSourceMapV1;
use crate::mir::compiler::loop_cond_break_continue_typed_map_issue::issue_loop_cond_break_continue_typed_source_map_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_route_policy::{
    freeze_loop_route_schedule_v1, issue_loop_cond_break_continue_policy_demand_v1,
    FrozenLoopRouteObservationV1, LoopGlobalEntryDispositionV1, LoopModeReleaseSnapshotV1,
    LoopReleaseAdmissionObservationV1, LoopRouteCandidateFactsV1, LoopRoutePolicyEvidenceV1,
    LoopRoutePolicySourceDeclineReasonV1, LoopRouteSourceDispositionV1,
    LoopRouteSuppressionDispositionV1, CANONICAL_LOOP_ROUTE_ORDER_V1,
};

pub(super) fn unit() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(crate::mir::compiler::loop_cond_function_for_test())
        .expect("fixture resolves")
}

pub(super) fn typed_map_for(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
) -> VerifiedLoopCondBreakContinueTypedSourceMapV1 {
    let body = input.source().root_body().expect("function body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("root loop");
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("root source");
    let projection =
        issue_loop_cond_break_continue_source_projection_v1(input, &loop_stmt, source)
            .expect("bounded source projection");
    issue_loop_cond_break_continue_typed_source_map_v1(input, projection)
        .expect("typed source map")
}

pub(super) fn target_cursor() -> usize {
    CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .position(|route| *route == LoopRouteId::LoopCondBreakContinue)
        .unwrap()
}

pub(super) fn schedule_with_winner(
    winner: Option<usize>,
) -> crate::mir::loop_route_policy::FrozenLoopRouteScheduleV1 {
    let observations = CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .enumerate()
        .map(|(index, _)| {
            FrozenLoopRouteObservationV1::new(
                LoopRouteSuppressionDispositionV1::Retained,
                LoopModeReleaseSnapshotV1::Release {
                    admission: LoopReleaseAdmissionObservationV1::Allowed,
                },
                LoopGlobalEntryDispositionV1::Allowed,
                LoopRouteSourceDispositionV1::Available,
                if Some(index) == winner {
                    LoopRoutePolicyEvidenceV1::Candidate(LoopRouteCandidateFactsV1::SourceAvailable)
                } else {
                    LoopRoutePolicyEvidenceV1::SourceDeclined(
                        LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
                    )
                },
            )
        })
        .collect::<Box<[_]>>();
    freeze_loop_route_schedule_v1(CANONICAL_LOOP_ROUTE_ORDER_V1.into(), observations).unwrap()
}

pub(super) fn demand_for(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
) -> crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyDemandV1 {
    issue_loop_cond_break_continue_policy_demand_v1(
        typed_map_for(input),
        schedule_with_winner(Some(target_cursor())),
    )
    .unwrap()
}

pub(crate) fn product() -> VerifiedLoopCondBreakContinueRecipeProductV1 {
    let unit = unit();
    let input = unit.root_function_input().unwrap();
    produce_loop_cond_break_continue_recipe_v1(demand_for(input), input.function()).unwrap()
}

#[test]
fn producer_emits_exact_verified_recipe_and_join_sig() {
    let product = product();
    let recipe = product.recipe().as_recipe();
    assert_eq!(recipe.loops.len(), 1);
    assert_eq!(recipe.blocks.len(), 4);
    assert_eq!(recipe.items.len(), 9);
    assert_eq!(recipe.bindings.len(), 1);
    assert_eq!(recipe.inputs, vec![LoopValueKeyV1::new(0)]);
    assert_eq!(recipe.values.len(), 7);
    assert_eq!(recipe.carriers.len(), 1);
    assert_eq!(recipe.exits.len(), 2);
    let LoopConditionV1::Predicate { block, value } = recipe.loops[0].condition else {
        panic!("LoopCond recipe must carry a predicate loop condition")
    };
    assert_eq!(block.raw(), 0);
    assert_eq!(value, LoopValueKeyV1::new(3));
    let LoopRecipeItemV1::Operation {
        operation:
            LoopOperationV1::CompareI64 {
                op,
                left,
                right,
                result,
            },
    } = &recipe.items[2].item
    else {
        panic!("loop predicate must be a compare")
    };
    assert_eq!(*op, LoopCompareI64OpV1::Less);
    assert_eq!(*left, LoopValueKeyV1::new(1));
    assert_eq!(*right, LoopValueKeyV1::new(2));
    assert_eq!(*result, LoopValueKeyV1::new(3));
    let LoopRecipeItemV1::Operation {
        operation:
            LoopOperationV1::CompareI64 {
                op,
                left,
                right,
                result,
            },
    } = &recipe.items[5].item
    else {
        panic!("branch predicate must be a compare")
    };
    assert_eq!(*op, LoopCompareI64OpV1::Equal);
    assert_eq!(*left, LoopValueKeyV1::new(4));
    assert_eq!(*right, LoopValueKeyV1::new(5));
    assert_eq!(*result, LoopValueKeyV1::new(6));
    assert!(matches!(
        recipe.items[6].item,
        LoopRecipeItemV1::If { .. }
    ));
    assert!(matches!(
        recipe.items[7].item,
        LoopRecipeItemV1::Exit { .. }
    ));
    assert_eq!(recipe.items[8].key, LoopItemKeyV1::new(8));

    let sig = product.join_sig().as_sig();
    assert_eq!(sig.branches.len(), 1);
    let roles = sig.loops[0]
        .edges
        .iter()
        .map(|edge| edge.role)
        .collect::<Vec<_>>();
    for expected in [
        LoopJoinEdgeRoleV1::Break,
        LoopJoinEdgeRoleV1::Continue,
        LoopJoinEdgeRoleV1::PredicateTrue,
        LoopJoinEdgeRoleV1::PredicateFalse,
    ] {
        assert!(
            roles.contains(&expected),
            "JoinSig must carry {:?} boundary, got {:?}",
            expected,
            roles
        );
    }
    let LoopJoinBranchArmV1::Exit(then_exit) = &sig.branches[0].then_arm else {
        panic!("then arm must be a direct exit");
    };
    let LoopJoinBranchArmV1::Exit(else_exit) = &sig.branches[0].else_arm else {
        panic!("else arm must be a direct exit");
    };
    assert_eq!(then_exit.payload, else_exit.payload);
}

#[test]
fn producer_is_deterministic_and_retains_policy_frame_receipt() {
    let first = product();
    let second = product();
    assert_eq!(first.recipe().as_recipe(), second.recipe().as_recipe());
    assert_eq!(first.join_sig().as_sig(), second.join_sig().as_sig());
    let unit = unit();
    let input = unit.root_function_input().unwrap();
    let frame = typed_map_for(input).root_frame_key().clone();
    assert!(first.policy_receipt().frame_key().matches(&frame));
}

#[test]
fn demand_rejects_schedule_without_loop_cond_candidate() {
    let unit = unit();
    let input = unit.root_function_input().unwrap();
    let result = issue_loop_cond_break_continue_policy_demand_v1(
        typed_map_for(input),
        schedule_with_winner(None),
    );
    assert!(result.is_err(), "no LoopCond candidate must not be admitted");
}

#[test]
fn demand_rejects_wrong_winner_cursor() {
    let unit = unit();
    let input = unit.root_function_input().unwrap();
    let wrong = (target_cursor() + 1) % CANONICAL_LOOP_ROUTE_ORDER_V1.len();
    let result = issue_loop_cond_break_continue_policy_demand_v1(
        typed_map_for(input),
        schedule_with_winner(Some(wrong)),
    );
    assert!(result.is_err(), "a different winning cursor must not seal");
}
