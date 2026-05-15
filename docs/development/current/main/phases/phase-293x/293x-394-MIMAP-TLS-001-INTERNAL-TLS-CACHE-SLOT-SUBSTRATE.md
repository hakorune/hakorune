# 293x-394 MIMAP-TLS-001 Internal TLS Cache-Slot Substrate

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-TLS-001` is the next allocator substrate row after
`MIMAP-WORKER-001`. It may adapt the existing `hako.tls` cache-slot route into
an allocator-facing worker-local cache observer/model.

This remains runtime/internal allocator substrate. It must not open
source-level `worker_local` syntax, public TLS cells, or true parallel language
semantics.

## Scope

- Reuse existing `TlsCoreBox.cache_slot_get_i64/set_i64` routes.
- Connect cache-slot proof state to the allocator worker identity row.
- Expose scalar proof fields for slot id, stored value, observed worker id, and
  operation counts.

## Stop Lines

- No source-level `worker_local` syntax or public TLS cell API.
- No atomics, remote-free, abandoned-owner policy, or page ownership transfer.
- No task scheduler, `nowait`/`await`, Channel, `task_scope`, or true thread
  pool semantics.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend `.inc` matcher shortcut.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_worker_tls_cache_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Added `HakoAllocWorkerTlsCache` as the allocator-facing owner for
  worker-local cache-slot proof state.
- Composed `HakoAllocWorkerIdentity.currentWorkerId()` with the existing
  `TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64` substrate routes.
- Added `apps/mimalloc-worker-tls-cache-proof` and
  `k2_wide_mimalloc_worker_tls_cache_exe_guard.sh` to prove the scalar
  slot/value/worker/count state in pure-first EXE.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_worker_tls_cache_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
