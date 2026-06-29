# 1823 - SOURCE-SELFHOST-WIDER-ROUTE-UNBLOCK-TASK-BREAKDOWN-001

## Token

```text
SOURCE-SELFHOST-WIDER-ROUTE-UNBLOCK-TASK-BREAKDOWN-001
```

## Purpose

Taskize the Source Selfhost design stop into a small, machine-derived unblock
lane. This card does not resume Source Selfhost by hand and does not select a
family. It records the next resolver and the conditional follow-up cards that
are allowed to move the lane forward.

## Design

The current stop is valid: the converter is not broken, but no eligible native
adoption candidate is machine-derived from the current route evidence.

The clean design is:

```text
GeneratedArtifactOnly
  -> resolve a leaf NativeOwnerSeedCandidate by policy
  -> materialize native .hako source owner for that leaf
  -> run that leaf's HakoAdopted decision
```

The composed closure itself remains an integration owner, not a semantic family
owner. Support-lane projectors and bounded-only surfaces remain excluded from
family adoption candidate selection.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-wider-route-unblock-task-breakdown-v0.json

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_family_guard.sh
```

No dedicated row guard is added. The family guard owns this row so that docs
maintenance cost does not grow per card.

## Task Order

```text
1. MIRBUILDER-GENERATED-ARTIFACT-NATIVE-OWNER-SEED-CANDIDATE-RESOLUTION-001
   Resolve whether exactly one leaf semantic owner satisfies the generated
   artifact -> native owner seed policy.

2. <ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001
   Conditional only after a resolver emits exactly one concrete
   route-matrix inconsistency.

3. <LEAF-OWNER>-HAKO-NATIVE-SOURCE-OWNER-SEED-001
   Conditional only after exactly one machine-derived leaf semantic owner is
   classified as NativeOwnerSeedCandidate.

4. <LEAF-OWNER>-HAKO-ADOPTION-DECISION-001
   Conditional only after that leaf's native source owner seed is green.
```

## Acceptance

```text
current_blocker_preserved =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

next_unblock_card =
  MIRBUILDER-GENERATED-ARTIFACT-NATIVE-OWNER-SEED-CANDIDATE-RESOLUTION-001

manual_family_selection = 0
composition_owner_as_semantic_owner = 0
support_lane_projector_as_adoption_candidate = 0
generated_artifact_as_edit_authority = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no Source Selfhost claim
no family adoption decision
no route repair without a concrete inconsistency
no manual candidate selection
no new guard script for this row
```
