# mimalloc-worker-tls-cache-proof

MIMAP-TLS-001 proof fixture for allocator-internal worker TLS cache-slot
state.

Scope:

- Compose `HakoAllocWorkerIdentity.currentWorkerId()` with
  `TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64`.
- Prove the single-worker lane can store, read, clear, and observe an allocator
  cache slot.
- Expose scalar proof state for slot id, stored value, observed worker id,
  get/set counts, and worker identity call count.

Non-goals:

- No source-level worker-local syntax or public TLS cell API.
- No atomics, remote-free, page ownership transfer, task scheduling, provider
  hooks, or allocator replacement.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_worker_tls_cache_exe_guard.sh
```
