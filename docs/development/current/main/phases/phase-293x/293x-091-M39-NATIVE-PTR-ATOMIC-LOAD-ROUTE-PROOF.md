---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M39 native pointer atomic load route proof
---

# 293x-091 M39 Native Ptr Atomic Load Route Proof

## Decision

`M39 native pointer atomic load route proof` is live-narrow.

M39 activates exactly one additional M34 pointer-atomic vocabulary row:
`hako_atomic_ptr_load_ordered(cell_ptr, order)`.

Accepted shape:

```text
externcall "hako_atomic_ptr_load_ordered"(cell_ptr, order)
  route_id = extern.hako_atomic.ptr_load_ordered
  core_op = HakoAtomicPtrLoadOrdered
  symbol = hako_atomic_ptr_load_ordered
  arity = 2
  return_shape = native_ptr_nullable
  value_demand = native_ptr_nullable
```

The pure-first backend converts the cell i64 transport value to LLVM `ptr`,
passes the order as i64, calls the runtime helper returning `ptr`, and converts
the result back to i64 transport. No native pointer attrs are emitted.

## Owned

- `apps/mimalloc-ptr-atomic-load-proof/`
- MIR extern route row for `hako_atomic_ptr_load_ordered/2`.
- pure-first `.inc` declaration/need/emit row for that route id.
- NyRT export for the symbol.
- Guard:
  `tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh`
- Guard contract sync: M35-M38/M34 guards now keep pointer CAS/fetch_add
  inactive while allowing the M39 load row.

## Not Owned

- `ptr_cas_ordered`.
- pointer `fetch_add`.
- AtomicCoreBox pointer methods.
- Production remote-free allocator policy.
- LLVM native pointer attrs or noalias/nonnull widening.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh
bash tools/checks/k2_wide_pointer_atomic_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must verify:

- MIR JSON publishes the pointer load extern route fact.
- pure-first build logs hit `mir_call_hako_atomic_ptr_load_ordered_emit`.
- the EXE stores a native pointer, loads it back, compares i64 transport
  values, and exits `0`.
- `.inc` does not branch on the fixture app name.
- pointer CAS/fetch_add rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh` passes.
