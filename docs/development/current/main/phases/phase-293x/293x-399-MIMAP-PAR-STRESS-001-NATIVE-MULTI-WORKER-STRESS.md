# 293x-399 MIMAP-PAR-STRESS-001 Native Multi-Worker Stress

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-PAR-STRESS-001` is the native multi-worker substrate stress row after
`MIMAP-THREADSAFE-ABI-001`.

It should add a native stress guard that exercises the allocator-facing
substrate under real OS threads without opening user-facing concurrency
language semantics.

This row is substrate smoke, not allocator-provider activation. It must not
install provider hooks, replace host malloc/free, add `#[global_allocator]`, or
make mimalloc the default process allocator.

## Scope

- Add a deterministic native multi-worker stress fixture for distinct
  `hako_mem` allocations, worker identity, TLS cache slots, fixed-slot atomics,
  and remote-free policy where practical.
- Keep VM as proof/reference only. True parallel behavior belongs to native
  Rust/kernel or LLVM/EXE smoke.
- Preserve the route boundary: backend lowering must consume MIR-owned route
  facts or runtime-decl rows, not raw helper-name classifiers.

## Stop Lines

- No source-level `worker_local`, `lock<T>`, `Channel`, `task_scope`, or
  broad true-parallel language semantics.
- No provider activation, hook installation, process allocator replacement, or
  `#[global_allocator]`.
- No page ownership mutation beyond the explicit stress fixture contract.
- No backend `.inc` matcher shortcut.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_parallel_substrate_stress_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Added `crates/nyash_kernel/src/tests/mimalloc_parallel_substrate.rs` as the
  native Rust/kernel stress fixture.
- The fixture spawns real OS threads and composes:
  - `hako_mem_alloc` / `hako_mem_free` for distinct node ownership;
  - `hako_worker_current_id_i64` for the current single-worker proof seam;
  - `hako_tls_cache_slot_set_i64` / `hako_tls_cache_slot_get_i64` for
    per-thread cache-slot isolation;
  - `hako_atomic_slot_fetch_add_i64` for deterministic fixed-slot counting;
  - `hako_atomic_ptr_load_ordered` / `hako_atomic_ptr_cas_ordered` for a
    pointer-CAS remote-free stack model.
- Added `tools/checks/k2_wide_mimalloc_parallel_substrate_stress_guard.sh` to
  keep the row narrow and prevent source-level concurrency or allocator-provider
  activation from leaking into the fixture.

## Evidence

```text
cargo test -q -p nyash_kernel mimalloc_parallel_substrate_stress
bash tools/checks/k2_wide_mimalloc_parallel_substrate_stress_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
