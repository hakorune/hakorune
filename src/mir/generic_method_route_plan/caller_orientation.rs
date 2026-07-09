use super::generated::mapload_scalar_i64_caller_orientation_contract::{
    HakoMapLoadCallerOrientationContract, MAPLOAD_SCALAR_I64_CALLER_ORIENTATION_CONTRACT,
};
use super::generated::string_scalar_i64_caller_orientation_contract::{
    HakoStringCallerOrientationContract, STRING_SCALAR_I64_CALLER_ORIENTATION_CONTRACTS,
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
}
