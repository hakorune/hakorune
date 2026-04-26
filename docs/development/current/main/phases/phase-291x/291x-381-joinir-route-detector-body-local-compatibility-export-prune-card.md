---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector body-local compatibility export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-380-joinir-route-detector-body-local-support-family-migration-card.md
---

# 291x-381: JoinIR Route Detector Body-Local Compatibility Export Prune

## Goal

Prune the final parent body-local compatibility exports after callers moved to
semantic support paths.

This is BoxShape-only. Do not remove the owning legacy modules.

## Change

Removed parent exports:

```text
loop_body_carrier_promoter
loop_body_cond_promoter
```

Body-local support remains reachable through:

```text
loop_route_detection::support::body_local::carrier
loop_route_detection::support::body_local::condition
```

## Preserved Behavior

- No body-local promoter logic changed.
- No legacy support module was deleted.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Boundary Improvement

The parent `loop_route_detection` module no longer re-exports legacy-named
support modules directly.

Its public surface is now the classifier API plus semantic `support` facades.

## Next Cleanup

Close out the route detector compatibility module ownership series by reviewing
the final parent surface and support facade docs.

## Non-Goals

- No physical file move.
- No support facade removal.
- No legacy support module deletion.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
