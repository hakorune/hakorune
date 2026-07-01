# 2025 - MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001

## Token

```text
MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-INVENTORY-001
```

## Purpose

Classify operation vocabulary for the 102 ID scalar source surfaces inventoried
by the previous card.

This card is an inventory only. It does not materialize `SourcePlanAndRecipe`,
behavior recipes, verifier results, seed drafts, native source seeds, or Hako.

## Result

```text
input_candidate_count = 4
operation_surface_count = 102
operation_vocabulary_token_count = 28
operation_vocabulary_complete_candidate_count = 4
unknown_operation_count = 0
selection_eligible_for_source_plan_count = 0

decision:
  SelectSourcePlanAndRecipeDerivabilityRerun

reason_token:
  IdScalarOperationVocabularyInventoried

selected_next_card:
  MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-002
```

## Boundary

Operation tokens are derived from source-surface roles first, then from stable
symbol/return-type rules for rows that were previously role-unclassified.

```text
classification_authority = surface_role_then_symbol_return_type_rule_table
manual_operation_selection = 0
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_operation_vocabulary_inventory.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_operation_vocabulary_inventory_guard.sh
```

## Non-Claims

```text
source_plan_materialization = 0
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
