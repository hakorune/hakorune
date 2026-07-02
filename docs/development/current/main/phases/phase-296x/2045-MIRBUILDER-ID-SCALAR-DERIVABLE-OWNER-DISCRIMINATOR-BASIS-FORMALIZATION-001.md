# 2045 - MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-FORMALIZATION-001

## Token

```text
MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-BASIS-FORMALIZATION-001
```

## Purpose

Materialize the missing machine-readable basis fixture referenced by 2044.

This is a fixture authority repair. It does not add a new proof axis, select an
owner, materialize SourcePlanAndRecipe, generate Hako, or claim Source
Selfhost.

## Result

```text
formalized_fixture =
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-derivable-owner-discriminator-basis-v0.json

decision = BasisFixtureMaterialized
reason_token = IdScalarDerivableOwnerDiscriminatorBasisFixtureMaterialized
selected_next_card =
  MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001
```

The materialized basis fixture preserves the 2040 rules:

```text
allowed proof axes:
  TypedEvidenceIndexCompleteness
  VerifierInputContractCompleteness
  NativeSeedFileBoundaryDeterminism
  StateTargetClosureQuality
  OperationEffectClassCompleteness
  SourcePlanRecipeComponentReadiness
  SemanticOperationAuthorityComplete
  SelectorGuardClean

tie-break signals only:
  AlreadyHakoAdoptedAdjacency
  MinimalPathProximity
  MigrationUnblockValue

forbidden selection axes:
  OwnerName
  LexicalOrder
  SurfaceCount
  RowCount
  ClusterSize
  CoveragePercentage
  RouteMembershipAlone
  ManualOwnerPreference
```

## Claims

```text
manual_owner_selection = 0
owner_name_as_proof = 0
lexical_order_as_proof = 0
surface_count_as_proof = 0
row_count_as_proof = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
source_plan_materialization = 0
behavior_recipe_materialization = 0
verifier_result_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_id_scalar_derivable_owner_discriminator_basis_formalization_guard.sh
```
