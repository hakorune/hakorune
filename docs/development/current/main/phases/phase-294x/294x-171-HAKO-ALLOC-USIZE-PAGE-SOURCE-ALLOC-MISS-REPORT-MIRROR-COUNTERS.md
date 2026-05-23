---
Status: Landed
Date: 2026-05-24
Scope: object-lifecycle facade page-source alloc-miss report mirror counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-170
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-170-HAKO-ALLOC-USIZE-PAGE-SOURCE-ALLOC-MISS-REPORT-MIRROR-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
---

# 294x-171 Hako Alloc Usize Page-Source Alloc-Miss Report Mirror Counters

## Decision

Migrate only the selected alloc-miss report mirror counters in
`HakoAllocObjectLifecycleFacadePageSourceAllocMissReport` to exact `usize`
storage:

- `fallback_attempt_count`
- `source_success_count`
- `source_failure_count`
- `retry_success_count`
- `retry_failure_count`

These fields mirror the already-exact owner-local fallback counters. The
alloc-miss EXE guard now verifies that both the owner counters and the report
mirror counters use exact `usize` typed-object storage.

## Stop Line

This row does not migrate:

- alloc-miss report `status`, `initial_*`, `fallback_attempted`, `source_*`,
  `retry_*`, `final_*`, `source_base`, `source_bytes`, `source_added_page_id`,
  `final_page_id`, or `final_block_id`;
- attach report status/source/page payload mirrors;
- page/block identity payloads;
- pointer-like fields;
- object lifecycle facade result status/reason/ok mirrors;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
