---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy export inventory
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
---

# 291x-353: JoinIR Route Detector Legacy Export Inventory

## Goal

Inventory the `loop_route_detection` legacy export surface before removing the
top-level wildcard re-export.

This is BoxShape-only. Do not change route behavior in this card.

## Findings

The current top-level module re-exports every public item from `legacy/`.

```text
src/mir/loop_route_detection/mod.rs
  pub use legacy::*;
```

That wildcard exposes:

```text
legacy route-shape functions:
  is_loop_simple_while_route
  is_loop_break_route
  is_if_phi_join_route
  is_loop_continue_only_route

legacy modules:
  condition_var_analyzer
  loop_condition_scope
  loop_body_carrier_promoter
  loop_body_cond_promoter
  loop_body_digitpos_promoter
  trim_loop_helper
  break_condition_analyzer
  function_scope_capture
  digitpos_detector
  trim_detector
  pinned_local_analyzer
  mutable_accumulator_analyzer

legacy convenience type re-exports:
  TrimLoopHelper
  DigitPosDetectionResult
  DigitPosDetector
  TrimDetectionResult
  TrimDetector
```

Live callers mostly use legacy modules via the top-level path:

```text
crate::mir::loop_route_detection::loop_condition_scope::...
crate::mir::loop_route_detection::function_scope_capture::...
crate::mir::loop_route_detection::trim_loop_helper::TrimLoopHelper
crate::mir::loop_route_detection::loop_body_cond_promoter::...
crate::mir::loop_route_detection::loop_body_carrier_promoter::...
crate::mir::loop_route_detection::break_condition_analyzer::...
crate::mir::loop_route_detection::pinned_local_analyzer::...
crate::mir::loop_route_detection::mutable_accumulator_analyzer::...
```

The legacy route-shape functions appear as documented examples and public
compatibility surface, not as active production decision calls in the searched
paths.

## Decision

The wildcard export is too broad. Replace it with an explicit compatibility
export list first.

This keeps current callers stable while making the route detector boundary
auditable.

## Next Cleanup

Replace:

```text
pub use legacy::*;
```

with explicit exports for the current compatibility surface:

```text
legacy modules used by callers
legacy route-shape functions
legacy convenience type re-exports
```

Do not migrate caller paths in the same card.

## Non-Goals

- No route behavior change.
- No legacy module deletion.
- No caller path migration.
- No route-shape function deletion.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
