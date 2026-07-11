---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local object-lifecycle facade page-source attach exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako
  - apps/mimalloc-facade-page-source-fresh-page-proof/main.hako
  - apps/mimalloc-facade-page-source-alloc-miss-proof/main.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
---

# 294x-80 Hako Alloc Usize Object Lifecycle Facade Page Source Attach Selection

## Decision

Select the owner-local monotonic counter owner in
`object_lifecycle_facade_page_source_box.hako` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-103`.

Chosen fields:

- `HakoAllocObjectLifecycleFacadePageSourceAttach.reserve_count`
- `HakoAllocObjectLifecycleFacadePageSourceAttach.commit_count`
- `HakoAllocObjectLifecycleFacadePageSourceAttach.attach_count`
- `HakoAllocObjectLifecycleFacadePageSourceAttach.reject_count`

These four fields are the only coherent surgical subset in this owner. They are
monotonic, non-negative, and stay fully owner-local while the existing fresh-page
attach and alloc-miss proofs exercise them without widening the page-source
report observer seam.

## Stop Line

The follow-on row must not migrate:

- `HakoAllocObjectLifecycleFacadePageSourceAttachReport.status`;
- any `source_*` report field, including reserve/commit/reject mirrors;
- `added_page_id`, `facade_page_count`, `base`, `bytes`, `block_size`,
  `capacity`, or `reserved`;
- page/block identity payloads, pointer-like fields, unrelated lifecycle
  observer owners, unrelated OSVM/bin/provider/hook seams, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
