# 293x-396 MIMAP-REMOTE-001 Remote-Free / Abandoned-Owner Policy

Status: ready
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

## Stop Lines

- No source-level `worker_local`, `lock<T>`, Channel, `task_scope`,
  `nowait`/`await`, or true thread-pool semantics.
- No arbitrary page-map pointer lookup unless a separate page ownership row
  explicitly owns it.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend `.inc` matcher shortcut.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
