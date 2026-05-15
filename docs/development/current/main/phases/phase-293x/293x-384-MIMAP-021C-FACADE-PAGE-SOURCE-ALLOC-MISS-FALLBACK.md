# 293x-384 MIMAP-021C Facade Page-Source Allocation-Miss Fallback

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-021C` is the next allocator behavior row after `MIMAP-021B`. It may add
one allocation-miss fallback path: when the object-lifecycle facade has no
available page for a small allocation, source exactly one fresh page through the
MIMAP-021B adapter, attach it, and retry the small allocation once.

This row must stay facade-owned and scalar-observer driven. It must not broaden
into page-source decommit/recommit policy, provider activation, page-map lookup,
or remote-free behavior.

## Scope

- Reuse `HakoAllocObjectLifecycleFacadePageSourceAttach`.
- Add one facade-facing allocation-miss route that sources and attaches at most
  one page.
- Retry one small allocation after successful attach.
- Expose scalar proof fields for fallback attempted, source success/failure,
  retry success/failure, and final allocation result.

## Stop Lines

- No release/realloc/alignment behavior changes.
- No purge/reclaim/decommit/recommit execution.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, page-map lookup, unreserve, or OS release.
- No loop over multiple fresh pages.
- No backend `.inc` matcher shortcut.

## Implementation

- Added `object_lifecycle_facade_page_source_alloc_miss_box.hako` as a separate
  owner so MIMAP-021B remains an attach-only seam.
- Added `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback` and
  `HakoAllocObjectLifecycleFacadePageSourceAllocMissReport`.
- Added `apps/mimalloc-facade-page-source-alloc-miss-proof/` as the executable
  proof fixture.
- Added `k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh` to pin
  MIR-owned calls, EXE emit, output shape, and stop lines.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
