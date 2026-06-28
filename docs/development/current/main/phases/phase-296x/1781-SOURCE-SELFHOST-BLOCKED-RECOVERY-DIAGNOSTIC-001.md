---
Status: Active
Decision: accepted
Date: 2026-06-28
Scope: Add a machine-checkable recovery diagnostic for the blocked
  source-selfhost adoption candidate pool.
Related:
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/phases/phase-296x/1780-SOURCE-SELFHOST-ADOPTION-PLAN-001.md
  - docs/development/current/main/phases/phase-296x/1775-MIRBUILDER-NEXT-HAKO-ADOPTION-CANDIDATE-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/1774-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-blocked-recovery-diagnostic-v0.json
  - tools/checks/rust_lifecycle_source_selfhost_blocked_recovery_diagnostic_guard.sh
---

# SOURCE-SELFHOST-BLOCKED-RECOVERY-DIAGNOSTIC-001

## Goal

Make the current source-selfhost stop line actionable without changing the
candidate pool. The existing evidence already proves that no next
family-specific `HakoAdopted` candidate can be selected. This row adds the
missing recovery line: why the pool is blocked, what must change before a
machine-derived owner can resume, and what remains forbidden.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Authority

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  next-hako-adoption-candidate-selection-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/
  variable-context-route-matrix-closeout-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/
  source-selfhost-adoption-plan-v0.json
lang/generated/rust_derived/hakorune_mir_builder/family_routes.json
tools/checks/rust_lifecycle_source_selfhost_blocked_recovery_diagnostic_guard.sh
```

## Required Delta

```text
preserve candidate_pool_state = Blocked
add stable recovery_reason_token
add stable next_action for the blocked state
name the minimum resume condition before any family-specific HakoAdopted row
keep manual family selection forbidden
```

## Acceptance

```text
candidate_pool_state = Blocked
eligible_candidate_count = 0
blocked_reason_token = NoEligibleDerivedMainlineRouteCandidate
parked_family = hakorune_mir_builder::variable_context
parked_reason = ReturnedReadBorrow
replacement_policy = OwnedReadSnapshotProjection
next_action = DesignConsultationRequired
resume_condition = MachineDerivedRepairLaneOrNewEligibleRoute
manual_family_selection = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
new HakoAdopted candidate = 0
VariableContext repair lane = 0
VariableContext HakoAdopted decision = 0
Source Selfhost = 0
Rust deletion = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-source-selfhost-blocked-recovery-diagnostic-v0
candidate_pool_state=Blocked
eligible_candidate_count=0
blocked_reason_token=NoEligibleDerivedMainlineRouteCandidate
parked_family=hakorune_mir_builder::variable_context
parked_reason=ReturnedReadBorrow
replacement_policy=OwnedReadSnapshotProjection
next_action=DesignConsultationRequired
resume_condition=MachineDerivedRepairLaneOrNewEligibleRoute
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
new_python_semantic_projector=0
source_selfhost_claim=0
summary=ok
```
