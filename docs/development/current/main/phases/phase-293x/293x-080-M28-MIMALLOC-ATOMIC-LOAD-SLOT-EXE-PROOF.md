---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M28 mimalloc atomic load slot EXE proof
---

# 293x-080 M28 Mimalloc Atomic Load Slot EXE Proof

## Decision

`M28 mimalloc atomic load slot EXE proof` is live-narrow.

The row adds one allocator-shaped `hako.atomic` primitive for pure-first EXE:
a fixed runtime-owned i64 atomic slot load. This is not a generic atomic API and
does not introduce pointer atomics or memory-order arguments.

Accepted shape:

```text
AtomicCoreBox.load_i64(slot)
  -> externcall "hako_atomic_slot_load_i64"(slot)
```

The runtime operation uses the current narrow default order owned by the
runtime export. Memory-order argument threading remains a future split.

## Owned

- `apps/mimalloc-atomic-load-proof/`
- `AtomicCoreBox.load_i64/1`
- MIR extern route row for `hako_atomic_slot_load_i64/1`
- pure-first `.inc` declaration/need/emit row for that route id.
- NyRT export for the symbol.
- Guard:
  `tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh`

## Not Owned

- Atomic store/fetch_add.
- Pointer atomics.
- Memory-order arguments on load.
- Remote-free list policy.
- TLS coupling.
- Native pointer strong attrs.
- Backend-local helper-name inference.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh
```

The guard must verify:

- MIR JSON publishes the `extern_call_routes` row.
- `lowering_plan` carries the same route id/symbol/arity.
- pure-first build logs hit only the route-fact emit consumer.
- the EXE proves a CAS-written value is visible through the load row.
- `.inc` does not branch on the fixture app name.

## Result

Result on 2026-05-10: `k2_wide_mimalloc_atomic_load_exe_guard.sh` passes.

## Follow-Up

Future rows may add `store`, `fetch_add`, and memory-order arguments. Those
must remain separate BoxCount cards.
