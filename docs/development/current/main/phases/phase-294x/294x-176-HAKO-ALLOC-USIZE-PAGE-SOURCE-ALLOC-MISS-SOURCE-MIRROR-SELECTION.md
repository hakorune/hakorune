---
Status: Landed
Date: 2026-05-24
Scope: select the next object-lifecycle facade page-source alloc-miss source mirror exact `usize` row.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-175
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-175-HAKO-ALLOC-USIZE-PAGE-SOURCE-ATTACH-REPORT-PAYLOADS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
---

# 294x-176 Hako Alloc Usize Page-Source Alloc-Miss Source Mirror Selection

## Decision

Select the alloc-miss report fields that mirror the already exact page-source
attach report counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-176`.

Chosen fields in
`HakoAllocObjectLifecycleFacadePageSourceAllocMissReport`:

- `source_reserved`
- `source_committed`
- `source_reject`
- `source_facade_page_count`

These fields mirror non-negative count fields from
`HakoAllocObjectLifecycleFacadePageSourceAttachReport`. They do not carry
status, reason, boolean, identity, pointer, or byte-length semantics.

## Stop Line

The follow-on row must not migrate:

- alloc-miss report `status`;
- alloc-miss report `initial_*`, `retry_*`, or `final_*`;
- alloc-miss report `fallback_attempted`;
- alloc-miss report `source_status`;
- alloc-miss report `source_added_page_id`;
- alloc-miss report `source_base`;
- alloc-miss report `source_bytes`;
- page/block identity payloads;
- pointer-like fields;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
