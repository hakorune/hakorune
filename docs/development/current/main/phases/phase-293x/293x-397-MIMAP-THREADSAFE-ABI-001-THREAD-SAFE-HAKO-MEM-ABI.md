# 293x-397 MIMAP-THREADSAFE-ABI-001 Thread-Safe hako_mem ABI

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-THREADSAFE-ABI-001` is the next allocator substrate row after
`MIMAP-REMOTE-001`. It should pin the thread-safe `hako_mem` ABI contract and
smoke boundary needed by allocator-style code without activating process
allocator replacement.

This row is ABI/substrate contract work. It must not install provider hooks,
replace host malloc/free, add `#[global_allocator]`, or imply that the default
process allocator path changes.

## Scope

- Document and guard `hako_mem_alloc`, `hako_mem_free`, and
  `hako_mem_realloc` as thread-safe runtime ABI leaves.
- Confirm existing runtime-decl / extern route ownership remains the acceptance
  path for `hako_mem` calls.
- Keep smoke tests deterministic and allocator-internal; true parallel stress
  remains parked for `MIMAP-PAR-STRESS-001`.

## Stop Lines

- No host allocator replacement, provider activation, hook installation, or
  `#[global_allocator]`.
- No broad true-parallel runtime semantics.
- No new source-level concurrency syntax.
- No backend `.inc` matcher shortcut.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
bash tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
