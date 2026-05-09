---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M30 mimalloc atomic fetch-add slot EXE proof
---

# 293x-082 M30 Mimalloc Atomic Fetch-Add Slot EXE Proof

## Decision

`M30 mimalloc atomic fetch-add slot EXE proof` is live-narrow.

The row adds one allocator-shaped `hako.atomic` primitive for pure-first EXE:
a fixed runtime-owned i64 atomic slot fetch-add. This is not a generic atomic
API and does not introduce pointer atomics or memory-order arguments.

Accepted shape:

```text
AtomicCoreBox.fetch_add_i64(slot, delta)
  -> externcall "hako_atomic_slot_fetch_add_i64"(slot, delta)
```

The runtime operation uses the current narrow default order owned by the
runtime export and returns the previous slot value.

## Owned

- `apps/mimalloc-atomic-fetch-add-proof/`
- `AtomicCoreBox.fetch_add_i64/2`
- MIR extern route row for `hako_atomic_slot_fetch_add_i64/2`
- pure-first `.inc` declaration/need/emit row for that route id.
- NyRT export for the symbol.
- Guard:
  `tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh`

## Not Owned

- Pointer atomics.
- Memory-order arguments on fetch-add.
- Remote-free list policy.
- TLS coupling.
- Native pointer strong attrs.
- Backend-local helper-name inference.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh
```

The guard must verify:

- MIR JSON publishes the `extern_call_routes` row.
- `lowering_plan` carries the same route id/symbol/arity.
- pure-first build logs hit only the route-fact emit consumer.
- the EXE proves previous-value return and accumulated value through the
  already-live load/store rows.
- `.inc` does not branch on the fixture app name.

## Result

Result on 2026-05-10: `k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh` passes.

## Follow-Up

Future rows may add memory-order arguments and pointer atomics. Those must
remain separate BoxCount cards.
