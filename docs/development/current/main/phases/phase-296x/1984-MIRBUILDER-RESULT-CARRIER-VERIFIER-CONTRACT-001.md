# 1984 - MIRBUILDER-RESULT-CARRIER-VERIFIER-CONTRACT-001

## Token

```text
MIRBUILDER-RESULT-CARRIER-VERIFIER-CONTRACT-001
```

## Purpose

Materialize the verifier contract for `ResultCarrierVerifierPolicyV1`.

This card verifies the selected ResultBox carrier rows have enough evidence to
support a later projection policy card. It does not emit Hako, materialize a
native seed, or claim Source Selfhost.

## Contract

```text
contract_id = ResultCarrierVerifierContractV1

required invariants:
  result_transport ends with ResultBox
  projection_contract present
  canonical_json_parity = 1
  runtime_fallback = 0
```

## Result

```text
result_carrier_contract_row_count = 3
result_carrier_contract_ready = 1

selected_next_card:
  MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-result-carrier-verifier-contract-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_result_carrier_verifier_contract.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_result_carrier_verifier_contract_guard.sh
```

## Non-Claims

```text
manual_carrier_selection = 0
owner_name_as_transport_policy = 0
cluster_size_as_proof = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
