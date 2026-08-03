use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::nested_predicate_producer_tests::nested_function;
use super::nested_predicate_profile::CanonicalNestedPredicatePlanV1;
use super::VerifiedResolvedSourceUnitV1;

fn nested_plan() -> CanonicalNestedPredicatePlanV1<'static> {
    let unit = Box::leak(Box::new(
        VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
            .expect("nested source unit"),
    ));
    let plan = CanonicalLoweringPreflightV1::verify(unit).expect("nested preflight");
    let CanonicalFirstFamilyPlanV1::Loop(
        super::capability::CanonicalLoopFamilyPlanV1::NestedPredicate(plan),
    ) = plan
    else {
        panic!("Nested must win before DirectAccum");
    };
    plan
}

#[test]
fn nested_probe_wins_over_overlapping_direct_accum_envelope() {
    let plan = nested_plan();
    assert_eq!(plan.input().owner(), plan.loop_stmt().owner());
    assert_eq!(plan.claims().prefix().owner(), plan.input().owner());
    assert_eq!(plan.emission().topology().owner(), plan.input().owner());
}

#[test]
fn nested_plan_seals_existing_binding_ssa_header_family() {
    let plan = nested_plan();
    let header = plan
        .seal_resolved_owner_header_v1()
        .expect("nested owner header");
    assert_eq!(
        header.family(),
        super::capability::ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
    );
    assert_eq!(header.owner(), plan.input().owner());
}
