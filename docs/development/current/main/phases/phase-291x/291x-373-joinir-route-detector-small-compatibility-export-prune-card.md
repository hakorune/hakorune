---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector small compatibility export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-372-joinir-route-detector-small-support-family-migration-card.md
---

# 291x-373: JoinIR Route Detector Small Compatibility Export Prune

## Goal

Prune parent compatibility exports whose source callers were migrated to
semantic `support` owner paths.

This is BoxShape-only. Do not remove legacy support modules.

## Change

Removed parent exports:

```text
break_condition_analyzer
pinned_local_analyzer
mutable_accumulator_analyzer
```

These are now reachable through stable support paths:

```text
loop_route_detection::support::break_condition
loop_route_detection::support::locals::pinned
loop_route_detection::support::locals::mutable_accumulator
```

## Preserved Behavior

- No support helper logic changed.
- No legacy support module was deleted.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Next Cleanup

Migrate the `trim_loop_helper` family to:

```text
loop_route_detection::support::trim
```

Keep the parent `trim_loop_helper` compatibility export until source callers
are gone.

## Non-Goals

- No migration for trim/function-scope/condition-scope/body-local families.
- No physical file move.
- No support facade removal.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
