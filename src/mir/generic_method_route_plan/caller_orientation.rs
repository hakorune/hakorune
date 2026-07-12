use super::generated::collection_len_scalar_i64_hako_policy::{
    HakoCollectionLenScalarI64Policy, COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES,
};
use super::generated::collection_scalar_i64_caller_orientation_contract::{
    HakoCollectionCallerOrientationContract, COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS,
};
use super::generated::mapload_scalar_i64_caller_orientation_contract::{
    HakoMapLoadCallerOrientationContract, MAPLOAD_SCALAR_I64_CALLER_ORIENTATION_CONTRACT,
};
use super::generated::mapload_scalar_i64_hako_policy::{
    HakoMapLoadScalarI64Policy, MAPLOAD_SCALAR_I64_HAKO_POLICY,
};
use super::generated::string_scalar_i64_caller_orientation_contract::{
    HakoStringCallerOrientationContract, STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS,
};
use super::generated::string_search_scalar_i64_hako_policy::{
    HakoStringSearchScalarI64Policy, STRING_SEARCH_SCALAR_I64_HAKO_POLICIES,
};
use super::generated::write_push_arrayappendany_caller_orientation_contract::{
    HakoWritePushArrayAppendAnyCallerOrientationContract,
    WRITE_PUSH_ARRAYAPPENDANY_CALLER_ORIENTATION_CONTRACT,
};
use super::generated::write_set_mapstore_any_caller_orientation_contract::{
    HakoMapStoreAnyCallerOrientationContract, WRITE_SET_MAPSTORE_ANY_CALLER_ORIENTATION_CONTRACT,
};
use super::generated::write_set_mapstore_i64_caller_orientation_contract::{
    HakoMapStoreI64CallerOrientationContract, WRITE_SET_MAPSTORE_I64_CALLER_ORIENTATION_CONTRACT,
};

const METADATA_ONLY: &str = "CallerOrientationContractMetadataOnly";
const SINGLE_SURFACE: &str = "SingleSurface";
const FORBIDDEN: &str = "Forbidden";
const FAIL_FAST: &str = "FailFast";

pub(super) fn assert_mapload_policy_row(policy_row_id: &str) {
    let contract = MAPLOAD_SCALAR_I64_CALLER_ORIENTATION_CONTRACT;
    assert_eq!(
        contract.policy_row_id, policy_row_id,
        "MapLoad caller-orientation policy row identity drift"
    );
    assert_contract_metadata(&contract);
}

fn assert_contract_metadata(contract: &HakoMapLoadCallerOrientationContract) {
    assert_eq!(contract.orientation_kind, METADATA_ONLY);
    assert_eq!(contract.scope, SINGLE_SURFACE);
    assert_eq!(contract.runtime_consumer, FORBIDDEN);
    assert_eq!(contract.backend_lowering_consumer, FORBIDDEN);
    assert_eq!(contract.mutation_consumer, FORBIDDEN);
    assert_eq!(contract.publication_consumer, FORBIDDEN);
    assert_eq!(contract.mismatch_policy, FAIL_FAST);
}

pub(super) fn assert_mapload_authority_pilot(policy_row_id: &str) {
    let policy = MAPLOAD_SCALAR_I64_HAKO_POLICY;
    assert_mapload_policy_row(policy_row_id);
    assert_eq!(
        policy.policy_row_id, policy_row_id,
        "MapLoad policy row identity drift"
    );
    assert_mapload_policy_metadata(&policy);
}

