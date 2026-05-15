# 293x-389 MIMAP-SUBSTRATE-CONC-001 Concurrency Substrate Cut

Status: landed
Date: 2026-05-15

## Decision

Mimalloc migration depends on allocator concurrency substrate, not on expanding
the user-facing concurrency language surface.

This row adds the boundary SSOT and task order before implementation rows.

## Scope

- Add `docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`.
- Update the phase-293x mimalloc taskboard with the substrate concurrency wave.
- Select `MIMAP-SUBSTRATE-CONC-002` as the next row.

## Stop Lines

- No behavior changes.
- No source-level `worker_local`.
- No `lock<T>` syntax work.
- No `ChannelBox` / `task_scope` work.
- No true parallel VM work.
- No provider hook, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
