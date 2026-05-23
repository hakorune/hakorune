---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local object-lifecycle facade page-source alloc-miss exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
  - apps/mimalloc-facade-page-source-alloc-miss-proof/main.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
---

# 294x-82 Hako Alloc Usize Object Lifecycle Facade Page Source Alloc Miss Selection

## Decision

Select the owner-local monotonic counter owner in
`object_lifecycle_facade_page_source_alloc_miss_box.hako` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-105`.

Chosen fields:

- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.fallback_attempt_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.source_success_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.source_failure_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.retry_success_count`
- `HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback.retry_failure_count`

These five fields are the only coherent surgical subset in this owner. They are
monotonic, non-negative, and stay fully owner-local while the existing
alloc-miss proof exercises the fallback path without widening the signed
alloc-miss report mirror seam.

## Stop Line

The follow-on row must not migrate:

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
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
