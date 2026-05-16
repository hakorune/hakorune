# 293x-550 MIMAP-063A Reclaim Scheduler Boundary Inventory

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-063A` is the allocator-internal scheduler boundary inventory selected
by `MIMAP-062A`.

The scalar reclaim lane is closed. Before broader reclaim behavior can model
handoff, retry, background purge/reclaim, or abandoned-owner work, this row must
name the scheduler boundary and keep it separate from source-level concurrency
features.

## Scope

- Add an SSOT for allocator-internal reclaim scheduler boundary semantics.
- Inventory the minimum facts needed to request/suppress modeled scheduling.
- Keep worker identity/TLS/atomic substrate as existing prerequisites, not new
  source language features.
- Add proof app and focused guard only if the row introduces a small `.hako`
  boundary owner; otherwise keep it docs/guard inventory.
- Select the next row after the boundary is fixed.

## Stop Lines

- No real thread scheduling.
- No source-level `nowait`, `Channel`, `task_scope`, `co`, `sync box`,
  `context`, or `worker_local` semantics.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `063A.1` | Write scheduler boundary SSOT. | allocator-internal vs source-level concurrency split is explicit. | no language feature |
| `063A.2` | Decide whether a `.hako` boundary owner is needed. | owner/no-owner decision is documented. | no scheduling execution |
| `063A.3` | Add focused guard. | stop lines and current docs are locked. | no provider |
| `063A.4` | Select follow-up row. | next row has one narrow owner. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_boundary_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
