# mimalloc-remote-free-policy-proof

Purpose: M37 pure-first EXE proof that connects the remote-free mailbox seam to
a small allocator policy box.

This fixture composes existing rows only:

- `TlsCoreBox.cache_slot_set_i64/get_i64`
- direct `hako_atomic_ptr_store_ordered(cell_ptr, value_ptr, order)`
- direct `hako_mem_alloc/free`
- same-module generic-i64 policy method routing

`AllocatorRemoteFreePolicy` owns the app-level policy decision:

- install a mailbox pointer into a TLS cache slot
- publish remote-free block pointers through the mailbox
- release the mailbox slot

The publish method stays straight-line (`TLS get -> pointer store`) so this row
does not widen same-module generic-i64 body acceptance. This is intentionally
not a full mimalloc remote-free list. Without pointer load/CAS, the fixture
proves only the allocator policy integration seam.

This fixture intentionally avoids:

- new MIR route rows
- new NyRT exports
- `ptr_load_ordered`
- `ptr_cas_ordered`
- pointer `fetch_add`
- production allocator remote-free list policy
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
```
