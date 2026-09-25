use super::family_admission_tests::{all_declined, candidate_fixture, fixture};
use super::{
    assemble_loop_family_admission_window_v1, issue_all_route_observation_set_v1,
    issue_whole_unit_loop_coverage_proof_v1, select_canonical_loop_family_v1,
    CanonicalLoopFamilySelectionOutcomeV1, CanonicalLoopFamilySelectionReasonV1,
    LoopAllRouteObservationRowV1, LoopAllRouteObservationSetRejectV1,
    LoopFamilyAdmissionAssemblyOutcomeV1, LoopRouteObservationOutcomeV1,
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
        (
            LoopRouteId::LoopSimpleWhile,
            LoopRouteRecipeBackingV1::PortableProducer(
                LoopRecipeProducerIdV1::VariableAccumRecurrenceV1,
            ),
        ),
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
    let (lease, _identity) = fixture();
    let proof = coverage(&lease, seal(all_declined_rows()));
    assert!(proof.matches_lease(&lease));
    assert!(proof.observation_set().all_pre_effect_declined());
}

#[test]
fn no_candidate_opens_only_with_all_declined_coverage() {
    let (lease, identity) = fixture();
    let proof = coverage(&lease, seal(all_declined_rows()));
    let window = match assemble_loop_family_admission_window_v1(lease, all_declined(&identity)) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("five declined rows must be ready for selector"),
    };
    match select_canonical_loop_family_v1(window, proof) {
        CanonicalLoopFamilySelectionOutcomeV1::NoCandidate(proof) => {
            assert!(proof.observation_set().all_pre_effect_declined());
        }
        _ => panic!("all-declined window plus all-declined coverage must be NoCandidate"),
    }
}

#[test]
fn backed_set_without_window_candidate_rejects_as_contradiction() {
    let (lease, identity) = fixture();
    let proof = coverage(
        &lease,
        seal(rows_with_backed(
            LoopRouteId::AccumConstLoop,
            LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::DirectAccumV1),
        )),
    );
    let window = match assemble_loop_family_admission_window_v1(lease, all_declined(&identity)) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("five declined rows must be ready for selector"),
    };
    match select_canonical_loop_family_v1(window, proof) {
        CanonicalLoopFamilySelectionOutcomeV1::Rejected(failure) => {
            assert_eq!(
                failure.reason(),
                CanonicalLoopFamilySelectionReasonV1::CoverageBackedWithoutCandidate
            );
        }
        _ => panic!("backed coverage without a window candidate must reject"),
    }
}

#[test]
fn foreign_coverage_identity_rejects() {
    let (lease, identity, candidate) = candidate_fixture();
    let foreign_lease = foreign_lease();
    let proof = coverage(&foreign_lease, seal(all_declined_rows()));
    let mut rows = all_declined(&identity).into_vec();
    rows[2] = candidate;
    let window = match assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice()) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("one candidate plus four declines must be ready"),
    };
    match select_canonical_loop_family_v1(window, proof) {
        CanonicalLoopFamilySelectionOutcomeV1::Rejected(failure) => {
            assert_eq!(
                failure.reason(),
                CanonicalLoopFamilySelectionReasonV1::CoverageIdentityMismatch
            );
        }
        _ => panic!("foreign coverage identity must reject"),
    }
}

#[test]
fn selected_window_retains_the_coverage_proof() {
    let (lease, identity, candidate) = candidate_fixture();
    let proof = coverage(
        &lease,
        seal(rows_with_backed(
            LoopRouteId::AccumConstLoop,
            LoopRouteRecipeBackingV1::PortableProducer(LoopRecipeProducerIdV1::DirectAccumV1),
        )),
    );
    let owner = identity.owner;
    let mut rows = all_declined(&identity).into_vec();
    rows[2] = candidate;
    let window = match assemble_loop_family_admission_window_v1(lease, rows.into_boxed_slice()) {
        LoopFamilyAdmissionAssemblyOutcomeV1::Ready(window) => window,
        _ => panic!("one candidate plus four declines must be ready"),
    };
    match select_canonical_loop_family_v1(window, proof) {
        CanonicalLoopFamilySelectionOutcomeV1::Selected(selection) => {
            assert!(selection.unit_coverage().matches_lease(selection.lease()));
            assert_eq!(selection.unit_coverage().owner(), owner);
        }
        _ => panic!("one candidate must be selected"),
    }
}
