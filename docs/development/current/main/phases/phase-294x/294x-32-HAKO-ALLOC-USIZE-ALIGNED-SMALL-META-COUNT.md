---
Status: Complete
Date: 2026-05-22
Scope: migrate aligned-small metadata store count fields from `i64` to exact `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
  - tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
  - tools/checks/k2_wide_aligned_small_metadata_record_store_guard.sh
---

# 294x-32 Hako Alloc Usize Aligned-Small Meta Count

## Decision

Migrate the aligned-small metadata count seam:

- `HakoAllocAlignedSmallMetaStore.count`
- `HakoAllocPageMapAlignedSmallPath.meta_count`

The metadata store `count` is the owner truth for appended aligned-small
metadata rows. `meta_count` is only the path-local mirror of that owner count,
so it migrates in the same row to avoid an exact-to-signed mirror seam.

## Stop Line

`next_ptr`, `last_result_ptr`, `last_alignment`, and `last_padded_size` remain
`i64` because they are pointer-shaped, alignment, or size observers. Metadata
payload columns (`ptrs`, `alignments`, `padded_sizes`) remain scalar ArrayBox
columns; this row does not open packed-record storage, native aligned
allocation, provider activation, host allocator replacement, hooks, or
`#[global_allocator]`.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
bash tools/checks/k2_wide_aligned_small_metadata_record_store_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
