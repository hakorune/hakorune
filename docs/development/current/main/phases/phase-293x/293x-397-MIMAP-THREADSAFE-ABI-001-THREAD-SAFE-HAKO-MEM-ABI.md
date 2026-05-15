# 293x-397 MIMAP-THREADSAFE-ABI-001 Thread-Safe hako_mem ABI

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-THREADSAFE-ABI-001` is the next allocator substrate row after
`MIMAP-REMOTE-001`. It should pin the thread-safe `hako_mem` ABI contract and
smoke boundary needed by allocator-style code without activating process
allocator replacement.

Thread-safe means calls are allowed concurrently for distinct allocation
ownership. Concurrent free/realloc/use of the same pointer is outside this ABI
contract.

This row is ABI/substrate contract work. It must not install provider hooks,
replace host malloc/free, add `#[global_allocator]`, or imply that the default
process allocator path changes.

## Scope

- Document and guard `hako_mem_alloc`, `hako_mem_free`, and
  `hako_mem_realloc` as thread-safe runtime ABI leaves for distinct
  allocations.
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
bash tools/checks/k2_wide_hako_mem_threadsafe_abi_guard.sh
bash tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
bash tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Added `hako_mem_realloc` to the Rust kernel `hako.mem` export set so the
  runtime-decl ABI row has a native link target alongside alloc/free.
- Added a Rust concurrent distinct-allocation smoke for
  alloc/realloc/free. The test intentionally does not bless concurrent
  mutation/free/realloc of the same pointer.
- Documented the thread-safe ABI contract in runtime substrate, ABI, return
  proof vocabulary, and C shim docs.
- Added `k2_wide_hako_mem_threadsafe_abi_guard.sh` as the consolidated
  MIMAP-THREADSAFE-ABI-001 guard. It runs the existing runtime-decl and
  pure-first hako_mem guards and checks the thread-safety stop lines.

## Evidence

```text
bash tools/checks/k2_wide_hako_mem_threadsafe_abi_guard.sh
bash tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
bash tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
