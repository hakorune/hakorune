# 2040 - MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-001

## Token

```text
MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-001
```

## Purpose

Define the allowed machine-derived discriminator axes for choosing between the
two ID scalar SourcePlanAndRecipe-derivable owners reported by 2039.

This card does not select `context_registry` or `emission_ssa_phi`. It records
which evidence can be used by a later resolver, which evidence is only a
tie-break signal, and which axes remain forbidden.

## Input State

```text
current_blocker = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
previous_card = MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003
previous_reason_token = MultipleEqualIdScalarSourcePlanDerivabilityCandidates
source_plan_derivable_count = 2
selection_eligible_count = 2
tied_owner_edges = mirbuilder::context_registry, mirbuilder::emission_ssa_phi
```

## Allowed Proof Axes

```text
TypedEvidenceIndexCompleteness:
  source plan, recipe, verifier input, and seed evidence are typed fixture refs,
  not owner-edge text mentions.

VerifierInputContractCompleteness:
  verifier preconditions and input facts are complete for the owner.

NativeSeedFileBoundaryDeterminism:
  native seed path, module export, and overwrite guard are derived from
  owner_edge + state targets + effect classes.

StateTargetClosureQuality:
  state targets are inside owner scope or explicitly declared external deps.

OperationEffectClassCompleteness:
  operation tokens are normalized into complete behavior-recipe effect classes.

SourcePlanRecipeComponentReadiness:
  source surfaces, mutation frame, error semantics, deterministic order, and
  verifier input contract are typed and complete.
```

## Tie-Break Signals Only

```text
AlreadyHakoAdoptedAdjacency
MinimalPathProximity
MigrationUnblockValue
```

These are not standalone proof. They may only break ties after proof axes have
established a real evidence-quality difference.

## Forbidden Selection Axes

```text
OwnerName
LexicalOrder
SurfaceCount
RowCount
ClusterSize
CoveragePercentage
RouteMembershipAlone
ManualOwnerPreference
```

`context_registry` having one surface is diagnostic only. `emission_ssa_phi`
having richer lifecycle coverage is diagnostic only unless expressed through a
typed proof axis above.

## Authority Rules

```text
typed_evidence_index_required = 1
mention_only_owner_edge_text_is_not_evidence = 1
fixture_declared_role_required_for_semantic_operation_mapping = 1
operation_name_fallback_is_diagnostic_only = 1
shape_name_is_provenance_not_semantic_policy = 1
eligible_zero_or_lexical_sort_selection_forbidden = 1
```

## Selected Next Tasks

```text
1. MIRBUILDER-ID-SCALAR-TYPED-EVIDENCE-INDEX-POLICY-001
   Define typed evidence rows and forbid owner_edge_id substring evidence.

2. MIRBUILDER-ID-SCALAR-OPERATION-VOCABULARY-AUTHORITY-SPLIT-001
   Split semantic role authority from diagnostic name/type suggestions.

3. MIRBUILDER-SEMANTIC-SELECTOR-NO-LEXICAL-TIEBREAK-GUARD-001
   Add a cross-selector guard against eligible[0], lexical tuple, owner-name,
   fixture-name, and manifest-order semantic selection.

4. MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-001
   Apply the basis after typed evidence and selector guardrails exist.
```

## Acceptance

```text
derivability_rerun_003_consumed = 1
previous_reason_token = MultipleEqualIdScalarSourcePlanDerivabilityCandidates
source_plan_derivable_count = 2
selection_eligible_count = 2

allowed_proof_axes_defined = 1
tie_break_signals_are_not_proof = 1
forbidden_selection_axes_defined = 1
typed_evidence_index_required = 1
operation_name_fallback_is_diagnostic_only = 1
shape_name_is_provenance_not_semantic_policy = 1
eligible_zero_or_lexical_sort_selection_forbidden = 1

manual_owner_selection = 0
owner_name_as_proof = 0
lexical_order_as_proof = 0
surface_count_as_proof = 0
row_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
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

## Recovery Tokens

```text
IdScalarDerivableOwnerDiscriminatorBasisMissing
IdScalarTypedEvidenceIndexRequired
IdScalarOperationVocabularyAuthorityFallbackOnly
MultipleEqualIdScalarDerivableOwnerDiscriminatorCandidates
```
