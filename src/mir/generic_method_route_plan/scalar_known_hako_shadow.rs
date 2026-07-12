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
#[cfg(test)]
use super::generated::write_set_mapstore_any_hako_policy::HakoMapStoreAnyPolicy;
#[cfg(test)]
use super::generated::write_set_mapstore_any_hako_policy::WRITE_SET_MAPSTORE_ANY_HAKO_POLICY;
#[cfg(test)]
use super::generated::write_set_mapstore_i64_hako_policy::HakoMapStoreI64Policy;
#[cfg(test)]
use super::generated::write_set_mapstore_i64_hako_policy::WRITE_SET_MAPSTORE_I64_HAKO_POLICY;
use super::generated::write_set_mapstore_route_policy::{
    MapStoreRoutePolicyRow, MAPSTORE_ROUTE_POLICY_ROWS,
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
    super::caller_orientation::assert_mapload_authority_pilot(policy.policy_row_id);
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
    super::caller_orientation::assert_string_authority_pilot(policy.policy_row_id);
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

#[allow(dead_code)]
pub(super) fn collection_scalar_i64_shadow_consumed_decision(
    route_kind: GenericMethodRouteKind,
    core_op: CoreMethodOp,
) -> GenericMethodRouteDecision {
    collection_scalar_i64_hako_route_authority_pilot_decision(route_kind, core_op, None)
}

pub(super) fn collection_scalar_i64_hako_route_authority_pilot_decision(
    route_kind: GenericMethodRouteKind,
    core_op: CoreMethodOp,
    receiver_domain: Option<&str>,
) -> GenericMethodRouteDecision {
    let policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES
        .iter()
        .find(|policy| policy.route_kind == route_kind)
        .expect("Collection .hako policy table missing Rust route kind");
    super::caller_orientation::assert_collection_authority_pilot(policy.policy_row_id);
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
    if let Some(receiver_domain) = receiver_domain {
        assert_eq!(
            policy.receiver_domain, receiver_domain,
            "Collection .hako authority pilot receiver domain diverged from Rust oracle"
        );
    }
    if route_kind == GenericMethodRouteKind::AnyLength {
        assert_eq!(
            policy.receiver_domain, "Box",
            "AnyLength must remain an explicit Box metadata row"
        );
    }

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
        GenericMethodRouteProof::LenSurfacePolicy,
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
        "Collection .hako authority pilot diverged from Rust oracle"
    );
    hako_decision
}

#[allow(dead_code)]
pub(super) fn mapstore_i64_shadow_consumed_decision() -> GenericMethodRouteDecision {
    mapstore_i64_hako_route_authority_pilot_decision()
}

pub(super) fn mapstore_i64_hako_route_authority_pilot_decision() -> GenericMethodRouteDecision {
    let policy = mapstore_policy(GenericMethodRouteKind::MapStoreI64);
    super::caller_orientation::assert_mapstore_i64_policy_row(policy.policy_row_id);
    assert_write_contract_contains(GenericMethodRouteKind::MapStoreI64, "MapStoreI64");
    assert_mapstore_policy_row(policy);

    let hako_decision = GenericMethodRouteDecision::new(
        policy.route_kind,
        GenericMethodRouteProof::SetSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(policy.core_op, policy.lowering_tier)),
        None,
        policy.value_demand,
        None,
    );
    let rust_oracle = GenericMethodRouteDecision::new(
        GenericMethodRouteKind::MapStoreI64,
        GenericMethodRouteProof::SetSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapSet,
            CoreMethodLoweringTier::ColdFallback,
        )),
        None,
        GenericMethodValueDemand::WriteAny,
        None,
    );
    assert_eq!(
        hako_decision, rust_oracle,
        "MapStoreI64 .hako authority pilot diverged from Rust oracle"
    );
    hako_decision
}

fn assert_write_contract_contains(route_kind: GenericMethodRouteKind, label: &str) {
    assert!(
        accepted_scalar_known_contracts().count() >= 4,
        "ScalarKnown accepted contract boundary lost prior closeouts"
    );
    let contains_route = accepted_scalar_known_contracts().any(|contract| {
        contract.contract_id.as_str() == "WriteScalarI64RoutesScopedCloseout"
            && contract.surface_id.as_str() == "WriteScalarI64Routes"
            && contract.surface_id == ScalarKnownSurfaceId::WriteScalarI64Routes
            && contract.effect_class.as_str() == ScalarKnownEffectClass::Mutate.as_str()
            && contract.route_kind_set.contains(&route_kind)
    });
    assert!(
        contains_route,
        "ScalarKnown Write contract no longer contains {label}"
    );
}

