# 2042 - MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001

## Token

```text
MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001
```

## Purpose

Split semantic operation authority from diagnostic operation suggestions for
the ID scalar SourcePlan lane.

This card does not change source operation inventory. It defines that only
fixture-declared role mappings are semantic authority. Symbol / return-type
fallbacks remain diagnostic suggestions and cannot select a SourcePlan owner.

## Input State

```text
current_blocker = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
typed_evidence_index_policy = mirbuilder-id-scalar-typed-evidence-index-policy-v0.json
operation_vocabulary_inventory = mirbuilder-id-scalar-operation-vocabulary-inventory-v0.json
derivability_rerun_003 = mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json
```

## Authority Policy

```text
semantic_operation_authority = FixtureDeclaredRoleMapped
diagnostic_operation_suggestion = SymbolReturnTypeMapped
symbol_return_type_fallback_is_semantic_authority = 0
diagnostic_suggestion_may_select_source_plan_owner = 0
fixture_declared_role_required_for_semantic_operation_mapping = 1
operation_name_fallback_is_diagnostic_only = 1
```

## Result

```text
semantic_role_mapped_operation_count = 94
diagnostic_suggestion_operation_count = 8
unknown_operation_count = 0
tied_derivable_owner_count = 2
tied_semantic_authority_complete_owner_count = 2
selection_eligible_count = 0

decision:
  PolicyDefined

reason_token:
  IdScalarOperationVocabularyAuthoritySplitDefined

selected_next_card:
  MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001
```

## Boundary

The two tied derivable owners have complete semantic role-mapped operation
authority. The eight diagnostic suggestion rows are outside the tied owner set
and cannot be used as SourcePlan owner selection proof.

## Acceptance

```text
semantic_role_mapped_operation_count = 94
diagnostic_suggestion_operation_count = 8
tied_semantic_authority_complete_owner_count = 2
selection_eligible_count = 0

symbol_return_type_fallback_as_semantic_authority = 0
diagnostic_suggestion_as_source_plan_selection_proof = 0
manual_owner_selection = 0
source_plan_materialization = 0
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
runner_semantic_owner = 0
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_id_scalar_operation_vocabulary_authority_split_guard.sh
```
