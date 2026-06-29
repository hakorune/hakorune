# 1810 - SOURCE-SELFHOST-POST-MAINTENANCE-TASK-INVENTORY-001

## Token

```text
SOURCE-SELFHOST-POST-MAINTENANCE-TASK-INVENTORY-001
```

## Purpose

Fix the post-maintenance task order after the docs/guard cleanup phase.

The Source Selfhost lane remains stopped at the wider route-selection design
stop. The next implementation task is not a manual family selection; it is a
native-slice decomposition of the minimal-path composed closure.

## Current State

```text
maintenance_phase:
  complete

current_blocker_token:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

next_semantic_task:
  MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
```

## Task Inventory

```text
1. MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
   type = implementation / fixture / guard
   goal = split the consultation-gated composed route into native-adoptable
          slices without selecting a family by hand

2. <ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001
   type = conditional implementation
   open only if decomposition emits exactly one ConcreteRouteMatrixInconsistency

3. <SELECTED-SLICE>-HAKO-ADOPTION-DECISION-001
   type = conditional decision
   open only if decomposition emits exactly one CandidateEligible native slice

4. SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
   type = stop line
   stay here if decomposition emits no candidate, multiple candidates, or a
   consultation-gated public boundary
```

## Decomposition Inputs

```text
source-selfhost-wider-route-selection-resolution-v0
source-selfhost-family-guard-manifest-v0
mirbuilder-minimal-path-mainline-readiness-resolution-v0
mirbuilder-minimal-path-mainline-pilot-v0
derived-to-native-hako-artifact-model-ssot
mirbuilder-selfhost-checkpoint-roadmap-ssot
```

## Decomposition Output Shape

```text
MirBuilderMinimalPathComposedClosureNativeSliceDecompositionV1:
  composed_route_id
  slice[]
    slice_id
    source_authority
    route_state
    native_owner_state
    candidate_classification
    reason_token
  decision
    KeepStopped
    SelectRouteRepair
    SelectFamilyAdoptionDecision
```

## Stop Conditions

```text
manual family selection required
multiple CandidateEligible slices
new ABI required
new backend route required
runtime fallback required
VM/interpreter semantic owner required
generated artifact used as semantic/edit authority
Source Selfhost claim requested
```

## Acceptance

```text
task_order_lines < 800
CURRENT_STATE remains compact
current_blocker_token =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no route repair
no family adoption decision
no Source Selfhost claim
```
