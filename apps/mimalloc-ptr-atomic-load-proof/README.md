# mimalloc-ptr-atomic-load-proof

Purpose: M39 pure-first EXE proof for the second active native pointer atomic
route: `hako_atomic_ptr_load_ordered(cell_ptr, order)`.

This fixture uses direct extern calls so the route owner stays MIR metadata,
not `AtomicCoreBox` pointer methods. It allocates a native pointer cell and a
native pointer value through `hako_mem_alloc`, stores the value pointer with the
M35 pointer-store route, loads it back with acquire order, compares transport
values, then frees both allocations.

This fixture intentionally avoids:

- `ptr_cas_ordered`
- pointer `fetch_add`
- `AtomicCoreBox` pointer methods
- production remote-free allocator policy
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh
```
