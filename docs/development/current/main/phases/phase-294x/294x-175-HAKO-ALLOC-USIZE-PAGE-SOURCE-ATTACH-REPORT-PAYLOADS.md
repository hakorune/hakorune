---
Status: Landed
Date: 2026-05-24
Scope: object-lifecycle facade page-source attach report payload exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-174
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-174-HAKO-ALLOC-USIZE-PAGE-SOURCE-ATTACH-REPORT-PAYLOAD-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
---

# 294x-175 Hako Alloc Usize Page-Source Attach Report Payloads

## Decision

Migrate only the selected page-source attach report payload fields in
`HakoAllocObjectLifecycleFacadePageSourceAttachReport` to exact `usize`
storage:

- `bytes`
- `block_size`
- `capacity`
- `reserved`

The fresh-page and alloc-miss EXE guards now verify that these payload fields
are exact `usize` while `status`, `added_page_id`, and pointer-like `base` stay
signed.

## Stop Line

This row does not migrate:

- attach report `status`;
- attach report `added_page_id`;
- attach report `base`;
- alloc-miss report source/final mirrors;
- page/block identity payloads;
- pointer-like fields;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
