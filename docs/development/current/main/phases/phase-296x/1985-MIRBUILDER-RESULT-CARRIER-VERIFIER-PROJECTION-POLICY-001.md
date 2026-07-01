# 1985 - MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-RESULT-CARRIER-VERIFIER-PROJECTION-POLICY-001
```

## Purpose

Select the projection policy for `ResultCarrierVerifierContractV1`.

This policy accepts ResultBox carriers only when a verifier contract proves the
ResultBox transport, projection contract, canonical JSON parity, and
`runtime_fallback = 0`.

## Selected Policy

```text
policy_id = VerifierBackedResultCarrierProjectionPolicyV1
applies_to_contract = ResultCarrierVerifierContractV1
projection_boundary = ResultBox carrier is accepted only with verifier contract evidence
hako_projection_selected = 0
candidate_rerun_required = 1
```

## Result

```text
result_carrier_projection_policy_row_count = 3
result_carrier_projection_policy_selected = 1

selected_next_card:
  MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-result-carrier-verifier-projection-policy-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_result_carrier_verifier_projection_policy.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_result_carrier_verifier_projection_policy_guard.sh
```

## Non-Claims

```text
hako_projection_selected = 0
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
