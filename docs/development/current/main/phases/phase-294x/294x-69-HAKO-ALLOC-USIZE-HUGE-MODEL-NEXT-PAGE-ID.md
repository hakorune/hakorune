---
Status: Landed
Date: 2026-05-23
Scope: huge-page model next-page id exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-68-HAKO-ALLOC-USIZE-HUGE-MODEL-NEXT-PAGE-ID-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_page_model_box.hako
  - tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
---

# 294x-69 Hako Alloc Usize Huge Model Next Page Id

## Decision

Migrate only `HakoAllocHugePageModel.next_page_id` to exact `usize` storage.

The field is the huge-page model's owner-local next page id source. It starts at
`1000`, is copied into signed page-map / metadata-store identity payloads, and
increments only after a successful huge allocation registration. The migration
keeps those published identity payloads signed and does not change huge pointer
or size observer semantics.

## Stop Line

This row does not migrate:

- `HakoAllocHugePageModel.next_ptr`;
- `HakoAllocHugePageModel.last_result_ptr`, `last_page_id`,
  `last_requested_size`, `last_committed_size`, or `last_failure_kind`;
- `HakoAllocHugePageMetaStore` page id / pointer / size payloads;
- page-map entry pointer/id fields or live flags;
- OSVM-backed `next_page_id`;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
