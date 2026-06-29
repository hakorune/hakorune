---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Keep the Source Selfhost lane explicitly stopped while the next wider
  route-selection basis is consultation-gated or machine-repaired.
Related:
  - docs/development/current/main/phases/phase-296x/1798-SOURCE-SELFHOST-NEXT-ROUTE-FAMILY-SELECTION-POLICY-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-design-stop-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-next-route-family-selection-policy-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/next-hako-adoption-candidate-selection-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-blocked-recovery-diagnostic-v0.json
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-adoption-plan-v0.json
  - tools/checks/rust_lifecycle_source_selfhost_wider_route_selection_design_stop_guard.sh
---

# SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

## Goal

Make the wider route-selection stop line explicit after the next route-family
policy reports no eligible native adoption candidate. The stop is not a family
selection, not a route repair, and not a new projection lane. It preserves the
blocked Source Selfhost state while naming the only allowed resume conditions.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Resolution

```text
decision:
  KeepSourceSelfhostStopped

output_contract:
  rust-lifecycle-source-selfhost-wider-route-selection-design-stop-v0

reason_token:
  NoEligibleNativeAdoptionCandidate

next_action:
  DesignConsultationRequired

resume_condition:
  ConsultationGatedWiderRouteSelectionOrMachineDerivedRouteRepair
```

The stop line is the current truth. Do not hand-pick a family, do not widen
VariableContext again, and do not interpret support-lane projectors as adoption
candidates.

## Input Authority

```text
authority:
  SOURCE-SELFHOST-NEXT-ROUTE-FAMILY-SELECTION-POLICY-001
  source-selfhost-blocked-recovery-diagnostic-v0
  next-hako-adoption-candidate-selection-v0
  source-selfhost-adoption-plan-v0
  CURRENT_STATE.toml
  mirbuilder-rust-to-hako-converter-task-order-ssot.md

pointer only:
  CURRENT_STATE.toml
  mirbuilder-rust-to-hako-converter-task-order-ssot.md
```

`CURRENT_STATE.toml` and task-order are pointer-only. They do not create new
candidate eligibility.

## Recovery Boundary

The only allowed resume paths are:

```text
1. ConsultationGatedWiderRouteSelection
2. MachineDerivedRouteRepair
```

If a future fixture makes exactly one family CandidateEligible, the queue may
resume from that machine-derived route repair. Until then, the Source Selfhost
lane remains stopped.

## Acceptance

```text
candidate_pool_state = Blocked
manual_family_selection = 0
route_membership_alone_as_proof = 0
coverage_percentage_as_proof = 0
bundle_size_as_proof = 0
support_lane_projector_as_adoption_candidate = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
rust_deletion = 0
consultation_gated_wider_route_selection = 1
machine_derived_route_repair_allowed = 1
```

## Non-Claims

```text
new HakoAdopted family = 0
new route family selected = 0
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
output_contract=rust-lifecycle-source-selfhost-wider-route-selection-design-stop-v0
decision=KeepSourceSelfhostStopped
reason_token=NoEligibleNativeAdoptionCandidate
next_action=DesignConsultationRequired
resume_condition=ConsultationGatedWiderRouteSelectionOrMachineDerivedRouteRepair
candidate_pool_state=Blocked
manual_family_selection=0
route_membership_alone_as_proof=0
support_lane_projector_as_adoption_candidate=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
summary=ok
```
