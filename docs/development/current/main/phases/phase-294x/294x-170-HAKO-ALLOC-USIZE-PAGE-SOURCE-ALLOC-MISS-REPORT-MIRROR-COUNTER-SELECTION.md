---
Status: Landed
Date: 2026-05-24
Scope: select the next object-lifecycle facade page-source alloc-miss report mirror exact `usize` row.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-169
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-83-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-FACADE-PAGE-SOURCE-ALLOC-MISS-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
---

# 294x-170 Hako Alloc Usize Page-Source Alloc-Miss Report Mirror Counter Selection

## Decision

Select the alloc-miss report mirror counters in
`HakoAllocObjectLifecycleFacadePageSourceAllocMissReport` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-170`.

Chosen fields:

- `fallback_attempt_count`
- `source_success_count`
- `source_failure_count`
- `retry_success_count`
- `retry_failure_count`

These fields only mirror the already-exact owner-local counters in
`HakoAllocObjectLifecycleFacadePageSourceAllocMissFallback`. They are
non-negative and have no sentinel values.

## Stop Line

The follow-on row must not migrate:

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
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
