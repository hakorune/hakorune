---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M35 native pointer atomic store route proof
---

# 293x-087 M35 Native Ptr Atomic Store Route Proof

## Decision

`M35 native pointer atomic store route proof` is live-narrow.

M35 activates exactly one M34 pointer-atomic vocabulary row:
`hako_atomic_ptr_store_ordered(cell_ptr, value_ptr, order)`.

Accepted shape:

```text
externcall "hako_atomic_ptr_store_ordered"(cell_ptr, value_ptr, order)
  route_id = extern.hako_atomic.ptr_store_ordered
  core_op = HakoAtomicPtrStoreOrdered
  symbol = hako_atomic_ptr_store_ordered
  arity = 3
  return_shape = scalar_i64
  value_demand = native_ptr_nullable
```

The pure-first backend converts the first two i64 transport values to LLVM
`ptr`, passes the order as i64, and consumes only the MIR-owned route fact.

## Owned

- `apps/mimalloc-ptr-atomic-store-proof/`
- MIR extern route row for `hako_atomic_ptr_store_ordered/3`
- pure-first `.inc` declaration/need/emit row for that route id.
- NyRT export for the symbol.
- Guard:
  `tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh`

## Not Owned

- `ptr_load_ordered`.
- `ptr_cas_ordered`.
- pointer `fetch_add`.
- AtomicCoreBox pointer methods.
- Memory-order verifier widening beyond the store helper's accepted order set.
- Production remote-free allocator policy.
- LLVM native pointer attrs or noalias/nonnull widening.

## Gate

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh
```

The guard must verify:

- MIR JSON publishes the pointer store extern route fact.
- pure-first build logs hit `mir_call_hako_atomic_ptr_store_ordered_emit`.
- the EXE stores a native pointer into a native pointer cell and exits `0`.
- `.inc` does not branch on the fixture app name.
- pointer load/CAS rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh` passes.
