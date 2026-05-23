---
Status: Landed
Date: 2026-05-23
Scope: object-lifecycle facade page-source alloc-miss owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-82-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-FACADE-PAGE-SOURCE-ALLOC-MISS-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
  - apps/mimalloc-facade-page-source-alloc-miss-proof/main.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
---

# 294x-83 Hako Alloc Usize Object Lifecycle Facade Page Source Alloc Miss Counters

## Decision

Migrate only the owner-local monotonic counters in
`object_lifecycle_facade_page_source_alloc_miss_box.hako` to exact `usize`
storage:

- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.fallback_attempt_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.source_success_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.source_failure_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.retry_success_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.retry_failure_count`

The alloc-miss guard now asserts the fallback owner itself uses exact `usize`
storage for these counters while the alloc-miss report object keeps the signed
count mirrors and all status/source/final observer fields unchanged. This keeps
the owner/report split strict and narrows the row to the fallback bookkeeping
only.

## Stop Line

This row does not migrate:

- attach-report `status`, `source_*`, `added_page_id`, `facade_page_count`,
  `base`, `bytes`, `block_size`, `capacity`, or `reserved`;
- alloc-miss report `status`, `initial_*`, `fallback_attempted`, `source_*`,
  `retry_*`, `final_*`, `source_base`, `source_bytes`, `final_page_id`, or
  `final_block_id`;
- alloc-miss report-mirror counts while the owner/report split stays strict;
- page/block identity payloads, pointer-like fields, unrelated lifecycle
  observer owners, huge-page-source / huge-failfast seams, unrelated OSVM/bin /
  provider / hook / global-allocator work, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
