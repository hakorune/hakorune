---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector legacy route function definition inventory
Related:
  - src/mir/loop_route_detection/legacy/mod.rs
  - src/mir/join_ir/lowering/simple_while.rs
  - docs/development/current/main/phases/phase-291x/291x-361-joinir-route-detector-legacy-route-function-export-prune-card.md
---

# 291x-362: JoinIR Route Detector Legacy Route Function Definition Inventory

## Goal

Inventory the legacy route-shape function definitions that remain in
`src/mir/loop_route_detection/legacy/mod.rs` after parent exports were pruned.

This is BoxShape-only. Do not delete definitions in this card.

## Findings

The remaining target definitions are:

```text
legacy::is_loop_simple_while_route
legacy::is_loop_break_route
legacy::is_if_phi_join_route
legacy::is_loop_continue_only_route
```

Repository search found no active caller outside the definitions and their own
legacy doc comments.

There is a different helper with the same name in:

```text
src/mir/join_ir/lowering/simple_while.rs
```

That helper takes `LoopScopeShape`, not `LoopForm`, and is outside this prune
target.

## Decision

The four `legacy/mod.rs` route-shape functions can be deleted.

They no longer own route selection; current route classification is:

```text
LoopFeatures -> classify() -> LoopRouteKind
```

## Next Cleanup

Delete the four route-shape function definitions and their surrounding stale
section comments from `src/mir/loop_route_detection/legacy/mod.rs`.

Keep legacy submodules intact.

## Non-Goals

- No deletion of the `src/mir/join_ir/lowering/simple_while.rs` helper.
- No route classifier behavior change.
- No route lowerer behavior change.
- No legacy submodule deletion.

## Validation

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
