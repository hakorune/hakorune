# 293x-236 M196 Page-Source Decommit Adapter

Status: Complete

## Purpose

M196 connects the M195 bounded decommit policy to the existing hako_alloc page
source by adding a narrow decommit-only adapter. The adapter exposes
`decommitPage(base, bytes)` and delegates to
`HakoAllocPageSourcePolicy.decommitPage(base, bytes)`.

This row opens page-source decommit only. Reserve/commit remain owned by
existing page-source rows, and unreserve / OS release remain closed.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako
```

The adapter is an executor for `HakoAllocBoundedDecommitPolicy`; it records
call/result counters but does not mutate heap/page state.

## Stop Lines

- Do not add reserve or commit behavior to the adapter.
- Do not unreserve pages.
- Do not release OSVM pages.
- Do not mutate heap/page state.
- Do not change allocation, release, realloc, aligned, or huge behavior.
- Do not add provider activation, hooks, or process allocator replacement.
- Do not add env toggles or mutable allocator options.

## Acceptance

- The adapter delegates only to `HakoAllocPageSourcePolicy.decommitPage`.
- M195 bounded policy can call the adapter exactly once for an eligible
  in-bound decision.
- Pure-first EXE proof reserves/commits through the existing facade, then
  decommits through the adapter.
- `unreserve_executed` and `os_release_executed` remain `0`.
- The guard confirms no `.inc` matcher leak and no reserve/commit/unreserve/OS
  release implementation in the adapter.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh
```
