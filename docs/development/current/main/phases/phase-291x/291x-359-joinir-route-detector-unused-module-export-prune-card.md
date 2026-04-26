---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector unused legacy module export prune
Related:
  - src/mir/loop_route_detection/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-357-joinir-route-detector-unused-module-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-358-joinir-route-detector-legacy-internal-import-owner-path-card.md
---

# 291x-359: JoinIR Route Detector Unused Module Export Prune

## Goal

Prune parent-module exports for legacy-internal-only modules after owner-path
internal imports were migrated.

This is BoxShape-only. Keep externally used module exports stable.

## Change

Removed these parent-module exports from `src/mir/loop_route_detection/mod.rs`:

```text
condition_var_analyzer
loop_body_digitpos_promoter
digitpos_detector
trim_detector
```

Kept externally used module exports:

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

## Preserved Behavior

- No legacy module was deleted.
- No detector logic changed.
- No route classifier behavior changed.
- Existing external module-path callers remain supported.

## Boundary Improvement

The parent `loop_route_detection` module no longer exposes legacy-internal
helper modules that are only owned inside `legacy/`.

## Next Cleanup

Inventory legacy route-shape function exports:

```text
is_loop_simple_while_route
is_loop_break_route
is_if_phi_join_route
is_loop_continue_only_route
```

Do not remove the functions in the inventory card.

## Non-Goals

- No legacy function export pruning in this card.
- No legacy module deletion.
- No caller migration.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
