# 1991 - MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-002

## Token

```text
MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-002
```

## Purpose

Rerun strict native-seed candidate selection after BridgePolicyV2.

BridgePolicyV2 allows mention-only forbidden non-claims to stop blocking a
clean selected narrow seed surface, while keeping them non-evidence and keeping
runtime fallback / backend / ABI / canonical-MIR claims forbidden.

## Result

```text
input_owner_edge_count = 3
bridge_eligible_after_bridge_policy_v2_count = 3
bridge_blocked_after_bridge_policy_v2_count = 0
selected_candidate_count = 1

selected_owner_edge_id =
  hakorune_mir_builder::direct_state_plan_refresh

selected_next_card =
  MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-002-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_002.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_002_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
seed_eligibility_from_forbidden_nonclaim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
generated_artifact_as_native_edit_authority = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runner_semantic_owner = 0
```
