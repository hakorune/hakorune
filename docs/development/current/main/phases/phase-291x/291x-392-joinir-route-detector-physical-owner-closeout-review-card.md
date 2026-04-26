---
Status: Landed
Date: 2026-04-27
Scope: JoinIR route detector physical owner closeout review
Related:
  - src/mir/loop_route_detection/
  - tools/checks/route_detector_legacy_surface_guard.sh
  - docs/development/current/main/REFACTORING_INDEX.md
  - docs/development/current/main/design/normalized-dev-removal-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-391-joinir-route-detector-body-local-physical-owner-closeout-card.md
---

# 291x-392: JoinIR Route Detector Physical Owner Closeout Review

## Goal

Confirm that the route detector physical owner migration is closed and that
live docs no longer point developers at the deleted `legacy/` storage.

This is a BoxShape closeout review. It does not change route classification
behavior.

## Review Result

Code state:

```text
src/mir/loop_route_detection/
  classify.rs
  features.rs
  kind.rs
  mod.rs
  support/
```

The `legacy/` storage directory is gone. The no-regrowth guard rejects:

```text
mod legacy;
loop_route_detection::legacy
src/mir/loop_route_detection/legacy/* files
old route detector compatibility module paths
```

Live docs updated:

```text
docs/development/current/main/REFACTORING_INDEX.md
docs/development/current/main/design/normalized-dev-removal-ssot.md
```

Historical card / investigation / archive mentions were intentionally left
unchanged.

## Preserved Behavior

- Route selection remains `LoopFeatures -> classify() -> LoopRouteKind`.
- Stable support callers remain under:

```text
loop_route_detection::support::...
```

- No route families or accepted shapes changed.

## Next Cleanup

Open the next compiler-cleanliness seam inventory before editing more code.
Do not reopen route detector support migration unless the owner path itself
changes.

## Validation

```bash
bash tools/checks/route_detector_legacy_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check --bin hakorune
git diff --check
```
