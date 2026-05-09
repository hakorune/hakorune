# mimalloc-tls-ptr-remote-free-proof

Purpose: M36 pure-first EXE composition proof for a remote-free mailbox seam.

This fixture composes existing rows only:

- `TlsCoreBox.cache_slot_set_i64/get_i64`
- direct `hako_atomic_ptr_store_ordered(cell_ptr, value_ptr, order)`
- direct `hako_mem_alloc/free`

The app stores a native mailbox pointer in the TLS cache slot, reads it back,
then publishes a native block pointer into that mailbox using release-order
pointer store. This is intentionally only a composition proof, not production
remote-free policy.

This fixture intentionally avoids:

- `ptr_load_ordered`
- `ptr_cas_ordered`
- pointer `fetch_add`
- new MIR route rows
- new NyRT exports
- production allocator remote-free policy
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh
```
