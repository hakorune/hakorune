# 2047 - MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002

## Token

```text
MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002
```

## Purpose

Apply the refined ID scalar proof axes from 2046.

This card may select exactly one SourcePlanAndRecipe owner, but it must not
materialize SourcePlanAndRecipe, generate Hako, adopt Hako, or claim Source
Selfhost.

## Result

```text
input_derivable_owner_count = 2
selection_eligible_count = 2
unique_refined_proof_tuple_count = 2
selected_owner_count = 1

selected_owner_edge_id = mirbuilder::emission_ssa_phi
decision = SelectSourcePlanAndRecipe
reason_token = ExactlyOneIdScalarDerivableOwnerAfterRefinedProofAxes
selected_next_card =
  MIRBUILDER-EMISSION_SSA_PHI-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-001
```

The selected owner is not chosen by owner name, richness, row count, surface
count, or operation/effect count. It is selected because refined boolean proof
axes establish a standalone projection subject and typed lifecycle contract
readiness.

## Claims

```text
owner_name_as_proof = 0
historical_descriptor_presence_as_preference = 0
lifecycle_richness_as_proof = 0
mutation_complexity_as_proof = 0
effect_class_count_as_proof = 0
surface_count_as_proof = 0
row_count_as_proof = 0
coverage_percentage_as_proof = 0
source_plan_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_id_scalar_derivable_owner_discriminator_resolution_002_guard.sh
```
