---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector condition-scope compatibility export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-378-joinir-route-detector-condition-scope-support-family-migration-card.md
---

# 291x-379: JoinIR Route Detector Condition-Scope Compatibility Export Prune

## Goal

Prune the parent `loop_condition_scope` compatibility export after callers
moved to `support::condition_scope` or owner-local legacy paths.

This is BoxShape-only. Do not remove the owning legacy module.

## Change

Removed parent export:

```text
loop_condition_scope
```

Condition-scope support remains reachable through:

```text
loop_route_detection::support::condition_scope
```

## Preserved Behavior

- No condition-scope logic changed.
- No legacy support module was deleted.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Next Cleanup

Migrate remaining body-local promoter compatibility exports:

```text
loop_body_carrier_promoter -> support::body_local::carrier
loop_body_cond_promoter -> support::body_local::condition
```

## Non-Goals

- No body-local migration in this card.
- No physical file move.
- No support facade removal.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
