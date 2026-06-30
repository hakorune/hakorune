# 1964 - MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001

## Token

```text
MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001
```

## Purpose

Filter strict converter emission evidence through the bridge policy and select
one native source seed candidate by stable priority.

This card does not materialize the native seed, generate Hako, run a
HakoAdopted decision, or claim Source Selfhost.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_strict_converter_emission_native_seed_candidate_selection.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_strict_converter_emission_native_seed_candidate_selection_guard.sh
```

## Acceptance

```text
bridge_policy_consumed = 1
strict_converter_emission_probe_consumed = 1
verified_hako_family_ir_count = 47
bridge_eligible_count = 9
bridge_blocked_count = 38

selection_rule:
  manual_family_selection = 0
  cluster_size_as_proof = 0
  coverage_percentage_as_proof = 0
  route_membership_alone_as_proof = 0

selected_owner_edge_id =
  hakorune_mir_builder::core_context

selected_owner_edge_confidence =
  FixtureMapped

selected_next_card =
  MIRBUILDER-CORE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001

native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
generated_artifact_as_native_edit_authority = 0
```

## Result

```text
decision:
  SelectNativeSeedCandidate

reason_token:
  StrictEmissionBridgeEligibleCandidateSelected

selected_next_card:
  MIRBUILDER-CORE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001
```

## Non-Claims

```text
no native source seed materialization
no Hako generation
no HakoAdopted decision
no Source Selfhost claim
no runtime fallback
no new backend route
no new ABI
no new Python SemanticProjector
no runner semantic ownership
```
