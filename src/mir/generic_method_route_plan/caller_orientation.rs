use super::generated::mapload_scalar_i64_caller_orientation_contract::{
    HakoMapLoadCallerOrientationContract, MAPLOAD_SCALAR_I64_CALLER_ORIENTATION_CONTRACT,
};
use super::generated::string_scalar_i64_caller_orientation_contract::{
    HakoStringCallerOrientationContract, STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS,
};
use super::generated::collection_scalar_i64_caller_orientation_contract::{
    HakoCollectionCallerOrientationContract, COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS,
};
use super::generated::write_set_mapstore_i64_caller_orientation_contract::{
    HakoMapStoreI64CallerOrientationContract,
    WRITE_SET_MAPSTORE_I64_CALLER_ORIENTATION_CONTRACT,
};
use super::generated::write_push_arrayappendany_caller_orientation_contract::{
    HakoWritePushArrayAppendAnyCallerOrientationContract,
    WRITE_PUSH_ARRAYAPPENDANY_CALLER_ORIENTATION_CONTRACT,
};
use super::generated::write_set_mapstore_any_caller_orientation_contract::{
    HakoMapStoreAnyCallerOrientationContract, WRITE_SET_MAPSTORE_ANY_CALLER_ORIENTATION_CONTRACT,
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

pub(super) fn assert_string_policy_row(policy_row_id: &str) {
    let contract = STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS
        .iter()
        .find(|contract| contract.policy_row_id == policy_row_id)
        .expect("String caller-orientation policy row identity drift");
    assert_string_contract_metadata(contract);
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

pub(super) fn assert_collection_policy_row(policy_row_id: &str) {
    let contract = COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS
        .iter()
        .find(|contract| contract.policy_row_id == policy_row_id)
        .expect("Collection caller-orientation policy row identity drift");
    assert_collection_contract_metadata(contract);
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

pub(super) fn assert_mapstore_i64_policy_row(policy_row_id: &str) {
    let contract = WRITE_SET_MAPSTORE_I64_CALLER_ORIENTATION_CONTRACT;
    assert_eq!(
        contract.policy_row_id, policy_row_id,
        "MapStoreI64 caller-orientation policy row identity drift"
    );
    assert_mapstore_i64_contract_metadata(&contract);
}

fn assert_mapstore_i64_contract_metadata(
    contract: &HakoMapStoreI64CallerOrientationContract,
) {
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
    fn string_assertion_accepts_all_existing_policy_rows() {
        for row_id in [
            "string_indexof_scalar_i64_routes",
            "string_lastindexof_scalar_i64_routes",
            "string_contains_scalar_i64_routes",
        ] {
            assert_string_policy_row(row_id);
        }
    }

    #[test]
    #[should_panic(expected = "String caller-orientation policy row identity drift")]
    fn string_assertion_rejects_unknown_policy_row() {
        assert_string_policy_row("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn string_assertion_rejects_metadata_drift() {
        let mut contract = STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS[0];
        contract.publication_consumer = "RuntimeConsumer";
        assert_string_contract_metadata(&contract);
    }

    #[test]
    fn collection_assertion_accepts_all_existing_policy_rows() {
        for row_id in [
            "collection_map_entry_count_scalar_i64_routes",
            "collection_array_slot_len_scalar_i64_routes",
            "collection_string_len_scalar_i64_routes",
            "collection_any_length_scalar_i64_routes",
        ] {
            assert_collection_policy_row(row_id);
        }
    }

    #[test]
    #[should_panic(expected = "Collection caller-orientation policy row identity drift")]
    fn collection_assertion_rejects_unknown_policy_row() {
        assert_collection_policy_row("unknown_policy_row");
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn collection_assertion_rejects_metadata_drift() {
        let mut contract = COLLECTION_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS[0];
        contract.mutation_consumer = "RuntimeConsumer";
        assert_collection_contract_metadata(&contract);
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
