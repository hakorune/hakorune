# mimalloc-ptr-atomic-store-proof

Purpose: M35 pure-first EXE proof for the first native pointer atomic route:
`hako_atomic_ptr_store_ordered(cell_ptr, value_ptr, order)`.

This fixture uses direct extern calls so the route owner stays MIR metadata,
not `AtomicCoreBox` pointer methods. It allocates a native pointer cell and a
native pointer value through `hako_mem_alloc`, stores the value pointer into the
cell with relaxed order, then frees both allocations.

This fixture intentionally avoids:

- `ptr_load_ordered`
- `ptr_cas_ordered`
- pointer `fetch_add`
- `AtomicCoreBox` pointer methods
- production remote-free allocator policy
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh
```
