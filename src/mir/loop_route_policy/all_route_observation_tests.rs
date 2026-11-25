use super::{
    generic_g0_source_unit_for_test, issue_all_route_observation_set_v1,
    issue_whole_unit_loop_coverage_proof_v1, LoopAllRouteObservationRowV1,
    LoopAllRouteObservationSetRejectV1, LoopRouteObservationOutcomeV1,
    LoopRoutePolicySourceDeclineReasonV1, LoopRouteRecipeBackingV1,
    VerifiedLoopAllRouteObservationSetV1, WholeUnitLoopCoverageProofV1,
    CANONICAL_LOOP_ROUTE_ORDER_V1,
};
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_recipe_contract::LoopRecipeProducerIdV1;
use crate::mir::resolved_semantics::VerifiedLoopFamilyWindowLeaseV1;

fn all_declined_rows() -> Box<[LoopAllRouteObservationRowV1]> {
    CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .map(|route| LoopAllRouteObservationRowV1 {
            route: *route,
            outcome: LoopRouteObservationOutcomeV1::PreEffectDeclined(
                LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
            ),
        })
        .collect()
}

fn rows_with_backed(
    backed_route: LoopRouteId,
    backing: LoopRouteRecipeBackingV1,
) -> Box<[LoopAllRouteObservationRowV1]> {
    CANONICAL_LOOP_ROUTE_ORDER_V1
        .iter()
        .map(|route| LoopAllRouteObservationRowV1 {
            route: *route,
            outcome: if *route == backed_route {
                LoopRouteObservationOutcomeV1::RecipeBacked(backing)
            } else {
                LoopRouteObservationOutcomeV1::PreEffectDeclined(
                    LoopRoutePolicySourceDeclineReasonV1::PreEffectDeclined,
                )
            },
        })
        .collect()
}

fn seal(
    rows: Box<[LoopAllRouteObservationRowV1]>,
) -> VerifiedLoopAllRouteObservationSetV1 {
    issue_all_route_observation_set_v1(rows).expect("typed 19-row set seals")
}

fn coverage(
    lease: &VerifiedLoopFamilyWindowLeaseV1,
    set: VerifiedLoopAllRouteObservationSetV1,
) -> WholeUnitLoopCoverageProofV1 {
    issue_whole_unit_loop_coverage_proof_v1(set, lease)
}

fn foreign_lease() -> VerifiedLoopFamilyWindowLeaseV1 {
    let unit = crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(
        crate::mir::compiler::nested_function_for_p3_test(),
    )
    .expect("foreign fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("root loop");
    input
        .function()
        .issue_loop_family_window_lease_v1(loop_stmt.site())
        .expect("foreign lease")
}

#[test]
fn all_declined_nineteen_row_set_seals() {
    let set = seal(all_declined_rows());
    assert_eq!(set.rows().len(), CANONICAL_LOOP_ROUTE_ORDER_V1.len());
    assert!(set.all_pre_effect_declined());
}

#[test]
fn attested_backed_routes_seal() {
    for (route, backing) in [
        (LoopRouteId::ScanWithInit, LoopRouteRecipeBackingV1::ScanWithInitV2),
        (
            LoopRouteId::AccumConstLoop,
            LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::DirectAccumV1),
        ),
        (
            LoopRouteId::NestedLoopMinimal,
            LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::NestedPredicateV1),
        ),
        (
            LoopRouteId::LoopTrueBreakContinue,
            LoopRouteRecipeBackingV1::PortableProducer(
                LoopRecipeProducerIdV1::LoopTrueBreakContinueV1,
            ),
        ),
        (
            LoopRouteId::LoopCondBreakContinue,
            LoopRouteRecipeBackingV1::PortableProducer(
                LoopRecipeProducerIdV1::LoopCondBreakContinueV1,
            ),
        ),
    ] {
        let set = seal(rows_with_backed(route, backing));
        assert!(!set.all_pre_effect_declined());
    }
}

#[test]
fn short_set_is_a_typed_reject() {
    let mut rows = all_declined_rows().into_vec();
    rows.pop();
    assert_eq!(
        issue_all_route_observation_set_v1(rows.into_boxed_slice()),
        Err(LoopAllRouteObservationSetRejectV1::RowCountMismatch {
            expected: CANONICAL_LOOP_ROUTE_ORDER_V1.len(),
            actual: CANONICAL_LOOP_ROUTE_ORDER_V1.len() - 1,
        })
    );
}

#[test]
fn out_of_order_route_is_a_typed_reject() {
    let mut rows = all_declined_rows().into_vec();
    rows.swap(0, 1);
    assert_eq!(
        issue_all_route_observation_set_v1(rows.into_boxed_slice()),
        Err(LoopAllRouteObservationSetRejectV1::RouteOrderMismatch {
            raw_cursor: 0,
            expected: LoopRouteId::LoopBreakRecipe,
            found: LoopRouteId::IfPhiJoin,
        })
    );
}

#[test]
fn unattested_backing_is_a_typed_reject() {
    assert_eq!(
        issue_all_route_observation_set_v1(rows_with_backed(
            LoopRouteId::IfPhiJoin,
            LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::DirectAccumV1),
        )),
        Err(LoopAllRouteObservationSetRejectV1::UnattestedRecipeBacking {
            route: LoopRouteId::IfPhiJoin,
        })
    );
    assert_eq!(
        issue_all_route_observation_set_v1(rows_with_backed(
            LoopRouteId::GenericLoopV0,
            LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::DirectAccumV1),
        )),
        Err(LoopAllRouteObservationSetRejectV1::UnattestedRecipeBacking {
            route: LoopRouteId::GenericLoopV0,
        })
    );
}

#[test]
fn second_backed_row_is_a_typed_reject() {
    let mut rows = rows_with_backed(
        LoopRouteId::AccumConstLoop,
        LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::DirectAccumV1),
    )
    .into_vec();
    rows[12].outcome = LoopRouteObservationOutcomeV1::RecipeBacked(
        LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::LoopTrueBreakContinueV1),
    );
    assert_eq!(
        issue_all_route_observation_set_v1(rows.into_boxed_slice()),
        Err(LoopAllRouteObservationSetRejectV1::MultipleRecipeBacked {
            first: LoopRouteId::AccumConstLoop,
            second: LoopRouteId::LoopTrueBreakContinue,
        })
    );
}

#[test]
fn coverage_proof_binds_the_lease_identity() {
    let unit = generic_g0_source_unit_for_test();
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let root_loop = input.source().body_stmt(&body, 0).expect("root loop");
    let lease = input
        .function()
        .issue_loop_family_window_lease_v1(root_loop.site())
        .expect("resolver lease");
    let proof = coverage(&lease, seal(all_declined_rows()));
    assert!(proof.matches_lease(&lease));
    assert!(!proof.matches_lease(&foreign_lease()));
    assert_eq!(proof.owner(), lease.owner());
    assert!(proof.observation_set().all_pre_effect_declined());
}
