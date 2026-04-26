---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector function-scope compatibility export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-376-joinir-route-detector-function-scope-support-family-migration-card.md
---

# 291x-377: JoinIR Route Detector Function-Scope Compatibility Export Prune

## Goal

Prune the parent `function_scope_capture` compatibility export after callers
moved to `support::function_scope`.

This is BoxShape-only. Do not remove the owning legacy module.

## Change

Removed parent export:

```text
function_scope_capture
```

Function-scope capture remains reachable through:

```text
loop_route_detection::support::function_scope
```

## Preserved Behavior

- No capture analysis logic changed.
- No legacy support module was deleted.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Next Cleanup

Migrate the `loop_condition_scope` family to:

```text
loop_route_detection::support::condition_scope
```

Keep parent `loop_condition_scope` compatibility export until source callers are
gone.

## Non-Goals

- No condition-scope migration in this card.
- No physical file move.
- No support facade removal.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
