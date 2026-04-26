---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy route function export inventory
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/join_ir/lowering/loop_routes/simple_while.rs
  - src/mir/join_ir/lowering/loop_routes/with_break.rs
  - src/mir/join_ir/lowering/loop_routes/with_continue.rs
  - src/mir/join_ir/lowering/loop_routes/with_if_phi.rs
---

# 291x-360: JoinIR Route Detector Legacy Route Function Export Inventory

## Goal

Inventory parent-module exports for legacy route-shape detector functions
before pruning them from `crate::mir::loop_route_detection`.

This is BoxShape-only. Do not remove exports in this card.

## Findings

The parent module still exports:

```text
is_loop_simple_while_route
is_loop_break_route
is_if_phi_join_route
is_loop_continue_only_route
```

Repository search found no active production callers through the parent module.

Remaining references are old `rust,ignore` example comments in route lowerer
stubs:

```text
src/mir/join_ir/lowering/loop_routes/simple_while.rs
src/mir/join_ir/lowering/loop_routes/with_break.rs
src/mir/join_ir/lowering/loop_routes/with_continue.rs
src/mir/join_ir/lowering/loop_routes/with_if_phi.rs
```

The route-selection authority now lives in the structure classifier surface:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

The legacy functions remain defined under `legacy/`, but exposing them from the
parent module makes the old route-shape entry look current.

## Decision

Prune parent-level exports for those legacy route-shape functions.

Update or remove stale `rust,ignore` examples in the same prune card so docs do
not point at removed parent exports.

## Next Cleanup

Remove the parent export group:

```text
is_if_phi_join_route
is_loop_break_route
is_loop_continue_only_route
is_loop_simple_while_route
```

and update the stale route lowerer examples.

## Non-Goals

- No deletion of the legacy functions themselves.
- No route classifier behavior change.
- No route lowerer behavior change.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