#[allow(dead_code)]
pub(super) fn write_push_shadow_consumed_decision() -> GenericMethodRouteDecision {
    write_push_hako_route_authority_pilot_decision()
}

pub(super) fn write_push_hako_route_authority_pilot_decision() -> GenericMethodRouteDecision {
    let policy = WRITE_PUSH_HAKO_POLICY;
    super::caller_orientation::assert_push_arrayappendany_policy_row(policy.policy_row_id);
    assert_write_contract_contains(GenericMethodRouteKind::ArrayAppendAny, "ArrayAppendAny");
    assert_hako_write_push_policy_matches_rust(&policy);

    let hako_decision = GenericMethodRouteDecision::new(
        policy.route_kind,
        GenericMethodRouteProof::PushSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            policy.core_op,
            policy.lowering_tier,
        )),
        Some(GenericMethodReturnShape::ScalarI64),
        policy.value_demand,
        Some(GenericMethodPublicationPolicy::NoPublication),
    );
    let rust_oracle = GenericMethodRouteDecision::new(
        GenericMethodRouteKind::ArrayAppendAny,
        GenericMethodRouteProof::PushSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::ArrayPush,
            CoreMethodLoweringTier::ColdFallback,
        )),
        Some(GenericMethodReturnShape::ScalarI64),
        GenericMethodValueDemand::WriteAny,
        Some(GenericMethodPublicationPolicy::NoPublication),
    );
    assert_eq!(
        hako_decision, rust_oracle,
        "Push .hako authority pilot diverged from Rust oracle"
    );
    hako_decision
}

#[allow(dead_code)]
pub(super) fn mapstore_any_shadow_consumed_decision() -> GenericMethodRouteDecision {
    mapstore_any_hako_route_authority_pilot_decision()
}

pub(super) fn mapstore_any_hako_route_authority_pilot_decision() -> GenericMethodRouteDecision {
    let policy = mapstore_policy(GenericMethodRouteKind::MapStoreAny);
    super::caller_orientation::assert_mapstore_any_policy_row(policy.policy_row_id);
    assert_write_contract_contains(GenericMethodRouteKind::MapStoreAny, "MapStoreAny");
    assert_mapstore_policy_row(policy);

    let hako_decision = GenericMethodRouteDecision::new(
        policy.route_kind,
        GenericMethodRouteProof::SetSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(policy.core_op, policy.lowering_tier)),
        None,
        policy.value_demand,
        None,
    );
    let rust_oracle = GenericMethodRouteDecision::new(
        GenericMethodRouteKind::MapStoreAny,
        GenericMethodRouteProof::SetSurfacePolicy,
        Some(CoreMethodOpCarrier::manifest(
            CoreMethodOp::MapSet,
            CoreMethodLoweringTier::ColdFallback,
        )),
        None,
        GenericMethodValueDemand::WriteAny,
        None,
    );
    assert_eq!(
        hako_decision, rust_oracle,
        "MapStoreAny .hako authority pilot diverged from Rust oracle"
    );
    hako_decision
}

fn mapstore_policy(route_kind: GenericMethodRouteKind) -> &'static MapStoreRoutePolicyRow {
    MAPSTORE_ROUTE_POLICY_ROWS
        .iter()
        .find(|policy| policy.route_kind == route_kind)
        .expect("MapStore RoutePolicyRow missing requested route")
}

fn assert_mapstore_policy_row(policy: &MapStoreRoutePolicyRow) {
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

#[cfg(test)]
fn assert_hako_policy_matches_rust(policy: &HakoMapStoreI64Policy) {
    assert_eq!(policy.surface, "SetSurfacePolicy");
    assert_eq!(policy.route_kind, GenericMethodRouteKind::MapStoreI64);
    assert_eq!(policy.core_op, CoreMethodOp::MapSet);
    assert_eq!(policy.lowering_tier, CoreMethodLoweringTier::ColdFallback);
    assert_eq!(policy.result_class, "NoneResult");
    assert_eq!(policy.return_shape, "None");
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
    assert_eq!(policy.effect_class, "mutate");
    assert_eq!(policy.mutation_class, "MutatesReceiverOrContainer");
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

#[cfg(test)]
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
    assert_eq!(policy.value_boundary, "Any");
    assert_eq!(policy.publication_policy, "NonePublication");
    assert_eq!(policy.effect_class, "mutate");
    assert_eq!(policy.mutation_class, "MutatesReceiverOrContainer");
    assert_eq!(policy.any_boundary_policy, "DeclaredMetadataOnly");
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

#[cfg(test)]
#[path = "scalar_known_hako_shadow/tests.rs"]
mod tests;
