# 293x-247 M203 Page Source Recommit Adapter

Status: Complete

## Purpose

M203 wires the M202 bounded recommit policy to the real page-source commit
operation through a recommit-only adapter.

This row opens only:

```text
HakoAllocPageSourceRecommitAdapter.commitPage(base, bytes)
  -> HakoAllocPageSourcePolicy.commitPage(base, bytes)
```

Heap integration, marker transition/clear, unreserve, OS release, allocation,
release, realloc, aligned, and huge behavior remain unchanged.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_page_source_recommit_adapter_box.hako
```

The adapter exposes only `commitPage(base, bytes)` for M202. It does not expose
reserve, decommit, unreserve, or release APIs.

## Stop Lines

- Do not add heap-level recommit integration in this row.
- Do not clear or mutate the M198 decommit state marker.
- Do not mutate heap/page/backing state.
- Do not call `reservePage(...)`, `decommitPage(...)`, `unreserve(...)`, or
  OS release APIs from the recommit adapter.
- Do not change allocation, release, realloc, aligned, huge, or purge decommit
  behavior.
- Do not add provider activation, hooks, or process allocator replacement.

## Acceptance

- M202 can recommit a decommitted-page decision through the recommit-only page
  source adapter.
- Adapter counters record one commit call and one success.
- `marker_cleared`, `unreserve_executed`, and `os_release_executed` remain `0`.
- The decommit marker remains marked after recommit until a future marker
  transition row.
- The heap's direct `decommit_count` remains `0`; setup decommit still flows
  through M197/M199 state-aware decommit.
- Pure-first EXE proof output matches the recommit adapter matrix.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_recommit_adapter_guard.sh
```
