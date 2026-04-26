---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy/support README boundary cleanup
Related:
  - src/mir/loop_route_detection/legacy/README.md
  - src/mir/loop_route_detection/support/README.md
  - docs/development/current/main/phases/phase-291x/291x-383-joinir-route-detector-support-facade-physical-owner-inventory-card.md
---

# 291x-384: JoinIR Route Detector Legacy/Support README Boundary

## Goal

Align local README files with the current private-legacy / public-support
boundary.

This is docs-only BoxShape cleanup.

## Change

Updated:

```text
src/mir/loop_route_detection/legacy/README.md
src/mir/loop_route_detection/support/README.md
```

Clarified:

```text
legacy/ = private implementation storage
support/ = stable caller paths
route selection = LoopFeatures -> classify() -> LoopRouteKind
```

## Preserved Behavior

- No code behavior changed.
- No module visibility changed.
- No physical files moved.

## Next Cleanup

Add a guard to prevent parent legacy-surface regrowth:

```text
pub mod legacy
pub use legacy::
loop_route_detection::<old compatibility module>
```

## Non-Goals

- No physical owner migration.
- No support facade split.
- No route classifier API change.

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
