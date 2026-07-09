use crate::mir::core_method_op::{CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier};
use crate::mir::generic_method_route_facts::{
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodValueDemand,
};

use super::generated::collection_len_scalar_i64_hako_policy::{
    HakoCollectionLenScalarI64Policy, COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES,
};
use super::generated::mapload_scalar_i64_hako_policy::{
    HakoMapLoadScalarI64Policy, MAPLOAD_SCALAR_I64_HAKO_POLICY,
};
use super::generated::string_search_scalar_i64_hako_policy::{
    HakoStringSearchScalarI64Policy, STRING_SEARCH_SCALAR_I64_HAKO_POLICIES,
};
use super::generated::write_push_hako_policy::{HakoWritePushPolicy, WRITE_PUSH_HAKO_POLICY};
use super::generated::write_set_mapstore_any_hako_policy::{
    HakoMapStoreAnyPolicy, WRITE_SET_MAPSTORE_ANY_HAKO_POLICY,
};
use super::generated::write_set_mapstore_i64_hako_policy::{
    HakoMapStoreI64Policy, WRITE_SET_MAPSTORE_I64_HAKO_POLICY,
};
use super::scalar_known_typed_direct_closeout_contract::{
    accepted_scalar_known_contracts, ScalarKnownContractId, ScalarKnownEffectClass,
    ScalarKnownSurfaceId,
};
use super::{GenericMethodRouteDecision, GenericMethodRouteKind, GenericMethodRouteProof};

#[allow(dead_code)]
pub(super) fn mapload_scalar_i64_shadow_consumed_decision(
    route_proof: GenericMethodRouteProof,
) -> GenericMethodRouteDecision {
    mapload_scalar_i64_hako_route_authority_pilot_decision(route_proof)
}

pub(super) fn mapload_scalar_i64_hako_route_authority_pilot_decision(
    route_proof: GenericMethodRouteProof,
) -> GenericMethodRouteDecision {
    let policy = MAPLOAD_SCALAR_I64_HAKO_POLICY;
    let accepted_contract_count = accepted_scalar_known_contracts().count();
    assert!(
        accepted_contract_count >= 4,
        "ScalarKnown accepted contract boundary lost prior closeouts"
    );
    let contract_contains_route = accepted_scalar_known_contracts().any(|contract| {
        contract.contract_id == ScalarKnownContractId::MapLoadScalarI64
            && contract.surface_id == ScalarKnownSurfaceId::MapLoadScalarI64Routes
            && contract.effect_class.as_str() == ScalarKnownEffectClass::Read.as_str()
            && contract
                .route_kind_set
                .contains(&GenericMethodRouteKind::MapLoadScalarI64)
    });
    assert!(
        contract_contains_route,
        "ScalarKnown MapLoad contract no longer contains MapLoadScalarI64"
    );
    assert_hako_mapload_scalar_i64_policy_matches_rust(&policy, route_proof);

    let hako_decision = GenericMethodRouteDecision::new(
        policy.route_kind,
        route_proof,
        Some(CoreMethodOpCarrier::manifest(
            policy.core_op,
            policy.lowering_tier,
        )),
        Some(policy.return_shape),
        policy.value_demand,
        Some(policy.publication_policy),
    );
    let rust_oracle = GenericMethodRouteDecision::new(
        GenericMethodRouteKind::MapLoadScalarI64,
        route_proof,
        Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapGet,
            CoreMethodLoweringTier::WarmDirectAbi,
        )),
        Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
        GenericMethodValueDemand::ScalarI64,
        Some(GenericMethodPublicationPolicy::NoPublication),
    );
    assert_eq!(
        hako_decision, rust_oracle,
        "MapLoad .hako authority pilot diverged from Rust oracle"
    );
    hako_decision
}

