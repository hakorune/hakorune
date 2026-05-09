---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M40 native pointer atomic CAS route proof
---

# 293x-092 M40 Native Ptr Atomic CAS Route Proof

## Decision

`M40 native pointer atomic CAS route proof` is live-narrow.

M40 activates exactly one additional M34 pointer-atomic vocabulary row:
`hako_atomic_ptr_cas_ordered(cell_ptr, expected_ptr, desired_ptr,
success_order, failure_order)`.

Accepted shape:

```text
externcall "hako_atomic_ptr_cas_ordered"(
  cell_ptr,
  expected_ptr,
  desired_ptr,
  success_order,
  failure_order
)
  route_id = extern.hako_atomic.ptr_cas_ordered
  core_op = HakoAtomicPtrCasOrdered
  symbol = hako_atomic_ptr_cas_ordered
  arity = 5
  return_shape = native_ptr_nullable
  value_demand = native_ptr_nullable
```

The pure-first backend converts the cell/expected/desired i64 transport values
to LLVM `ptr`, passes success/failure orders as i64, calls the runtime helper
returning `ptr`, and converts the result back to i64 transport. No native
pointer attrs are emitted.

## Owned

- `apps/mimalloc-ptr-atomic-cas-proof/`
- MIR extern route row for `hako_atomic_ptr_cas_ordered/5`.
- pure-first `.inc` declaration/need/emit row for that route id.
- NyRT export for the symbol.
- Guard:
  `tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh`
- Guard contract sync: M34-M39 guards now keep pointer fetch_add inactive while
  allowing the M40 CAS row.

## Not Owned

- pointer `fetch_add`.
- AtomicCoreBox pointer methods.
- Production remote-free allocator policy.
- LLVM native pointer attrs or noalias/nonnull widening.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh
bash tools/checks/k2_wide_pointer_atomic_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must verify:

- MIR JSON publishes the pointer CAS extern route fact.
- pure-first build logs hit `mir_call_hako_atomic_ptr_cas_ordered_emit`.
- the EXE observes successful and failed CAS return values and cell state.
- `.inc` does not branch on the fixture app name.
- pointer fetch_add rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh` passes.
