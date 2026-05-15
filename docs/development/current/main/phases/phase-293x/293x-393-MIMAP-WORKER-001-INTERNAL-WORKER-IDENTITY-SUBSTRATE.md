# 293x-393 MIMAP-WORKER-001 Internal Worker Identity Substrate

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-WORKER-001` is the next allocator substrate row after `MIMAP-021C`.
It may add an allocator-internal worker/thread identity substrate so later
TLS/cache and remote-free rows can key policy by a stable worker id.

This is not a source-level worker identity feature. The VM/reference lane may
return worker id `0`; native/EXE behavior should remain a narrow substrate
smoke, not a true parallel language semantics claim.

## Scope

- Add one allocator-facing worker id route or model owner.
- Expose scalar proof fields for current worker id and call count.
- Keep worker id deterministic in the single-worker proof.
- Prepare the next TLS/cache-slot row without implementing cache slots here.

## Stop Lines

- No source-level `worker_local` syntax or public worker identity semantics.
- No TLS/cache slot storage in this row.
- No atomics, remote-free, abandoned-owner policy, or page ownership transfer.
- No task scheduler, `nowait`/`await`, Channel, `task_scope`, or true thread
  pool semantics.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend `.inc` matcher shortcut.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_worker_identity_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
