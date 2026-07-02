# 2044 - MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001

## Token

```text
MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001
```

## Purpose

Apply the ID scalar derivable-owner discriminator basis after the typed evidence
index, operation vocabulary authority split, and no-lexical selector guard.

This card must either select exactly one SourcePlanAndRecipe owner by allowed
proof axes or keep Source Selfhost stopped. It must not select by owner name,
surface count, row count, lexical order, route membership alone, or manual
preference.

## Inputs

```text
derivability_rerun_003 =
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-source-plan-and-recipe-derivability-resolution-003-v0.json

discriminator_basis =
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-derivable-owner-discriminator-basis-v0.json

typed_evidence_index_policy =
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-typed-evidence-index-policy-v0.json

operation_vocabulary_authority_split =
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-operation-vocabulary-authority-split-v0.json

selector_guard =
  docs/development/current/main/design/fixtures/rust-lifecycle/
    semantic-selector-no-lexical-tiebreak-guard-v0.json
```

## Result

```text
input_derivable_owner_count = 2
selection_eligible_count = 2
unique_proof_tuple_count = 1
selected_owner_count = 0

tied_owner_edges:
  mirbuilder::context_registry
  mirbuilder::emission_ssa_phi

decision = KeepStopped
reason_token = MultipleEqualIdScalarDerivableOwnerDiscriminatorCandidates
selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

Both tied owners satisfy the currently allowed proof axes:

```text
TypedEvidenceIndexCompleteness
VerifierInputContractCompleteness
NativeSeedFileBoundaryDeterminism
StateTargetClosureQuality
OperationEffectClassCompleteness
SourcePlanRecipeComponentReadiness
SemanticOperationAuthorityComplete
SelectorGuardClean
```

The resolver therefore cannot choose a SourcePlanAndRecipe owner without adding
a new non-count, non-name, machine-derived discriminator.

## Claims

```text
manual_owner_selection = 0
owner_name_as_proof = 0
lexical_order_as_proof = 0
surface_count_as_proof = 0
row_count_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
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
tools/checks/rust_lifecycle_mirbuilder_id_scalar_derivable_owner_discriminator_resolution_guard.sh
```
