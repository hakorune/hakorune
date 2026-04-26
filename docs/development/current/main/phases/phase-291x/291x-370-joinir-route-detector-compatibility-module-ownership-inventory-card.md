---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector compatibility module ownership inventory
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-369-joinir-route-detector-parent-module-doc-closeout-card.md
---

# 291x-370: JoinIR Route Detector Compatibility Module Ownership Inventory

## Goal

Inventory the remaining parent compatibility module exports and choose the
next stable owner boundary.

This is BoxShape-only. Do not migrate callers or change code in this card.

## Remaining Compatibility Exports

`src/mir/loop_route_detection/mod.rs` still re-exports:

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

Source caller counts:

```text
break_condition_analyzer      1
function_scope_capture       10
loop_body_carrier_promoter    2
loop_body_cond_promoter       2
loop_condition_scope         17
mutable_accumulator_analyzer  1
pinned_local_analyzer         1
trim_loop_helper              9
```

## Decision

Use a stable semantic facade before physical file moves:

```text
crate::mir::loop_route_detection::support::condition_scope
crate::mir::loop_route_detection::support::function_scope
crate::mir::loop_route_detection::support::trim
crate::mir::loop_route_detection::support::body_local::{carrier, condition}
crate::mir::loop_route_detection::support::break_condition
crate::mir::loop_route_detection::support::locals::{pinned, mutable_accumulator}
```

The facade should initially re-export public items from private `legacy/`
storage. This keeps physical movement separate from caller migration.

## Migration Order

1. Add `support` facade with semantic owner modules and no caller migration.
2. Migrate small families first:
   `break_condition`, `locals::{pinned, mutable_accumulator}`.
3. Migrate `trim` callers.
4. Migrate `function_scope` callers.
5. Migrate `condition_scope` and `body_local` callers.
6. Prune parent compatibility exports only after source callers are gone.

## Rationale

This keeps `legacy/` private implementation storage while giving callers a
stable non-legacy route-support owner path.

It also avoids a large physical move that would mix ownership cleanup with
file relocation.

## Non-Goals

- No code change in this card.
- No compatibility export deletion.
- No physical file move.
- No route classifier behavior change.

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
