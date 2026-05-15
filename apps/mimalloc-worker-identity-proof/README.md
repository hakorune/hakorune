# mimalloc-worker-identity-proof

MIMAP-WORKER-001 proof fixture for allocator-internal worker identity.

Scope:

- Route `HakoAllocWorkerIdentity.currentWorkerId()` through
  `WorkerCoreBox.current_id_i64()`.
- Prove the single-worker lane returns worker id `0`.
- Expose scalar call-count and last-worker-id proof state.

Non-goals:

- No source-level `worker_local` syntax or public worker identity semantics.
- No TLS/cache slots, atomics, remote-free, task scheduling, provider hooks, or
  allocator replacement.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_worker_identity_exe_guard.sh
```
