---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector export surface closeout review
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-353-joinir-route-detector-legacy-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-365-joinir-route-detector-legacy-convenience-reexport-prune-card.md
---

# 291x-366: JoinIR Route Detector Export Surface Closeout Review

## Goal

Review the route detector export surface after the 291x-353 through 291x-365
cleanup sequence.

This is BoxShape-only. Do not change code in this card.

## Closed In This Sequence

Removed from the parent `loop_route_detection` surface:

```text
pub use legacy::*;
direct legacy type exports
legacy-internal-only module exports
legacy route-shape function exports
```

Removed from `legacy/mod.rs`:

```text
unused legacy route-shape function definitions
unused convenience type re-exports
```

Stale route lowerer examples now point at the current route-selection authority:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

## Current Parent Surface

The parent module now exposes:

```text
classify
LoopFeatures
LoopRouteKind
legacy support modules still used by external callers
```

The retained legacy support module exports are:

```text
break_condition_analyzer
function_scope_capture
loop_body_carrier_promoter
loop_body_cond_promoter
loop_condition_scope
mutable_accumulator_analyzer
pinned_local_analyzer
trim_loop_helper
```

## Remaining Seam

`src/mir/loop_route_detection/mod.rs` still declares:

```text
pub mod legacy;
```

That means direct `crate::mir::loop_route_detection::legacy::...` access remains
possible even though current callers use the parent compatibility exports or
owner module paths.

## Decision

Treat route detector export cleanup as closed for the parent compatibility
surface.

Open the next seam as a separate visibility inventory:

```text
Can `pub mod legacy` become private `mod legacy` while selected compatibility
exports stay public?
```

## Non-Goals

- No code change in this card.
- No visibility change for `legacy`.
- No route classifier behavior change.

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
