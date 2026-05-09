# mimalloc-ptr-remote-free-list-proof

Purpose: M41 pure-first EXE proof that composes the active native pointer
atomic store/load/CAS routes into a minimal remote-free list push shape.

The fixture uses:

- `hako_atomic_ptr_store_ordered` to initialize the head cell and write each
  block's first word as a next pointer.
- `hako_atomic_ptr_load_ordered` to observe the head and next links.
- `hako_atomic_ptr_cas_ordered` to publish each block as the new head.

This fixture intentionally avoids:

- new MIR extern route rows
- pointer `fetch_add`
- `AtomicCoreBox` pointer methods
- production allocator policy
- pointer arithmetic or layout syntax
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh
```
