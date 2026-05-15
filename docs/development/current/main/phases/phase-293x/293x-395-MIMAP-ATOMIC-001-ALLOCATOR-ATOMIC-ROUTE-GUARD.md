# 293x-395 MIMAP-ATOMIC-001 Allocator Atomic Route Guard

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-ATOMIC-001` is the next allocator substrate row after
`MIMAP-TLS-001`. It should consolidate the allocator-facing atomic route set
and guard the existing slot atomic load/store/CAS/fetch_add rows without
opening user-facing concurrency language semantics.

This remains runtime/internal allocator substrate. It must not open source-level
atomic syntax, `lock<T>`, task scheduling, Channel semantics, remote-free
policy, page ownership transfer, or provider activation.

## Scope

- Reuse existing `AtomicCoreBox` slot routes for CAS/load/store/fetch_add.
- Confirm MIR-owned `extern_call_routes` / `lowering_plan` rows are the only
  backend acceptance path for these atomic leaves.
- Keep pointer atomic rows and remote-free rows as separate owners unless the
  guard explicitly proves they are already part of the selected route set.

## Stop Lines

- No source-level atomic syntax or public concurrency API.
- No `lock<T>`, `worker_local`, Channel, `task_scope`, `nowait`/`await`, or
  true thread-pool semantics.
- No remote-free / abandoned-owner / page ownership transfer behavior.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend `.inc` matcher shortcut.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
