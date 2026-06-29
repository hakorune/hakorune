---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Freeze the consultation-gated wider route-selection basis and the
  current runner vocabulary without selecting a family by hand.
Related:
  - docs/development/current/main/phases/phase-296x/1799-SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001.md
  - docs/development/current/main/phases/phase-296x/1800-SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001.md
  - docs/development/current/main/design/source-selfhost-wider-route-selection-basis-ssot.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-basis-v0.json
  - tools/checks/rust_lifecycle_source_selfhost_wider_route_selection_basis_guard.sh
---

# SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001

## Goal

Fix the current wider route-selection basis so the Source Selfhost lane stays
stopped until a consultation-gated basis or a machine-derived repair exists.
Also fix the runner vocabulary so VM/interpreter/EXE/AOT are validation or
reference lanes only.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Resolution

```text
output_contract:
  rust-lifecycle-source-selfhost-wider-route-selection-basis-v0

current_blocker_preserved:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

basis_kind:
  KeepSourceSelfhostStopped

reason_token:
  NoEligibleNativeAdoptionCandidate

next_action:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Runner Contract

```text
single_hako_meaning_source = 1
runner_semantic_owner = 0
exe_aot_gate_is_semantic_owner = 0
vm_hako_co_mainline_claim = 0
future_interpreter_required_for_projector_migration = 0
manual_family_selection = 0
```

The only allowed semantics owners are native `.hako` after adoption, Hako
projectors during HakoMainline stages, or Rust while that family is still
primary/oracle.

## Basis Rules

```text
if route matrix has a concrete inconsistency:
  basis = MachineDerivedRouteRepair

elif exactly one family is candidate-eligible:
  basis = ConsultationGatedWiderRouteSelection

else:
  basis = KeepSourceSelfhostStopped
```

Do not promote support-lane projectors into adoption candidates. Do not use
coverage or bundle size as proof.

## Non-Claims

```text
Source Selfhost = 0
Rust deletion = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
new Python SemanticProjector = 0
future interpreter activation = 0
```

## Closeout

```text
output_contract=rust-lifecycle-source-selfhost-wider-route-selection-basis-v0
current_blocker_preserved=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
basis_kind=KeepSourceSelfhostStopped
reason_token=NoEligibleNativeAdoptionCandidate
next_action=SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
single_hako_meaning_source=1
runner_semantic_owner=0
future_interpreter_required_for_projector_migration=0
manual_family_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
summary=ok
```
