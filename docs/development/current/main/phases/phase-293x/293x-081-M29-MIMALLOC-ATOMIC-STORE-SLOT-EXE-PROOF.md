---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M29 mimalloc atomic store slot EXE proof
---

# 293x-081 M29 Mimalloc Atomic Store Slot EXE Proof

## Decision

`M29 mimalloc atomic store slot EXE proof` is live-narrow.

The row adds one allocator-shaped `hako.atomic` primitive for pure-first EXE:
a fixed runtime-owned i64 atomic slot store. This is not a generic atomic API
and does not introduce pointer atomics or memory-order arguments.

Accepted shape:

```text
AtomicCoreBox.store_i64(slot, value)
  -> externcall "hako_atomic_slot_store_i64"(slot, value)
```

The runtime operation uses the current narrow default order owned by the
runtime export and returns `0` on success.

## Owned

- `apps/mimalloc-atomic-store-proof/`
- `AtomicCoreBox.store_i64/2`
- MIR extern route row for `hako_atomic_slot_store_i64/2`
- pure-first `.inc` declaration/need/emit row for that route id.
- NyRT export for the symbol.
- Guard:
  `tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh`

## Not Owned

- Atomic fetch_add.
- Pointer atomics.
- Memory-order arguments on store.
- Remote-free list policy.
- TLS coupling.
- Native pointer strong attrs.
- Backend-local helper-name inference.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh
```

The guard must verify:

- MIR JSON publishes the `extern_call_routes` row.
- `lowering_plan` carries the same route id/symbol/arity.
- pure-first build logs hit only the route-fact emit consumer.
- the EXE proves a stored value is visible through the already-live load row.
- `.inc` does not branch on the fixture app name.

## Result

Result on 2026-05-10: `k2_wide_mimalloc_atomic_store_exe_guard.sh` passes.

## Follow-Up

Future rows may add `fetch_add` and memory-order arguments. Those must remain
separate BoxCount cards.
