# 293x-396 MIMAP-REMOTE-001 Remote-Free / Abandoned-Owner Policy

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-REMOTE-001` is the next allocator substrate row after
`MIMAP-ATOMIC-001`. It may model allocator-owned remote-free and
abandoned-owner policy on top of the already-live worker identity, TLS
cache-slot, and atomic route rows.

This row is allocator policy modeling only. It must not activate true parallel
runtime behavior, public concurrency language semantics, provider hooks, host
allocator replacement, or page ownership transfer outside the explicit policy
owner.

## Scope

- Reuse existing worker/TLS/atomic substrate rows.
- Keep policy state in `hako_alloc` allocator-facing owner boxes.
- Prove same-owner / remote-owner / abandoned-owner scalar transitions without
  needing true threads.
- Keep VM/reference behavior deterministic; native parallel stress stays parked.

## Existing Proof Role Table

| Proof / Owner | Role For This Row | Status |
| --- | --- | --- |
| `mimalloc-remote-free-i64-proof` | Historical fixed-slot i64 LIFO sketch. Keep as route-shape evidence; do not promote to production owner. | reused guard |
| `mimalloc-tls-ptr-remote-free-proof` | Historical TLS mailbox + pointer-store seam. Keep as mailbox evidence; do not duplicate policy. | reference only |
| `mimalloc-remote-free-policy-proof` | Historical app-local mailbox policy over TLS + pointer-store. Keep as substrate evidence. | reused guard |
| `mimalloc-ptr-remote-free-list-proof` | Pointer load/store/CAS two-node list proof. Keep route evidence under pointer atomic rows. | reference only |
| `mimalloc-remote-free-list-policy-proof` | Same-module wrapper around the two-node pointer list shape. Keep as intermediate proof. | reference only |
| `mimalloc-remote-free-retry-loop-proof` | Bounded pointer CAS retry loop shape. Reuse through `HakoAllocRemoteFreePolicy`. | owner source |
| `HakoAllocRemoteFreePolicy` | Current bounded pointer remote-free policy owner. MIMAP-REMOTE-001 may call it; it must not grow provider/hook behavior. | active owner |
| `HakoAllocRemoteFreePageInbox` | Page-local remote-free collection proof. Keep page mutation local to M170; MIMAP-REMOTE-001 does not claim arbitrary page ownership. | reference only |
| `HakoAllocThreadHeapOwnerInventory` | Read-only owner-token classification for abandoned-owner eligibility. | active inventory |
| `HakoAllocAbandonedReclaimInventory` | Read-only abandoned/reclaim classification. Reclaim execution remains inactive. | active inventory |

MIMAP-REMOTE-001 adds only the small composition owner that decides which of the
already-proven roles applies. It must not create a second remote-free queue
implementation or a second abandoned/reclaim owner.

## Stop Lines

- No source-level `worker_local`, `lock<T>`, Channel, `task_scope`,
  `nowait`/`await`, or true thread-pool semantics.
- No arbitrary page-map pointer lookup unless a separate page ownership row
  explicitly owns it.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend `.inc` matcher shortcut.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

The consolidated guard must run and require these existing proof guards:

```text
k2_wide_mimalloc_remote_free_i64_exe_guard.sh
k2_wide_mimalloc_remote_free_policy_exe_guard.sh
```

## Implementation

- Added `HakoAllocRemoteAbandonedOwnerPolicy` as the allocator-facing
  composition owner for same-owner, remote-owner, abandoned-owner, and reject
  decisions.
- Reused `HakoAllocWorkerTlsCache`, `HakoAllocRemoteFreePolicy`,
  `HakoAllocThreadHeapOwnerInventory`, and
  `HakoAllocAbandonedReclaimInventory` without adding route rows.
- Added `apps/mimalloc-remote-abandoned-owner-policy-proof` and
  `k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh`.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
