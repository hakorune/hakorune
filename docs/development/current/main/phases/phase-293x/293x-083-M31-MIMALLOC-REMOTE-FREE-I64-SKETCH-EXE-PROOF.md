---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M31 mimalloc remote-free i64 sketch EXE proof
---

# 293x-083 M31 Mimalloc Remote-Free I64 Sketch EXE Proof

## Decision

`M31 mimalloc remote-free i64 sketch EXE proof` is live-narrow.

The row composes the already-live `hako.atomic` fixed-slot i64 primitives into a
minimal remote-free push sketch under pure-first EXE. It does not add a new
runtime helper or a new `.inc` route row.

Accepted shape:

```text
remote_free_i64(head_slot, next_slot, block_id)
  old = AtomicCoreBox.load_i64(head_slot)
  AtomicCoreBox.store_i64(next_slot, old)
  prev = AtomicCoreBox.cas_i64(head_slot, old, block_id)
  if prev == old:
    AtomicCoreBox.fetch_add_i64(count_slot, 1)
```

This proves that the allocator-shaped remote-free pattern can be expressed as
ordinary `.hako` composition over MIR-owned route facts. Pointer atomics,
memory-order arguments, and real block pointers remain future rows.

## Owned

- `apps/mimalloc-remote-free-i64-proof/`
- Guard:
  `tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh`
- Docs/index wiring for the M31 composition proof.

## Not Owned

- New `hako.atomic` extern route rows.
- New NyRT exports.
- Pointer atomics or native pointer list nodes.
- Memory-order arguments.
- A production remote-free allocator policy.
- TLS coupling beyond the already-live independent cache-slot row.
- Backend-local helper-name inference or app-specific `.inc` matching.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh
```

The guard must verify:

- MIR JSON still publishes the existing CAS/load/store/fetch_add route facts.
- The remote-free sketch reaches pure-first EXE without a new backend route.
- The EXE proves LIFO head update, next-link storage, and enqueue count.
- `.inc` does not branch on the fixture app name.

## Result

Result on 2026-05-10: `k2_wide_mimalloc_remote_free_i64_exe_guard.sh` passes.

## Follow-Up

Future rows may add pointer atomics, memory-order arguments, or a real
allocator remote-free policy. Those must remain separate cards and must not be
hidden in this i64 sketch fixture.
