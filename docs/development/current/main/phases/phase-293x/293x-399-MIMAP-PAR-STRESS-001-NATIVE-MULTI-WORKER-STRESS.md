# 293x-399 MIMAP-PAR-STRESS-001 Native Multi-Worker Stress

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-PAR-STRESS-001` is the next allocator substrate row after
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
