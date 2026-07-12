use super::generated::write_set_mapstore_route_policy::{
    MapStoreRoutePolicyRow, MAPSTORE_ROUTE_POLICY_ROWS,
};
use super::GenericMethodRouteKind;
use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp};
use crate::mir::generic_method_route_facts::GenericMethodValueDemand;

pub(super) fn mapstore_policy(route_kind: GenericMethodRouteKind) -> &'static MapStoreRoutePolicyRow {
    MAPSTORE_ROUTE_POLICY_ROWS
        .iter()
        .find(|policy| policy.route_kind == route_kind)
        .expect("MapStore RoutePolicyRow missing requested route")
}

pub(super) fn assert_mapstore_policy_row(policy: &MapStoreRoutePolicyRow) {
    assert_eq!(policy.result_shape, "None");
    assert_eq!(policy.effect_class, "mutate");
    assert_eq!(policy.mutation_class, "MutatesReceiverOrContainer");
    assert_eq!(policy.publication_policy, "NonePublication");
    assert!(policy.surface.starts_with("SetSurfacePolicy"));
    assert_eq!(policy.core_op, CoreMethodOp::MapSet);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::ColdFallback);
    assert_eq!(policy.value_demand, GenericMethodValueDemand::WriteAny);
    assert_eq!(policy.authority_kind, "HakoPolicyRow");

    let expected_domains = match policy.route_kind {
        GenericMethodRouteKind::MapStoreI64 => ("I64", "Any"),
        GenericMethodRouteKind::MapStoreAny => ("Any", "Any"),
        _ => panic!("unexpected route in MapStore RoutePolicyRow"),
    };
    assert_eq!(policy.key_domain, expected_domains.0);
    assert_eq!(policy.stored_value_domain, expected_domains.1);
}
