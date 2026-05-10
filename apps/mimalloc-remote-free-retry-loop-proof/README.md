# mimalloc-remote-free-retry-loop-proof

Purpose: M43 pure-first EXE proof that the same-module allocator remote-free
policy can own a bounded CAS retry loop over the existing native pointer atomic
routes.

`AllocatorRemoteFreeRetryPolicy.push_retry` owns:

- load old head
- store `block.next = old_head`
- optionally inject one competing push to force the first CAS failure
- retry until CAS publishes the requested block

This fixture intentionally avoids:

- new MIR extern route rows
- new NyRT exports
- pointer `fetch_add`
- `AtomicCoreBox` pointer methods
- production allocator policy or unbounded runtime loop
- pointer arithmetic or layout syntax
- LLVM native pointer attrs
- app-specific backend matchers

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh
```
