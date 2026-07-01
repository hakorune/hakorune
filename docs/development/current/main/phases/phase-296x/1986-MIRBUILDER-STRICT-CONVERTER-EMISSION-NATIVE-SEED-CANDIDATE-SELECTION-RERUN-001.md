# 1986 - MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-001

## Token

```text
MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-001
```

## Purpose

Rerun strict native-seed candidate selection after
`VerifierBackedResultCarrierProjectionPolicyV1`.

The result carrier policy covers the ResultBox carrier gap for three rows, but
the rows still include denied-boundary vocabulary that mixes scope exclusions
and forbidden non-claims. This card does not turn those rows into native seed
candidates.

## Result

```text
base_verified_hako_family_ir_count = 47
base_bridge_eligible_count = 0
result_carrier_policy_covered_count = 3
bridge_eligible_after_policy_count = 0
denied_boundary_vocabulary_blocked_count = 3
unclassified_denied_boundary_count = 0

denied boundary classes:
  ForbiddenNonClaimBoundary = 3
  NarrowRefreshScopeExclusion = 3
  ScopeExclusionBoundary = 3

selected_next_card:
  MIRBUILDER-STRICT-DENIED-BOUNDARY-VOCABULARY-NORMALIZATION-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_strict_converter_emission_native_seed_candidate_selection_rerun_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
generated_artifact_as_native_edit_authority = 0
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
