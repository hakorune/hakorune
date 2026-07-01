# 1997 - MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-004

## Token

```text
MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-004
```

## Purpose

Rerun strict native-seed candidate selection after
`record_packed_layout_refresh` adoption.

The rerun consumes rerun 003, excludes adopted `direct_state_plan_refresh` and
`record_packed_layout_refresh`, and selects the remaining bridge-eligible owner.

## Result

```text
input_owner_edge_count = 3
already_hako_adopted_count = 2
bridge_eligible_remaining_count = 1
selected_candidate_count = 1

selected_owner_edge_id =
  hakorune_mir_builder::typed_object_plan_refresh

selected_next_card =
  MIRBUILDER-TYPED-OBJECT-PLAN-REFRESH-HAKO-NATIVE-SOURCE-SEED-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-004-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_004.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_004_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
new_python_semantic_projector = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runner_semantic_owner = 0
```
