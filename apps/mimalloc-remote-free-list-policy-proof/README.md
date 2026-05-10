# mimalloc-remote-free-list-policy-proof

Purpose: M42 pure-first EXE proof that moves the M41 pointer CAS remote-free
list push shape behind a same-module allocator policy box.

`AllocatorRemoteFreeListPolicy` owns:

- head initialization through `hako_atomic_ptr_store_ordered`
- two-step push shape: load old head, store block next, CAS head
- readback helpers for head and next pointers

This fixture intentionally avoids:

- new MIR extern route rows
- new NyRT exports
- pointer `fetch_add`
- `AtomicCoreBox` pointer methods
- production allocator policy or retry loop
- pointer arithmetic or layout syntax
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh
```
