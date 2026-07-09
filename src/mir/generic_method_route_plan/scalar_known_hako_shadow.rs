use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::{
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};

use super::generated::write_set_mapstore_i64_hako_policy::{
    HakoMapStoreI64Policy, WRITE_SET_MAPSTORE_I64_HAKO_POLICY,
};
use super::scalar_known_typed_direct_closeout_contract::{
    accepted_scalar_known_contracts, ScalarKnownEffectClass, ScalarKnownSurfaceId,
};
use super::{GenericMethodRouteDecision, GenericMethodRouteKind, GenericMethodRouteProof};

pub(super) fn mapstore_i64_shadow_consumed_decision() -> GenericMethodRouteDecision {
    let policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
    let accepted_contract_count = accepted_scalar_known_contracts().count();
    assert!(
        accepted_contract_count >= 4,
        "ScalarKnown accepted contract boundary lost prior closeouts"
    );
    let contract_contains_route = accepted_scalar_known_contracts().any(|contract| {
        contract.contract_id.as_str() == "WriteScalarI64RoutesScopedCloseout"
            && contract.surface_id.as_str() == "WriteScalarI64Routes"
            && contract.surface_id == ScalarKnownSurfaceId::WriteScalarI64Routes
            && contract.effect_class.as_str() == ScalarKnownEffectClass::Mutate.as_str()
            && contract
                .route_kind_set
                .contains(&GenericMethodRouteKind::MapStoreI64)
    });
    assert!(
        contract_contains_route,
        "ScalarKnown Write contract no longer contains MapStoreI64"
    );
    assert_hako_policy_matches_rust(&policy);

    GenericMethodRouteDecision::new(
        GenericMethodRouteKind::MapStoreI64,
        GenericMethodRouteProof::SetSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapSet,
            CoreMethodLoweringTier::ColdFallback,
        )),
        None,
        GenericMethodValueDemand::WriteAny,
        None,
    )
}

fn assert_hako_policy_matches_rust(policy: &HakoMapStoreI64Policy) {
    assert_eq!(policy.surface, "SetSurfacePolicy");
    assert_eq!(policy.route_kind, GenericMethodRouteKind::MapStoreI64);
    assert_eq!(policy.core_op, CoreMethodOp::MapSet);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::ColdFallback);
    assert_eq!(policy.result_class, "NoneResult");
    assert_eq!(policy.return_shape, "None");
    assert_eq!(
        GenericMethodReturnShape::ScalarI64.as_metadata_name(),
        "scalar_i64"
    );
    assert_eq!(
        policy.value_demand,
        GenericMethodValueDemand::WriteAny,
        "MapStoreI64 .hako policy value demand drifted"
    );
    assert_eq!(
        GenericMethodValueDemand::WriteAny.as_metadata_name(),
        "write_any"
    );
    assert_eq!(policy.value_boundary, "ScalarI64");
    assert_eq!(policy.publication_policy, "NonePublication");
    assert_eq!(
        GenericMethodPublicationPolicy::NoPublication.as_metadata_name(),
        "no_publication"
    );
    assert_eq!(policy.effect_class, "mutate");
    assert_eq!(policy.mutation_class, "MutatesReceiverOrContainer");
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

#[cfg(test)]
mod tests {
    use super::super::scalar_known_typed_direct_closeout_contract::ScalarKnownContractId;
    use super::*;

    #[test]
    fn mapstore_i64_shadow_artifact_matches_rust_fastpath_policy() {
        let _ = mapstore_i64_shadow_consumed_decision();
        assert_eq!(
            ScalarKnownContractId::WriteScalarI64Routes.as_str(),
            "WriteScalarI64RoutesScopedCloseout"
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_shadow_rejects_route_kind_mismatch() {
        let mut policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
        policy.route_kind = GenericMethodRouteKind::MapStoreAny;
        assert_hako_policy_matches_rust(&policy);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_shadow_rejects_core_op_mismatch() {
        let mut policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
        policy.core_op = CoreMethodOp::MapDelete;
        assert_hako_policy_matches_rust(&policy);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_shadow_rejects_role_mismatch() {
        let mut policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
        policy.role = "hako_runtime_route_authority";
        assert_hako_policy_matches_rust(&policy);
    }
}