#[allow(dead_code)]
pub(super) fn string_scalar_i64_shadow_consumed_decision(
    route_kind: GenericMethodRouteKind,
    route_proof: GenericMethodRouteProof,
    core_op: CoreMethodOp,
) -> GenericMethodRouteDecision {
    string_scalar_i64_hako_route_authority_pilot_decision(route_kind, route_proof, core_op)
}

pub(super) fn string_scalar_i64_hako_route_authority_pilot_decision(
    route_kind: GenericMethodRouteKind,
    route_proof: GenericMethodRouteProof,
    core_op: CoreMethodOp,
) -> GenericMethodRouteDecision {
    let policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES
        .iter()
        .find(|policy| policy.route_kind == route_kind)
        .expect("String .hako policy table missing Rust route kind");
    let accepted_contract_count = accepted_scalar_known_contracts().count();
    assert!(
        accepted_contract_count >= 4,
        "ScalarKnown accepted contract boundary lost prior closeouts"
    );
    let contract_contains_route = accepted_scalar_known_contracts().any(|contract| {
        contract.contract_id == ScalarKnownContractId::StringSearchScalarI64
            && contract.surface_id == ScalarKnownSurfaceId::StringScalarI64Routes
            && contract.effect_class.as_str() == ScalarKnownEffectClass::Read.as_str()
            && contract.route_kind_set.contains(&route_kind)
    });
    assert!(
        contract_contains_route,
        "ScalarKnown String contract no longer contains requested route"
    );
    assert_hako_string_scalar_i64_policy_matches_rust(policy, route_kind, route_proof, core_op);

    let hako_decision = GenericMethodRouteDecision::new(
        policy.route_kind,
        policy.proof_or_policy_source,
        Some(CoreMethodOpCarrier::manifest(
            policy.core_op,
            policy.lowering_tier,
        )),
        Some(policy.return_shape),
        policy.value_demand,
        Some(policy.publication_policy),
    );
    let rust_oracle = GenericMethodRouteDecision::new(
        route_kind,
        route_proof,
        Some(CoreMethodOpCarrier::manifest(
            core_op,
            CoreMethodLoweringTier::WarmDirectAbi,
        )),
        Some(GenericMethodReturnShape::ScalarI64),
        GenericMethodValueDemand::ScalarI64,
        Some(GenericMethodPublicationPolicy::NoPublication),
    );
    assert_eq!(
        hako_decision, rust_oracle,
        "String .hako authority pilot diverged from Rust oracle"
    );
    hako_decision
}

pub(super) fn collection_scalar_i64_shadow_consumed_decision(
    route_kind: GenericMethodRouteKind,
    core_op: CoreMethodOp,
) -> GenericMethodRouteDecision {
    let policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES
        .iter()
        .find(|policy| policy.route_kind == route_kind)
        .expect("Collection .hako policy table missing Rust route kind");
    let accepted_contract_count = accepted_scalar_known_contracts().count();
    assert!(
        accepted_contract_count >= 4,
        "ScalarKnown accepted contract boundary lost prior closeouts"
    );
    let contract_contains_route = accepted_scalar_known_contracts().any(|contract| {
        contract.contract_id == ScalarKnownContractId::CollectionLenScalarI64
            && contract.surface_id == ScalarKnownSurfaceId::CollectionScalarI64Routes
            && contract.effect_class.as_str() == ScalarKnownEffectClass::Observe.as_str()
            && contract.route_kind_set.contains(&route_kind)
    });
    assert!(
        contract_contains_route,
        "ScalarKnown Collection contract no longer contains requested route"
    );
    assert_hako_collection_scalar_i64_policy_matches_rust(policy, route_kind, core_op);

    GenericMethodRouteDecision::new(
        route_kind,
        GenericMethodRouteProof::LenSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            core_op,
            CoreMethodLoweringTier::WarmDirectAbi,
        )),
        Some(GenericMethodReturnShape::ScalarI64),
        GenericMethodValueDemand::ScalarI64,
        Some(GenericMethodPublicationPolicy::NoPublication),
    )
}

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

