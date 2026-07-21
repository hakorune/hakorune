//! HEADERPORT0 WIRING-I0-ROUTEINV-P0e: test-only route closure.
//!
//! The production route matrix and route-owned policy remain the only route
//! authorities. This proof projects those existing products to P0b/P0c/P0d;
//! it owns no caller-authored symbol inventory or production consumer.

use super::module_invocation_route_matrix::{
    InvocationRootFamilyV1, InvocationRouteMatrixRowV1, InvocationRouteMatrixV1,
};
use super::route_owned_invocation_inventory::{
    InvocationInventoryAuthorityV2, RouteFallbackPolicyV2, RouteOwnedInvocationInventoryV2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RouteAuthorityLaneV1 {
    RawLedger,
    SingleOwnerHeader,
    CallableBatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RouteAuthorityProjectionV1 {
    route: InvocationRouteMatrixRowV1,
    authority: RouteAuthorityLaneV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RouteObservationV1 {
    entered: bool,
    changed: bool,
}

const FAMILIES: [InvocationRootFamilyV1; 5] = [
    InvocationRootFamilyV1::Raw,
    InvocationRootFamilyV1::CanonicalAPlus,
    InvocationRootFamilyV1::BindingSsaTrivial,
    InvocationRootFamilyV1::BindingSsaAcyclic,
    InvocationRootFamilyV1::BindingSsaRecursive,
];

fn authority_lane(authority: InvocationInventoryAuthorityV2) -> RouteAuthorityLaneV1 {
    match authority {
        InvocationInventoryAuthorityV2::RawExpansionReceipts => RouteAuthorityLaneV1::RawLedger,
        InvocationInventoryAuthorityV2::CanonicalResolvedOwner => {
            RouteAuthorityLaneV1::SingleOwnerHeader
        }
        InvocationInventoryAuthorityV2::CanonicalCallableCatalog => {
            RouteAuthorityLaneV1::CallableBatch
        }
    }
}

fn project_existing_authorities() -> (Vec<RouteAuthorityProjectionV1>, [usize; 4]) {
    let mut rows = Vec::new();
    let mut policy_lane_counts = [0; 4];

    for family in FAMILIES {
        let inventory = RouteOwnedInvocationInventoryV2::derive(family).unwrap();
        let lane = match &inventory {
            RouteOwnedInvocationInventoryV2::Raw(_) => 0,
            RouteOwnedInvocationInventoryV2::CanonicalSingle(_) => 1,
            RouteOwnedInvocationInventoryV2::BindingSsaAcyclic(_) => 2,
            RouteOwnedInvocationInventoryV2::BindingSsaRecursive(_) => 3,
        };
        let policy = inventory.policy();
        assert_eq!(policy.family(), family);
        assert_eq!(policy.fallback(), RouteFallbackPolicyV2::Forbidden);
        policy_lane_counts[lane] += policy.matrix_rows().len();

        let authority = authority_lane(policy.inventory_authority());
        rows.extend(
            policy
                .matrix_rows()
                .iter()
                .copied()
                .map(|route| RouteAuthorityProjectionV1 { route, authority }),
        );
    }

    (rows, policy_lane_counts)
}

#[test]
fn route_matrix_projects_all_nine_rows_to_exactly_one_existing_authority() {
    let (projected, policy_lane_counts) = project_existing_authorities();
    assert_eq!(policy_lane_counts, [4, 3, 1, 1]);
    assert_eq!(projected.len(), 9);

    for expected in InvocationRouteMatrixV1::rows() {
        assert_eq!(
            projected
                .iter()
                .filter(|actual| actual.route == *expected)
                .count(),
            1
        );
    }

    let authority_counts = [
        RouteAuthorityLaneV1::RawLedger,
        RouteAuthorityLaneV1::SingleOwnerHeader,
        RouteAuthorityLaneV1::CallableBatch,
    ]
    .map(|authority| {
        projected
            .iter()
            .filter(|row| row.authority == authority)
            .count()
    });
    assert_eq!(authority_counts, [4, 3, 2]);
}

#[test]
fn route_failure_and_publication_laws_remain_matrix_projections() {
    let (projected, _) = project_existing_authorities();
    for row in projected {
        let matrix = InvocationRouteMatrixV1::rows()
            .iter()
            .find(|expected| **expected == row.route)
            .expect("projected row remains in the route-matrix SSOT");
        assert_eq!(row.route.publication(), matrix.publication());
        assert_eq!(row.route.failure(), matrix.failure());
        assert!(!row.route.failure().stages().is_empty());
        assert!(row.route.failure().collector_prefix_unchanged());
        assert!(!row.route.failure().retry());
    }
}

#[test]
fn entered_and_changed_observations_are_independent_dimensions() {
    let route = InvocationRouteMatrixV1::rows()[0];
    let entered_without_change = (
        route,
        RouteObservationV1 {
            entered: true,
            changed: false,
        },
    );
    let entered_with_change = (
        route,
        RouteObservationV1 {
            entered: true,
            changed: true,
        },
    );

    assert_eq!(entered_without_change.0, entered_with_change.0);
    assert!(entered_without_change.1.entered);
    assert!(!entered_without_change.1.changed);
    assert!(entered_with_change.1.entered);
    assert!(entered_with_change.1.changed);
}
