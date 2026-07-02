# 2046 - MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001

## Token

```text
MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-PROOF-AXIS-REFINEMENT-001
```

## Purpose

Define refined boolean proof axes for the tied ID scalar derivable owners.

This card does not select `context_registry` or `emission_ssa_phi`. It also
does not materialize SourcePlanAndRecipe, generate Hako, or claim Source
Selfhost.

## Refined Axes

```text
PriorProjectionPolicyDisposition
  -> StandaloneProjectionSubjectEstablished

ContractLifecycleDescriptorPresence
  -> LifecycleContractDescriptorCompleteness

LifecycleMutationShape
  -> MutationFrameSemanticCompleteness

VerifierEffectClassPresence
  -> VerifierEffectClassCoverageCompleteness
```

Each raw axis is forbidden as preference. It may only be used through the
refined boolean predicate recorded in the fixture.

## Decision

```text
kind = ProofAxesRefined
reason_token = IdScalarDerivableOwnerProofAxesRefinedWithoutCountOrNameProof
selected_next_card =
  MIRBUILDER-ID-SCALAR-DERIVABLE-OWNER-DISCRIMINATOR-RESOLUTION-002
```

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
tools/checks/rust_lifecycle_mirbuilder_id_scalar_derivable_owner_proof_axis_refinement_guard.sh
```
