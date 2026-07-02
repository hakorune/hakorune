# 2041 - MIRBUILDER-ID-SCALAR-TYPED-EVIDENCE-INDEX-POLICY-001

## Token

```text
MIRBUILDER-ID-SCALAR-TYPED-EVIDENCE-INDEX-POLICY-001
```

## Purpose

Define the typed evidence index required before resolving the two ID scalar
SourcePlanAndRecipe-derivable owners.

This card forbids owner-edge substring matches, fixture-path substring matches,
and mention-only evidence. It indexes only typed fixture rows already produced
by the ID scalar basis cards.

## Input State

```text
current_blocker = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
previous_card = MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-001
derivability_rerun_003 = mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json
input_tied_owner_count = 2
```

## Policy

```text
typed_evidence_index_required = 1
mention_only_owner_edge_text_is_not_evidence = 1
owner_edge_substring_search_allowed = 0
fixture_path_substring_search_allowed = 0
typed_fixture_refs_only = 1
```

## Indexed Evidence Kinds

```text
SourceSurfaceInventory
OperationVocabularyInventory
OwnerScopeBoundedness
NativeSeedFileBoundary
IdDomainBoundary
StateMutationFrame
ErrorSemantics
DeterministicOrder
BehaviorRecipeEffectCoverage
VerifierInputContract
```

## Result

```text
input_tied_owner_count = 2
typed_evidence_complete_owner_count = 2
selection_eligible_count = 0

decision:
  PolicyDefined

reason_token:
  IdScalarTypedEvidenceIndexPolicyDefined

selected_next_card:
  MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001
```

## Boundary

Both tied owners have complete typed evidence rows. This card still does not
select an owner. Evidence completeness is a prerequisite for the later
discriminator resolution, not a SourcePlan materialization decision.

## Acceptance

```text
typed_evidence_rows = 2
all rows include all indexed evidence kinds
typed_evidence_complete_owner_count = 2
selection_eligible_count = 0

owner_edge_text_mention_as_evidence = 0
fixture_path_substring_as_evidence = 0
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
tools/checks/rust_lifecycle_mirbuilder_id_scalar_typed_evidence_index_policy_guard.sh
```
