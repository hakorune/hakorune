# mimalloc-remote-abandoned-owner-policy-proof

MIMAP-REMOTE-001 proof fixture for allocator-owned remote-free and
abandoned-owner policy.

Scope:

- Compose `HakoAllocWorkerTlsCache`, `HakoAllocRemoteFreePolicy`,
  `HakoAllocThreadHeapOwnerInventory`, and
  `HakoAllocAbandonedReclaimInventory`.
- Prove same-owner, active remote-owner publish, abandoned-owner candidate, and
  remote-pending reject paths without true threads.
- Keep route ownership in the existing worker/TLS/atomic rows.

Non-goals:

- No source-level `worker_local`, `lock<T>`, Channel, or task scheduling.
- No arbitrary page-map lookup, page ownership mutation, reclaim execution,
  provider hooks, or allocator replacement.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh
```
