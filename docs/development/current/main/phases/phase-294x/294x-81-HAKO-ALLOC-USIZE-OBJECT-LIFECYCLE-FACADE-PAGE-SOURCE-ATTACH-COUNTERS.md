---
Status: Landed
Date: 2026-05-23
Scope: object-lifecycle facade page-source attach owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-80-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-FACADE-PAGE-SOURCE-ATTACH-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako
  - apps/mimalloc-facade-page-source-fresh-page-proof/main.hako
  - apps/mimalloc-facade-page-source-alloc-miss-proof/main.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
---

# 294x-81 Hako Alloc Usize Object Lifecycle Facade Page Source Attach Counters

## Decision

Migrate only the owner-local monotonic counters in
`object_lifecycle_facade_page_source_box.hako` to exact `usize` storage:

- `HakoAllocObjectLifecycleFacadePageSourceAttach.reserve_count`
- `HakoAllocObjectLifecycleFacadePageSourceAttach.commit_count`
- `HakoAllocObjectLifecycleFacadePageSourceAttach.attach_count`
- `HakoAllocObjectLifecycleFacadePageSourceAttach.reject_count`

The fresh-page attach and alloc-miss guards now assert the attach owner itself
uses exact `usize` storage for these counters while the attached report object
remains fully signed. This keeps the page-source/OSVM-adjacent observer seam
stable and narrows the row to the owner-local bookkeeping only.

## Stop Line

This row does not migrate:

- `HakoAllocObjectLifecycleFacadePageSourceAttachReport.status`;
- any `source_*` report field;
- `added_page_id`, `facade_page_count`, `base`, `bytes`, `block_size`,
  `capacity`, or `reserved`;
- page/block identity payloads, pointer-like fields, unrelated lifecycle
  observer owners, unrelated OSVM/bin/provider/hook seams, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
