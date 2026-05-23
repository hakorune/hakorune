---
Status: Landed
Date: 2026-05-24
Scope: object-lifecycle facade page-source alloc-miss source byte mirror exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-178
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-178-HAKO-ALLOC-USIZE-PAGE-SOURCE-ALLOC-MISS-SOURCE-BYTE-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
---

# 294x-179 Hako Alloc Usize Page-Source Alloc-Miss Source Byte

## Decision

Migrate only the selected alloc-miss report source byte-length mirror in
`HakoAllocObjectLifecycleFacadePageSourceAllocMissReport` to exact `usize`
storage:

- `source_bytes`

The alloc-miss EXE guard now verifies that this byte-length mirror is exact
`usize` while the status, reason, identity, and pointer-like seams stay signed.

## Stop Line

This row does not migrate:

- alloc-miss report `status`;
- alloc-miss report `initial_*`, `retry_*`, or `final_*`;
- alloc-miss report `fallback_attempted`;
- alloc-miss report `source_status`;
- alloc-miss report `source_added_page_id`;
- alloc-miss report `source_base`;
- page/block identity payloads;
- pointer-like fields;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
