---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M33 atomic memory-order args docs/route vocabulary lock
---

# 293x-085 M33 Atomic Memory-Order Args Vocab Lock

## Decision

`M33 atomic memory-order args docs/route vocabulary lock` is live-docs.

M33 reserves the ordered fixed-slot i64 atomic operation vocabulary that later
pointer atomic rows will use. It does not activate the methods, MIR route rows,
NyRT exports, or `.inc` lowering.

Reserved `.hako` facade shape:

```text
AtomicCoreBox.load_i64_ordered(slot, order)
AtomicCoreBox.store_i64_ordered(slot, value, order)
AtomicCoreBox.fetch_add_i64_ordered(slot, delta, order)
AtomicCoreBox.cas_i64_ordered(slot, expected, desired, success_order, failure_order)
```

Reserved extern route vocabulary:

```text
extern.hako_atomic.slot_load_i64_ordered
  core_op = HakoAtomicSlotLoadI64Ordered
  symbol = hako_atomic_slot_load_i64_ordered
  arity = 2
  return_shape = scalar_i64
  value_demand = runtime_i64

extern.hako_atomic.slot_store_i64_ordered
  core_op = HakoAtomicSlotStoreI64Ordered
  symbol = hako_atomic_slot_store_i64_ordered
  arity = 3
  return_shape = scalar_i64
  value_demand = runtime_i64

extern.hako_atomic.slot_fetch_add_i64_ordered
  core_op = HakoAtomicSlotFetchAddI64Ordered
  symbol = hako_atomic_slot_fetch_add_i64_ordered
  arity = 3
  return_shape = scalar_i64
  value_demand = runtime_i64

extern.hako_atomic.slot_cas_i64_ordered
  core_op = HakoAtomicSlotCasI64Ordered
  symbol = hako_atomic_slot_cas_i64_ordered
  arity = 5
  return_shape = scalar_i64
  value_demand = runtime_i64
```

## Order Argument Contract

- `order` must be one of the existing `AtomicCoreBox.order_*_i64()` values.
- `load_i64_ordered`, `store_i64_ordered`, and `fetch_add_i64_ordered` each
  take exactly one order argument.
- `cas_i64_ordered` takes `success_order` and `failure_order`.
- CAS `success_order` may use any current order value.
- CAS `failure_order` may use only `Relaxed`, `Acquire`, or `SeqCst`.
- Invalid order values must fail-fast before backend lowering.
- Future verifier rows own exact invalid-combination diagnostics.

## Owned

- Route vocabulary names and arities.
- Public docs for the future ordered fixed-slot i64 operations.
- Guard:
  `tools/checks/k2_wide_atomic_memory_order_args_vocab_guard.sh`

## Not Owned

- Adding methods to `AtomicCoreBox`.
- Adding MIR extern route rows.
- Adding `.inc` declaration/need/emit rows.
- Adding NyRT exports.
- Pointer atomics.
- Production allocator remote-free policy.
- LLVM native pointer attrs or noalias/nonnull widening.

## Gate

```bash
bash tools/checks/k2_wide_atomic_memory_order_args_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must verify:

- docs contain the reserved names, route ids, core ops, and symbols.
- taskboard status moves M33 to `live-docs` and M34 to `next-card`.
- active source and `.inc` do not contain ordered atomic implementation rows.

## Result

Result on 2026-05-10:
`k2_wide_atomic_memory_order_args_vocab_guard.sh` passes.