pub(super) fn write_push_shadow_consumed_decision() -> GenericMethodRouteDecision {
    let policy = WRITE_PUSH_HAKO_POLICY;
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
                .contains(&GenericMethodRouteKind::ArrayAppendAny)
    });
    assert!(
        contract_contains_route,
        "ScalarKnown Write contract no longer contains ArrayAppendAny"
    );
    assert_hako_write_push_policy_matches_rust(&policy);

    GenericMethodRouteDecision::new(
        GenericMethodRouteKind::ArrayAppendAny,
        GenericMethodRouteProof::PushSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::ArrayPush,
            CoreMethodLoweringTier::ColdFallback,
        )),
        Some(GenericMethodReturnShape::ScalarI64),
        GenericMethodValueDemand::WriteAny,
        Some(GenericMethodPublicationPolicy::NoPublication),
    )
}

pub(super) fn mapstore_any_shadow_consumed_decision() -> GenericMethodRouteDecision {
    let policy = WRITE_SET_MAPSTORE_ANY_HAKO_POLICY;
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
                .contains(&GenericMethodRouteKind::MapStoreAny)
    });
    assert!(
        contract_contains_route,
        "ScalarKnown Write contract no longer contains MapStoreAny"
    );
    assert_hako_mapstore_any_policy_matches_rust(&policy);

    GenericMethodRouteDecision::new(
        GenericMethodRouteKind::MapStoreAny,
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

fn assert_hako_write_push_policy_matches_rust(policy: &HakoWritePushPolicy) {
    assert_eq!(policy.surface, "PushSurfacePolicy");
    assert_eq!(policy.route_kind, GenericMethodRouteKind::ArrayAppendAny);
    assert_eq!(policy.core_op, CoreMethodOp::ArrayPush);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::ColdFallback);
    assert_eq!(policy.result_class, "ScalarI64Result");
    assert_eq!(policy.return_shape, "ScalarI64");
    assert_eq!(
        GenericMethodReturnShape::ScalarI64.as_metadata_name(),
        "scalar_i64"
    );
    assert_eq!(
        policy.value_demand,
        GenericMethodValueDemand::WriteAny,
        "Write Push .hako policy value demand drifted"
    );
    assert_eq!(
        GenericMethodValueDemand::WriteAny.as_metadata_name(),
        "write_any"
    );
    assert_eq!(policy.publication_policy, "NoPublication");
    assert_eq!(
        GenericMethodPublicationPolicy::NoPublication.as_metadata_name(),
        "no_publication"
    );
    assert_eq!(policy.effect_class, "mutate");
    assert_eq!(policy.mutation_class, "MutatesReceiverOrContainer");
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

fn assert_hako_mapload_scalar_i64_policy_matches_rust(
    policy: &HakoMapLoadScalarI64Policy,
    route_proof: GenericMethodRouteProof,
) {
    assert_eq!(policy.surface, "MapLoadScalarI64Routes");
    assert_eq!(policy.route_kind, GenericMethodRouteKind::MapLoadScalarI64);
    assert_eq!(policy.core_op, CoreMethodOp::MapGet);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::WarmDirectAbi);
    assert_eq!(policy.result_class, "ScalarI64OrMissingZeroResult");
    assert_eq!(
        policy.return_shape,
        GenericMethodReturnShape::ScalarI64OrMissingZero
    );
    assert_eq!(
        GenericMethodReturnShape::ScalarI64OrMissingZero.as_metadata_name(),
        "scalar_i64_or_missing_zero"
    );
    assert_eq!(
        policy.value_demand,
        GenericMethodValueDemand::ScalarI64,
        "MapLoad .hako policy value demand drifted"
    );
    assert_eq!(
        GenericMethodValueDemand::ScalarI64.as_metadata_name(),
        "scalar_i64"
    );
    assert_eq!(
        policy.publication_policy,
        GenericMethodPublicationPolicy::NoPublication
    );
    assert_eq!(
        GenericMethodPublicationPolicy::NoPublication.as_metadata_name(),
        "no_publication"
    );
    assert_eq!(policy.effect_class, "read");
    assert_eq!(policy.proof_family, "ScalarI64MapGetStoreFact");
    assert!(
        policy.allowed_proofs.contains(&route_proof),
        "MapLoad .hako policy does not allow Rust scalar-map proof"
    );
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

fn assert_hako_string_scalar_i64_policy_matches_rust(
    policy: &HakoStringSearchScalarI64Policy,
    route_kind: GenericMethodRouteKind,
    route_proof: GenericMethodRouteProof,
    core_op: CoreMethodOp,
) {
    assert_eq!(policy.surface, "StringScalarI64Routes");
    assert_eq!(policy.route_kind, route_kind);
    assert_eq!(policy.core_op, core_op);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::WarmDirectAbi);
    assert_eq!(policy.result_class, "ScalarI64Result");
    assert_eq!(policy.return_shape, GenericMethodReturnShape::ScalarI64);
    assert_eq!(
        GenericMethodReturnShape::ScalarI64.as_metadata_name(),
        "scalar_i64"
    );
    assert_eq!(
        policy.value_demand,
        GenericMethodValueDemand::ScalarI64,
        "String .hako policy value demand drifted"
    );
    assert_eq!(
        GenericMethodValueDemand::ScalarI64.as_metadata_name(),
        "scalar_i64"
    );
    assert_eq!(
        policy.publication_policy,
        GenericMethodPublicationPolicy::NoPublication
    );
    assert_eq!(
        GenericMethodPublicationPolicy::NoPublication.as_metadata_name(),
        "no_publication"
    );
    assert_eq!(policy.effect_class, "read");
    assert_eq!(policy.proof_or_policy_source, route_proof);
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

fn assert_hako_collection_scalar_i64_policy_matches_rust(
    policy: &HakoCollectionLenScalarI64Policy,
    route_kind: GenericMethodRouteKind,
    core_op: CoreMethodOp,
) {
    assert_eq!(policy.surface, "CollectionScalarI64Routes");
    assert_eq!(policy.route_kind, route_kind);
    assert_eq!(policy.core_op, core_op);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::WarmDirectAbi);
    assert_eq!(policy.result_class, "ScalarI64Result");
    assert_eq!(policy.return_shape, GenericMethodReturnShape::ScalarI64);
    assert_eq!(
        policy.value_demand,
        GenericMethodValueDemand::ScalarI64,
        "Collection .hako policy value demand drifted"
    );
    assert_eq!(
        policy.publication_policy,
        GenericMethodPublicationPolicy::NoPublication
    );
    assert_eq!(policy.effect_class, "observe");
    assert_eq!(
        policy.proof_or_policy_source,
        GenericMethodRouteProof::LenSurfacePolicy
    );
    assert_eq!(policy.role, "classifier_policy_mirror_only");
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

fn assert_hako_mapstore_any_policy_matches_rust(policy: &HakoMapStoreAnyPolicy) {
    assert_eq!(policy.surface, "SetSurfacePolicy/MapStoreAny");
    assert_eq!(policy.route_kind, GenericMethodRouteKind::MapStoreAny);
    assert_eq!(policy.core_op, CoreMethodOp::MapSet);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::ColdFallback);
    assert_eq!(policy.result_class, "NoneResult");
    assert_eq!(policy.return_shape, "None");
    assert_eq!(
        policy.value_demand,
        GenericMethodValueDemand::WriteAny,
        "MapStoreAny .hako policy value demand drifted"
    );
    assert_eq!(
        GenericMethodValueDemand::WriteAny.as_metadata_name(),
        "write_any"
    );
    assert_eq!(policy.value_boundary, "Any");
    assert_eq!(policy.publication_policy, "NonePublication");
    assert_eq!(
        GenericMethodPublicationPolicy::NoPublication.as_metadata_name(),
        "no_publication"
    );
    assert_eq!(policy.effect_class, "mutate");
    assert_eq!(policy.mutation_class, "MutatesReceiverOrContainer");
    assert_eq!(policy.any_boundary_policy, "DeclaredMetadataOnly");
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
    fn mapload_scalar_i64_shadow_artifact_matches_rust_fastpath_policy() {
        let decision = mapload_scalar_i64_hako_route_authority_pilot_decision(
            GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
        );
        assert_eq!(
            decision,
            GenericMethodRouteDecision::new(
                GenericMethodRouteKind::MapLoadScalarI64,
                GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
                Some(CoreMethodOpCarrier::manifest(
                    CoreMethodOp::MapGet,
                    CoreMethodLoweringTier::WarmDirectAbi,
                )),
                Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
                GenericMethodValueDemand::ScalarI64,
                Some(GenericMethodPublicationPolicy::NoPublication),
            )
        );
        assert_eq!(
            ScalarKnownContractId::MapLoadScalarI64.as_str(),
            "MapLoadScalarI64ScalarKnownTypedDirectCloseoutContract"
        );
    }

    #[test]
    fn string_scalar_i64_shadow_artifact_matches_rust_fastpath_policy() {
        for (route_kind, route_proof, core_op) in [
            (
                GenericMethodRouteKind::StringIndexOf,
                GenericMethodRouteProof::IndexOfSurfacePolicy,
                CoreMethodOp::StringIndexOf,
            ),
            (
                GenericMethodRouteKind::StringLastIndexOf,
                GenericMethodRouteProof::LastIndexOfSurfacePolicy,
                CoreMethodOp::StringLastIndexOf,
            ),
            (
                GenericMethodRouteKind::StringContains,
                GenericMethodRouteProof::ContainsSurfacePolicy,
                CoreMethodOp::StringContains,
            ),
        ] {
            let decision =
                string_scalar_i64_shadow_consumed_decision(route_kind, route_proof, core_op);
            assert_eq!(
                decision,
                GenericMethodRouteDecision::new(
                    route_kind,
                    route_proof,
                    Some(CoreMethodOpCarrier::manifest(
                        core_op,
                        CoreMethodLoweringTier::WarmDirectAbi,
                    )),
                    Some(GenericMethodReturnShape::ScalarI64),
                    GenericMethodValueDemand::ScalarI64,
                    Some(GenericMethodPublicationPolicy::NoPublication),
                )
            );
        }
        assert_eq!(
            ScalarKnownContractId::StringSearchScalarI64.as_str(),
            "StringSearchScalarI64TypedDirectCloseoutContract"
        );
    }

    #[test]
    fn collection_scalar_i64_shadow_artifact_matches_rust_fastpath_policy() {
        for (route_kind, core_op) in [
            (GenericMethodRouteKind::MapEntryCount, CoreMethodOp::MapLen),
            (GenericMethodRouteKind::ArraySlotLen, CoreMethodOp::ArrayLen),
            (GenericMethodRouteKind::StringLen, CoreMethodOp::StringLen),
            (GenericMethodRouteKind::AnyLength, CoreMethodOp::AnyLen),
        ] {
            let decision = collection_scalar_i64_shadow_consumed_decision(route_kind, core_op);
            assert_eq!(
                decision,
                GenericMethodRouteDecision::new(
                    route_kind,
                    GenericMethodRouteProof::LenSurfacePolicy,
                    Some(CoreMethodOpCarrier::manifest(
                        core_op,
                        CoreMethodLoweringTier::WarmDirectAbi,
                    )),
                    Some(GenericMethodReturnShape::ScalarI64),
                    GenericMethodValueDemand::ScalarI64,
                    Some(GenericMethodPublicationPolicy::NoPublication),
                )
            );
        }
        assert_eq!(
            ScalarKnownContractId::CollectionLenScalarI64.as_str(),
            "CollectionLenScalarI64TypedDirectCloseoutContract"
        );
    }

    #[test]
    fn write_push_shadow_artifact_matches_rust_fastpath_policy() {
        let _ = write_push_shadow_consumed_decision();
        assert_eq!(
            ScalarKnownContractId::WriteScalarI64Routes.as_str(),
            "WriteScalarI64RoutesScopedCloseout"
        );
    }

    #[test]
    fn mapstore_any_shadow_artifact_matches_rust_fastpath_policy() {
        let _ = mapstore_any_shadow_consumed_decision();
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
    #[should_panic(expected = "MapLoad .hako policy does not allow Rust scalar-map proof")]
    fn mapload_scalar_i64_shadow_rejects_unlisted_proof() {
        assert_hako_mapload_scalar_i64_policy_matches_rust(
            &MAPLOAD_SCALAR_I64_HAKO_POLICY,
            GenericMethodRouteProof::GetSurfacePolicy,
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapload_scalar_i64_shadow_rejects_role_mismatch() {
        let mut policy = MAPLOAD_SCALAR_I64_HAKO_POLICY;
        policy.role = "hako_runtime_route_authority";
        assert_hako_mapload_scalar_i64_policy_matches_rust(
            &policy,
            GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn string_scalar_i64_shadow_rejects_route_kind_mismatch() {
        let mut policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES[0];
        policy.route_kind = GenericMethodRouteKind::StringContains;
        assert_hako_string_scalar_i64_policy_matches_rust(
            &policy,
            GenericMethodRouteKind::StringIndexOf,
            GenericMethodRouteProof::IndexOfSurfacePolicy,
            CoreMethodOp::StringIndexOf,
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn string_scalar_i64_shadow_rejects_role_mismatch() {
        let mut policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES[0];
        policy.role = "hako_runtime_route_authority";
        assert_hako_string_scalar_i64_policy_matches_rust(
            &policy,
            GenericMethodRouteKind::StringIndexOf,
            GenericMethodRouteProof::IndexOfSurfacePolicy,
            CoreMethodOp::StringIndexOf,
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn collection_scalar_i64_shadow_rejects_core_op_mismatch() {
        let mut policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES[0];
        policy.core_op = CoreMethodOp::AnyLen;
        assert_hako_collection_scalar_i64_policy_matches_rust(
            &policy,
            GenericMethodRouteKind::MapEntryCount,
            CoreMethodOp::MapLen,
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn collection_scalar_i64_shadow_rejects_role_mismatch() {
        let mut policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES[0];
        policy.role = "hako_runtime_route_authority";
        assert_hako_collection_scalar_i64_policy_matches_rust(
            &policy,
            GenericMethodRouteKind::MapEntryCount,
            CoreMethodOp::MapLen,
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_shadow_rejects_role_mismatch() {
        let mut policy = WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
        policy.role = "hako_runtime_route_authority";
        assert_hako_policy_matches_rust(&policy);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn write_push_shadow_rejects_publication_mismatch() {
        let mut policy = WRITE_PUSH_HAKO_POLICY;
        policy.publication_policy = "RuntimePublication";
        assert_hako_write_push_policy_matches_rust(&policy);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn write_push_shadow_rejects_role_mismatch() {
        let mut policy = WRITE_PUSH_HAKO_POLICY;
        policy.role = "hako_runtime_route_authority";
        assert_hako_write_push_policy_matches_rust(&policy);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_any_shadow_rejects_value_boundary_mismatch() {
        let mut policy = WRITE_SET_MAPSTORE_ANY_HAKO_POLICY;
        policy.value_boundary = "ScalarI64";
        assert_hako_mapstore_any_policy_matches_rust(&policy);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_any_shadow_rejects_any_boundary_policy_mismatch() {
        let mut policy = WRITE_SET_MAPSTORE_ANY_HAKO_POLICY;
        policy.any_boundary_policy = "RuntimeAuthority";
        assert_hako_mapstore_any_policy_matches_rust(&policy);
    }
}
