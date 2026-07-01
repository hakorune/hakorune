# 1987 - MIRBUILDER-STRICT-DENIED-BOUNDARY-VOCABULARY-NORMALIZATION-001

## Token

```text
MIRBUILDER-STRICT-DENIED-BOUNDARY-VOCABULARY-NORMALIZATION-001
```

## Purpose

Normalize the denied-boundary vocabulary left after
`VerifierBackedResultCarrierProjectionPolicyV1`.

The previous strict candidate rerun proves the three ResultBox carrier gaps are
covered, but the rows remain bridge-blocked because their denied boundaries mix
bounded scope exclusions with forbidden non-claims. This card fixes the
vocabulary boundary before strict native-seed candidate selection reruns.

## Recommended Design

```text
policy_id = StrictDeniedBoundaryVocabularyNormalizationV1

normalize classes:
  ForbiddenNonClaimBoundary
    - runtime_fallback
    - new_backend_route
    - new_abi
    - new_canonical_mir_instruction
    - never becomes seed eligibility evidence

  ScopeExclusionBoundary
    - module_metadata_publication
    - semantic_refresh
    - all_functions_phi_materialization
    - full_finalize_module
    - mainline_selected
    - direct_state_plan_refresh
    - bounded scope exclusion, not carrier/type transport gap

  NarrowRefreshScopeExclusion
    - *_field_value_type_refresh
    - *_collection_field_element_refresh
    - bounded refresh-scope exclusion, not carrier/type transport gap
```

## Input

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-rerun-v0.json

selected rows:
  hakorune_mir_builder::direct_state_plan_refresh
  hakorune_mir_builder::record_packed_layout_refresh
  hakorune_mir_builder::typed_object_plan_refresh
```

## Task Shape

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_strict_denied_boundary_vocabulary_normalization.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-denied-boundary-vocabulary-normalization-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_strict_denied_boundary_vocabulary_normalization_guard.sh
```

## Acceptance

```text
strict_candidate_selection_rerun_consumed = 1
result_carrier_policy_covered_count = 3
denied_boundary_vocabulary_blocked_count = 3
unclassified_denied_boundary_count = 0

classes normalized:
  ForbiddenNonClaimBoundary
  ScopeExclusionBoundary
  NarrowRefreshScopeExclusion

normalized class summary:
  ForbiddenNonClaimBoundary = 12
  NarrowRefreshScopeExclusion = 6
  ScopeExclusionBoundary = 16

forbidden_non_claim_never_proves_seed_eligibility = 1
scope_exclusion_not_transport_gap = 1
narrow_refresh_scope_exclusion_not_transport_gap = 1

selected_next_card =
  MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-NORMALIZED-RERUN-001
```

## Non-Claims

```text
manual_boundary_reclassification = 0
seed_eligibility_selected = 0
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

## Recovery

```text
if unclassified_denied_boundary_count > 0:
  selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
  reason_token = StrictDeniedBoundaryVocabularyRequiresDesignConsultation

if all boundaries normalize:
  selected_next_card =
    MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-NORMALIZED-RERUN-001
```