fn assert_mapload_policy_metadata(policy: &HakoMapLoadScalarI64Policy) {
    assert_eq!(policy.surface, "MapLoadScalarI64Routes");
    assert_eq!(
        policy.route_kind,
        super::GenericMethodRouteKind::MapLoadScalarI64
    );
    assert_eq!(
        policy.core_op,
        crate::mir::core_method_op::CoreMethodOp::MapGet
    );
    assert_eq!(
        policy.lowering_tier,
        crate::mir::core_method_op::CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(
        policy.return_shape,
        crate::mir::generic_method_route_facts::GenericMethodReturnShape::ScalarI64OrMissingZero
    );
    assert_eq!(
        policy.value_demand,
        crate::mir::generic_method_route_facts::GenericMethodValueDemand::ScalarI64
    );
    assert_eq!(
        policy.publication_policy,
        crate::mir::generic_method_route_facts::GenericMethodPublicationPolicy::NoPublication
    );
    assert_eq!(policy.effect_class, "read");
    assert_eq!(policy.proof_family, "ScalarI64MapGetStoreFact");
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

pub(super) fn assert_string_authority_pilot(policy_row_id: &str) {
    const EXPECTED_ROW_IDS: [&str; 3] = [
        "string_indexof_scalar_i64_routes",
        "string_lastindexof_scalar_i64_routes",
        "string_contains_scalar_i64_routes",
    ];

    assert_eq!(
        STRING_SEARCH_SCALAR_I64_HAKO_POLICIES.len(),
        EXPECTED_ROW_IDS.len(),
        "String caller-orientation policy set size drift"
    );
    assert_eq!(
        STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS.len(),
        EXPECTED_ROW_IDS.len(),
        "String caller-orientation contract set size drift"
    );

    for expected_row_id in EXPECTED_ROW_IDS {
        let contract = STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS
            .iter()
            .find(|contract| contract.policy_row_id == expected_row_id)
            .expect("String caller-orientation contract row missing");
        assert_string_contract_metadata(contract);

        let policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES
            .iter()
            .find(|policy| policy.policy_row_id == expected_row_id)
            .expect("String caller-orientation policy row missing");
        assert_string_policy_metadata(policy);
        assert_eq!(contract.policy_row_id, policy.policy_row_id);
    }

    let policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES
        .iter()
        .find(|policy| policy.policy_row_id == policy_row_id)
        .expect("String caller-orientation policy row identity drift");
    let contract = STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS
        .iter()
        .find(|contract| contract.policy_row_id == policy_row_id)
        .expect("String caller-orientation policy row identity drift");
    assert_string_contract_metadata(contract);
    assert_string_policy_metadata(policy);
}

fn assert_string_policy_metadata(policy: &HakoStringSearchScalarI64Policy) {
    assert_eq!(policy.surface, "StringScalarI64Routes");
    assert_eq!(
        policy.lowering_tier,
        super::super::core_method_op::CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(policy.result_class, "ScalarI64Result");
    assert_eq!(
        policy.return_shape,
        super::super::generic_method_route_facts::GenericMethodReturnShape::ScalarI64
    );
    assert_eq!(
        policy.value_demand,
        super::super::generic_method_route_facts::GenericMethodValueDemand::ScalarI64
    );
    assert_eq!(
        policy.publication_policy,
        super::super::generic_method_route_facts::GenericMethodPublicationPolicy::NoPublication
    );
    assert_eq!(policy.effect_class, "read");
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

fn assert_string_contract_metadata(contract: &HakoStringCallerOrientationContract) {
    assert_eq!(contract.orientation_kind, METADATA_ONLY);
    assert_eq!(contract.scope, SINGLE_SURFACE);
    assert_eq!(contract.runtime_consumer, FORBIDDEN);
    assert_eq!(contract.backend_lowering_consumer, FORBIDDEN);
    assert_eq!(contract.mutation_consumer, FORBIDDEN);
    assert_eq!(contract.publication_consumer, FORBIDDEN);
    assert_eq!(contract.mismatch_policy, FAIL_FAST);
}

pub(super) fn assert_collection_authority_pilot(policy_row_id: &str) {
    const EXPECTED_ROWS: [(&str, &str); 4] = [
        ("collection_map_entry_count_scalar_i64_routes", "MapBox"),
        ("collection_array_slot_len_scalar_i64_routes", "ArrayBox"),
        ("collection_string_len_scalar_i64_routes", "StringBox"),
        ("collection_any_length_scalar_i64_routes", "Box"),
    ];

    assert_eq!(
        COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES.len(),
        EXPECTED_ROWS.len(),
        "Collection caller-orientation policy set size drift"
    );
    assert_eq!(
        COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS.len(),
        EXPECTED_ROWS.len(),
        "Collection caller-orientation contract set size drift"
    );

    for (expected_row_id, expected_receiver_domain) in EXPECTED_ROWS {
        let contract = COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS
            .iter()
            .find(|contract| contract.policy_row_id == expected_row_id)
            .expect("Collection caller-orientation contract row missing");
        assert_collection_contract_metadata(contract);

        let policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES
            .iter()
            .find(|policy| policy.policy_row_id == expected_row_id)
            .expect("Collection caller-orientation policy row missing");
        assert_collection_policy_metadata(policy, expected_receiver_domain);
        assert_eq!(contract.policy_row_id, policy.policy_row_id);
    }

    let policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES
        .iter()
        .find(|policy| policy.policy_row_id == policy_row_id)
        .expect("Collection caller-orientation policy row identity drift");
    let contract = COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS
        .iter()
        .find(|contract| contract.policy_row_id == policy_row_id)
        .expect("Collection caller-orientation policy row identity drift");
    assert_collection_contract_metadata(contract);
    let expected_receiver_domain = EXPECTED_ROWS
        .iter()
        .find(|(expected_row_id, _)| *expected_row_id == policy_row_id)
        .map(|(_, receiver_domain)| *receiver_domain)
        .expect("Collection caller-orientation policy row identity drift");
    assert_collection_policy_metadata(policy, expected_receiver_domain);
}

fn assert_collection_contract_metadata(contract: &HakoCollectionCallerOrientationContract) {
    assert_eq!(contract.orientation_kind, METADATA_ONLY);
    assert_eq!(contract.scope, SINGLE_SURFACE);
    assert_eq!(contract.runtime_consumer, FORBIDDEN);
    assert_eq!(contract.backend_lowering_consumer, FORBIDDEN);
    assert_eq!(contract.mutation_consumer, FORBIDDEN);
    assert_eq!(contract.publication_consumer, FORBIDDEN);
    assert_eq!(contract.mismatch_policy, FAIL_FAST);
}

fn assert_collection_policy_metadata(
    policy: &HakoCollectionLenScalarI64Policy,
    expected_receiver_domain: &str,
) {
    assert_eq!(policy.surface, "CollectionScalarI64Routes");
    assert_eq!(policy.receiver_domain, expected_receiver_domain);
    assert_eq!(
        policy.lowering_tier,
        super::super::core_method_op::CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(policy.result_class, "ScalarI64Result");
    assert_eq!(
        policy.return_shape,
        super::super::generic_method_route_facts::GenericMethodReturnShape::ScalarI64
    );
    assert_eq!(
        policy.value_demand,
        super::super::generic_method_route_facts::GenericMethodValueDemand::ScalarI64
    );
    assert_eq!(
        policy.publication_policy,
        super::super::generic_method_route_facts::GenericMethodPublicationPolicy::NoPublication
    );
    assert_eq!(policy.effect_class, "observe");
    assert_eq!(
        policy.proof_or_policy_source,
        super::GenericMethodRouteProof::LenSurfacePolicy
    );
    assert_eq!(policy.role, "classifier_policy_mirror_only");
}

pub(super) fn assert_mapstore_i64_policy_row(policy_row_id: &str) {
    let policy = super::mapstore_route_policy_validator::mapstore_policy(
        super::GenericMethodRouteKind::MapStoreI64,
    );
    super::mapstore_route_policy_validator::assert_mapstore_policy_row(policy);
    let contract = WRITE_SET_MAPSTORE_I64_CALLER_ORIENTATION_CONTRACT;
    assert_eq!(
        contract.policy_row_id, policy_row_id,
        "MapStoreI64 caller-orientation policy row identity drift"
    );
    assert_mapstore_i64_contract_metadata(&contract);
}

fn assert_mapstore_i64_contract_metadata(contract: &HakoMapStoreI64CallerOrientationContract) {
    assert_eq!(contract.orientation_kind, METADATA_ONLY);
    assert_eq!(contract.scope, SINGLE_SURFACE);
    assert_eq!(contract.runtime_consumer, FORBIDDEN);
    assert_eq!(contract.backend_lowering_consumer, FORBIDDEN);
    assert_eq!(contract.mutation_consumer, FORBIDDEN);
    assert_eq!(contract.publication_consumer, FORBIDDEN);
    assert_eq!(contract.mismatch_policy, FAIL_FAST);
}

pub(super) fn assert_push_arrayappendany_policy_row(policy_row_id: &str) {
    let contract = WRITE_PUSH_ARRAYAPPENDANY_CALLER_ORIENTATION_CONTRACT;
    assert_eq!(
        contract.policy_row_id, policy_row_id,
        "ArrayAppendAny caller-orientation policy row identity drift"
    );
    assert_push_arrayappendany_contract_metadata(&contract);
}

fn assert_push_arrayappendany_contract_metadata(
    contract: &HakoWritePushArrayAppendAnyCallerOrientationContract,
) {
    assert_eq!(contract.orientation_kind, METADATA_ONLY);
    assert_eq!(contract.scope, SINGLE_SURFACE);
    assert_eq!(contract.runtime_consumer, FORBIDDEN);
    assert_eq!(contract.backend_lowering_consumer, FORBIDDEN);
    assert_eq!(contract.mutation_consumer, FORBIDDEN);
    assert_eq!(contract.publication_consumer, FORBIDDEN);
    assert_eq!(contract.mismatch_policy, FAIL_FAST);
}

pub(super) fn assert_mapstore_any_policy_row(policy_row_id: &str) {
    let policy = super::mapstore_route_policy_validator::mapstore_policy(
        super::GenericMethodRouteKind::MapStoreAny,
    );
    super::mapstore_route_policy_validator::assert_mapstore_policy_row(policy);
    let contract = WRITE_SET_MAPSTORE_ANY_CALLER_ORIENTATION_CONTRACT;
    assert_eq!(
        contract.policy_row_id, policy_row_id,
        "MapStoreAny caller-orientation policy row identity drift"
    );
    assert_mapstore_any_contract_metadata(&contract);
}

fn assert_mapstore_any_contract_metadata(contract: &HakoMapStoreAnyCallerOrientationContract) {
    assert_eq!(contract.orientation_kind, METADATA_ONLY);
    assert_eq!(contract.scope, SINGLE_SURFACE);
    assert_eq!(contract.runtime_consumer, FORBIDDEN);
    assert_eq!(contract.backend_lowering_consumer, FORBIDDEN);
    assert_eq!(contract.mutation_consumer, FORBIDDEN);
    assert_eq!(contract.publication_consumer, FORBIDDEN);
    assert_eq!(contract.mismatch_policy, FAIL_FAST);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapload_assertion_accepts_existing_policy_row() {
        assert_mapload_policy_row("map_load_scalar_i64_routes");
    }

    #[test]
    #[should_panic(expected = "policy row identity drift")]
    fn mapload_assertion_rejects_unknown_policy_row() {
        assert_mapload_policy_row("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapload_assertion_rejects_metadata_drift() {
        let mut contract = MAPLOAD_SCALAR_I64_CALLER_ORIENTATION_CONTRACT;
        contract.runtime_consumer = "RuntimeConsumer";
        assert_contract_metadata(&contract);
    }

    #[test]
    fn mapload_authority_pilot_accepts_existing_policy_row() {
        assert_mapload_authority_pilot("map_load_scalar_i64_routes");
    }

    #[test]
    #[should_panic(expected = "policy row identity drift")]
    fn mapload_authority_pilot_rejects_unknown_policy_row() {
        assert_mapload_authority_pilot("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapload_authority_pilot_rejects_policy_metadata_drift() {
        let mut policy = MAPLOAD_SCALAR_I64_HAKO_POLICY;
        policy.role = "caller_selected_route_authority";
        assert_mapload_policy_metadata(&policy);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn string_assertion_rejects_metadata_drift() {
        let mut contract = STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS[0];
        contract.publication_consumer = "RuntimeConsumer";
        assert_string_contract_metadata(&contract);
    }

    #[test]
    fn string_authority_pilot_accepts_exact_policy_set() {
        for policy in STRING_SEARCH_SCALAR_I64_HAKO_POLICIES {
            assert_string_authority_pilot(policy.policy_row_id);
        }
    }

    #[test]
    #[should_panic(expected = "policy row identity drift")]
    fn string_authority_pilot_rejects_unknown_policy_row() {
        assert_string_authority_pilot("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn string_authority_pilot_rejects_policy_metadata_drift() {
        let mut policy = STRING_SEARCH_SCALAR_I64_HAKO_POLICIES[0];
        policy.role = "caller_selected_route_authority";
        assert_string_policy_metadata(&policy);
    }

    #[test]
    fn collection_authority_pilot_accepts_exact_policy_set() {
        for policy in COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES {
            assert_collection_authority_pilot(policy.policy_row_id);
        }
    }

    #[test]
    #[should_panic(expected = "policy row identity drift")]
    fn collection_authority_pilot_rejects_unknown_policy_row() {
        assert_collection_authority_pilot("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn collection_authority_pilot_rejects_receiver_domain_drift() {
        let mut policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES[0];
        policy.receiver_domain = "Box";
        assert_collection_policy_metadata(&policy, "MapBox");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn collection_authority_pilot_rejects_policy_metadata_drift() {
        let mut policy = COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES[0];
        policy.role = "caller_selected_route_authority";
        assert_collection_policy_metadata(&policy, "MapBox");
    }

    #[test]
    fn mapstore_i64_assertion_accepts_existing_policy_row() {
        assert_mapstore_i64_policy_row("map_store_i64_set_surface");
    }

    #[test]
    #[should_panic(expected = "policy row identity drift")]
    fn mapstore_i64_assertion_rejects_unknown_policy_row() {
        assert_mapstore_i64_policy_row("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_i64_assertion_rejects_metadata_drift() {
        let mut contract = WRITE_SET_MAPSTORE_I64_CALLER_ORIENTATION_CONTRACT;
        contract.mutation_consumer = "RuntimeConsumer";
        assert_mapstore_i64_contract_metadata(&contract);
    }

    #[test]
    fn push_arrayappendany_assertion_accepts_existing_policy_row() {
        assert_push_arrayappendany_policy_row("array_append_any_push_surface");
    }

    #[test]
    #[should_panic(expected = "policy row identity drift")]
    fn push_arrayappendany_assertion_rejects_unknown_policy_row() {
        assert_push_arrayappendany_policy_row("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn push_arrayappendany_assertion_rejects_metadata_drift() {
        let mut contract = WRITE_PUSH_ARRAYAPPENDANY_CALLER_ORIENTATION_CONTRACT;
        contract.publication_consumer = "RuntimeConsumer";
        assert_push_arrayappendany_contract_metadata(&contract);
    }

    #[test]
    fn mapstore_any_assertion_accepts_existing_policy_row() {
        assert_mapstore_any_policy_row("map_store_any_set_surface");
    }

    #[test]
    #[should_panic(expected = "policy row identity drift")]
    fn mapstore_any_assertion_rejects_unknown_policy_row() {
        assert_mapstore_any_policy_row("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn mapstore_any_assertion_rejects_metadata_drift() {
        let mut contract = WRITE_SET_MAPSTORE_ANY_CALLER_ORIENTATION_CONTRACT;
        contract.mutation_consumer = "RuntimeConsumer";
        assert_mapstore_any_contract_metadata(&contract);
    }
}
