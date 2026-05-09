---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M34 pointer atomic vocabulary docs lock
---

# 293x-086 M34 Pointer Atomic Vocabulary Lock

## Decision

`M34 pointer atomic vocabulary docs lock` is live-docs.

M34 reserves the native-pointer atomic vocabulary needed before a real
remote-free list can move beyond the M31 fixed-slot i64 sketch. It does not
activate parser syntax, MIR route rows, NyRT exports, `.inc` lowering, or
allocator policy.

Reserved `.hako` facade shape:

```text
AtomicCoreBox.ptr_load_ordered(cell_ptr, order)
AtomicCoreBox.ptr_store_ordered(cell_ptr, value_ptr, order)
AtomicCoreBox.ptr_cas_ordered(cell_ptr, expected_ptr, desired_ptr, success_order, failure_order)
```

Reserved extern route vocabulary:

```text
extern.hako_atomic.ptr_load_ordered
  core_op = HakoAtomicPtrLoadOrdered
  symbol = hako_atomic_ptr_load_ordered
  arity = 2
  return_shape = native_ptr_nullable
  value_demand = native_ptr_nullable

extern.hako_atomic.ptr_store_ordered
  core_op = HakoAtomicPtrStoreOrdered
  symbol = hako_atomic_ptr_store_ordered
  arity = 3
  return_shape = scalar_i64
  value_demand = native_ptr_nullable

extern.hako_atomic.ptr_cas_ordered
  core_op = HakoAtomicPtrCasOrdered
  symbol = hako_atomic_ptr_cas_ordered
  arity = 5
  return_shape = native_ptr_nullable
  value_demand = native_ptr_nullable
```

## Pointer Boundary Contract

- `cell_ptr`, `value_ptr`, `expected_ptr`, and `desired_ptr` are native pointer
  transport values, not runtime handles.
- The route vocabulary must not attach LLVM `nonnull`, `dereferenceable`,
  `noalias`, or alignment attrs.
- Pointer atomics must validate memory-order values using the M33 contract.
- Pointer CAS uses separate success and failure orders. Failure order is
  restricted to Relaxed, Acquire, or SeqCst.
- Pointer `fetch_add` is not reserved by M34; allocator remote-free needs
  load/store/CAS, and pointer arithmetic atomic rows require separate evidence.

## Owned

- Pointer atomic facade names.
- Pointer atomic route ids, core ops, symbols, arities, and return classes.
- Guard:
  `tools/checks/k2_wide_pointer_atomic_vocab_guard.sh`

## Not Owned

- Adding methods to `AtomicCoreBox`.
- Adding MIR extern route rows.
- Adding `.inc` declaration/need/emit rows.
- Adding NyRT exports.
- Pointer dereference/load-store helpers.
- Production remote-free allocator policy.
- LLVM native pointer attrs or noalias/nonnull widening.

## Gate

```bash
bash tools/checks/k2_wide_pointer_atomic_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must verify:

- docs contain the reserved pointer atomic names, route ids, core ops, symbols,
  arities, and native pointer return classes.
- taskboard status moves M34 to `live-docs` and M35 to `next-card`.
- active source, NyRT, and `.inc` do not contain pointer atomic implementation
  rows.

## Result

Result on 2026-05-10:
`k2_wide_pointer_atomic_vocab_guard.sh` passes.
