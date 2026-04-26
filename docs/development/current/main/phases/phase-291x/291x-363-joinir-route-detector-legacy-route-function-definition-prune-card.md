---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy route function definition prune
Related:
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-362-joinir-route-detector-legacy-route-function-definition-inventory-card.md
---

# 291x-363: JoinIR Route Detector Legacy Route Function Definition Prune

## Goal

Delete unused legacy route-shape function definitions from
`src/mir/loop_route_detection/legacy/mod.rs`.

This is BoxShape-only. Keep legacy support submodules intact.

## Change

Deleted:

```text
legacy::is_loop_simple_while_route
legacy::is_loop_break_route
legacy::is_if_phi_join_route
legacy::is_loop_continue_only_route
```

Updated the module header to state that current route selection is owned by:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

## Preserved Behavior

- No active caller was removed.
- No route classifier behavior changed.
- No route lowerer behavior changed.
- No legacy support submodule was deleted.

## Boundary Improvement

`legacy/mod.rs` is now a support-module hub, not an alternate route-shape
classification API.

## Next Cleanup

Inventory convenience re-exports that remain inside `legacy/mod.rs`:

```text
TrimLoopHelper
DigitPosDetectionResult
DigitPosDetector
TrimDetectionResult
TrimDetector
```

Do not prune them without checking legacy-internal callers.

## Non-Goals

- No deletion of detector/promoter submodules.
- No cleanup of convenience re-exports in this card.
- No route classifier API change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
