---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Fix the machine policy for selecting the next Source Selfhost
  route-family after the bounded VariableContext native surface closeout.
Related:
  - docs/development/current/main/phases/phase-296x/1797-MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-NEED-RESOLVER-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-next-route-family-selection-policy-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-hako-adoption-decision-v0.json
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - tools/checks/rust_lifecycle_source_selfhost_next_route_family_selection_policy_guard.sh
---

# SOURCE-SELFHOST-NEXT-ROUTE-FAMILY-SELECTION-POLICY-001

## Goal

Define the next route-family selection policy after VariableContext bounded
native adoption. The policy decides whether the next Source Selfhost step is a
family-specific HakoAdopted decision, a route repair, a projector promotion, or
a consultation-gated stop.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
machine_checkable_fixture_required = 1
```

## Decision

```text
policy:
  NextRouteFamilySelectionPolicy

output_contract:
  rust-lifecycle-source-selfhost-next-route-family-selection-policy-v0

current_decision:
  KeepSourceSelfhostStopped

reason_token:
  NoEligibleNativeAdoptionCandidate

next_action:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

The current evidence has no machine-derived family-specific HakoAdopted
candidate after excluding already adopted families, bounded-only
VariableContext, support-lane projectors, and consultation-gated wider routes.

## Input Authority

```text
authority:
  route manifests
  route matrix fixtures
  HakoAdopted decision fixtures
  HakoShadow / HakoMainline stage-state fixtures
  Derived-to-Native model SSOT
  selfhost roadmap SSOT

pointer only:
  CURRENT_STATE.toml
  mirbuilder-rust-to-hako-converter-task-order-ssot.md
```

`CURRENT_STATE.toml` and task-order are not candidate eligibility authority.
They only point at the current card, latest card, and thin queue state.

## Classification Policy

Every route-family row consumed by the policy must land in exactly one
classification.

```text
AlreadyAdopted
BoundedSurfaceAdopted
SupportLaneOnly
NeedsRouteRepair
NeedsNativeAdoptionDecision
NeedsHakoProjectorPromotion
ConsultationGated
NoEligibleCandidate
```

Allowed subclassification tokens:

```text
ParkedFullClaim
ParkedMutLeaseRequired
ParkedWiderRouteRequired
```

## Stable Selection Rules

```text
1. CandidateEligible route-family rows are collected from machine fixtures.
2. AlreadyAdopted rows are excluded from adoption candidate selection.
3. BoundedSurfaceAdopted rows with full claim parked are excluded.
4. SupportLaneOnly rows are not family adoption candidates.
5. NeedsRouteRepair wins before adoption selection.
6. If multiple native adoption candidates remain, use stable priority:
   route_state_priority,
   native_surface_completeness,
   mainline_route_presence,
   family_id lexical order.
7. If no candidate remains, emit a blocked decision with recovery guidance.
```

## Blocked Diagnostic

```text
[mirbuilder:source-selfhost][next-route-family:blocked]
reason=NoEligibleNativeAdoptionCandidate
candidate_pool_state=Blocked
summary=No machine-derived family-specific HakoAdopted candidate remains after excluding already-adopted families, bounded-surface-only VariableContext, support-lane projector stages, and consultation-gated wider routes.
next_action=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
recovery=Open a consultation-gated wider-route selection or provide a machine-derived route repair fixture that makes one family CandidateEligible.
do_not=manual family selection,route membership alone as proof,coverage percentage,bundle size,runtime fallback,new backend route,new ABI
```

## Acceptance

```text
manual_family_selection = 0
route_membership_alone_as_proof = 0
coverage_percentage_as_proof = 0
bundle_size_as_proof = 0
support_lane_projector_as_adoption_candidate = 0
classification_partition_complete = 1
exactly_one_decision = 1
blocked_result_has_recovery_message = 1
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
rust_deletion = 0
task_order_delta_is_pointer_only = 1
```

## Non-Claims

```text
new HakoAdopted family = 0
full VariableContext = 0
Source Selfhost = 0
Rust deletion = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
new Python SemanticProjector = 0
```

## Closeout

```text
output_contract=rust-lifecycle-source-selfhost-next-route-family-selection-policy-v0
decision=KeepSourceSelfhostStopped
reason_token=NoEligibleNativeAdoptionCandidate
next_action=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
classification_partition_complete=1
manual_family_selection=0
route_membership_alone_as_proof=0
support_lane_projector_as_adoption_candidate=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
summary=ok
```
