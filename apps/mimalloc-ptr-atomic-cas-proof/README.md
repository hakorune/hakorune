# mimalloc-ptr-atomic-cas-proof

Purpose: M40 pure-first EXE proof for the third active native pointer atomic
route: `hako_atomic_ptr_cas_ordered(cell_ptr, expected_ptr, desired_ptr,
success_order, failure_order)`.

This fixture uses direct extern calls so the route owner stays MIR metadata,
not `AtomicCoreBox` pointer methods. It allocates a native pointer cell plus
three native pointer values, stores the old value pointer, performs a
successful CAS to the new pointer, then performs a failed CAS that must return
the observed new pointer and leave the cell unchanged.

This fixture intentionally avoids:

- pointer `fetch_add`
- `AtomicCoreBox` pointer methods
- production remote-free allocator policy
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh
```
