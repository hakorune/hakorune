---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector trim compatibility export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-374-joinir-route-detector-trim-support-family-migration-card.md
---

# 291x-375: JoinIR Route Detector Trim Compatibility Export Prune

## Goal

Prune the parent `trim_loop_helper` compatibility export after callers moved to
`support::trim`.

This is BoxShape-only. Do not remove the owning legacy module.

## Change

Removed parent export:

```text
trim_loop_helper
```

Trim support remains reachable through:

```text
loop_route_detection::support::trim
```

## Preserved Behavior

- No Trim helper logic changed.
- No legacy support module was deleted.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Next Cleanup

Migrate the `function_scope_capture` family to:

```text
loop_route_detection::support::function_scope
```

Keep parent `function_scope_capture` compatibility export until source callers
are gone.

## Non-Goals

- No function-scope migration in this card.
- No physical file move.
- No support facade removal.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
