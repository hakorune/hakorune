---
Status: Landed
Date: 2026-05-23
Scope: select page-map release seam page-count exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
---

# 294x-64 Hako Alloc Usize Page Map Release Page Count Selection

## Decision

Select `HakoAllocPageMapReleaseSeam.page_count` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-087`.

`page_count` is the release seam page-array length. The owner already rejects
negative `page_id` values before comparing `page_id >= page_count`, so the
follow-on row can migrate the length field while keeping the signed page id
guard as the explicit id/index seam.

## Stop Line

The follow-on row must not migrate:

- page-map entry `ptr`, `page_id`, `block_id`, or `live`;
- `HakoAllocPageMapReleaseSeam` event/reject counters already migrated in
  `HAKO-ALLOC-USIZE-FIELD-GROUP-052`;
- page/block identity values;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
