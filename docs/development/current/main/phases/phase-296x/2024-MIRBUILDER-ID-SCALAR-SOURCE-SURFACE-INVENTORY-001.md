# 2024 - MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001

## Token

```text
MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001
```

## Purpose

Inventory the required source surfaces for the four ID scalar packet candidates
from existing projection-policy fixture evidence.

This card does not derive `SourcePlanAndRecipe`; it only proves that the source
surface set can be machine-derived before operation vocabulary classification.

## Result

```text
input_candidate_count = 4
required_source_surface_count = 102
surface_complete_candidate_count = 4
surface_incomplete_candidate_count = 0
selection_eligible_for_source_plan_count = 0

decision:
  SelectOperationVocabularyInventory

reason_token:
  IdScalarRequiredSourceSurfacesInventoried

selected_next_card:
  MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001
```

## Candidate Surface Counts

```text
mirbuilder::context_registry = 1
mirbuilder::emission_ssa_phi = 13
mirbuilder::join_i_r_plan = 35
mirbuilder::join_i_r_route_verify = 53
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-source-surface-inventory-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_source_surface_inventory.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_source_surface_inventory_guard.sh
```

## Non-Claims

```text
manual_surface_selection = 0
source_plan_materialization = 0
operation_vocabulary_evaluated = 0
behavior_recipe_materialization = 0
verifier_result_materialization = 0
derived_artifact_seed_draft_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```
