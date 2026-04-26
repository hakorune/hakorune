---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector parent module doc closeout
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-368-joinir-route-detector-legacy-module-visibility-prune-card.md
---

# 291x-369: JoinIR Route Detector Parent Module Doc Closeout

## Goal

Align parent module docs with the private `legacy` storage boundary.

This is docs/comment-only BoxShape cleanup.

## Change

Clarified:

```text
legacy/ is private implementation storage
selected modules are parent compatibility exports
current route selection uses classify / LoopFeatures / LoopRouteKind
legacy route-shape function entry points are not current route API
```

## Preserved Behavior

- No code behavior changed.
- No visibility changed.
- No route classifier behavior changed.
- No route lowerer behavior changed.

## Current Boundary

The route detector parent surface is now:

```text
classify
LoopFeatures
LoopRouteKind
selected support-module compatibility exports
```

The direct `legacy` module path is private.

## Next Cleanup

Inventory the remaining selected support-module compatibility exports and decide
which ones should move to stable owner modules versus remain parent
compatibility modules.

## Non-Goals

- No compatibility export deletion.
- No caller migration.
- No support module move.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
