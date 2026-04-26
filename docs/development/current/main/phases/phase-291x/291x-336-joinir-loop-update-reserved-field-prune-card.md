---
Status: Landed
Date: 2026-04-26
Scope: JoinIR loop-update reserved field prune
Related:
  - src/mir/builder/control_flow/plan/route_prep_pipeline.rs
  - docs/development/current/main/phases/phase-291x/291x-335-joinir-loop-update-caller-surface-inventory-card.md
---

# 291x-336: JoinIR Loop-update Reserved Field Prune

## Goal

Remove the never-read `RoutePrepContext.loop_update_summary` reserved field.

This is behavior-preserving BoxShape cleanup.

## Change

Removed:

```text
RoutePrepContext.loop_update_summary
LoopUpdateSummary import in route_prep_pipeline.rs
None initializers for the field
```

## Preserved Behavior

- `RoutePrepContext::is_if_phi_join_pattern(...)` still computes the summary
  locally from `loop_body`.
- `analyze_loop_updates_from_ast(...)` remains the public analyzer entrypoint.
- `LoopFeatures.update_summary` remains available for observed summary data.
- No route decision changed.

## Non-Goals

- No loop-update classification behavior change.
- No IfPhiJoin route change.
- No Case-A shape change.
- No public analyzer API change.

## Validation

```bash
cargo test -q route_prep
cargo test -q loop_update_
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
