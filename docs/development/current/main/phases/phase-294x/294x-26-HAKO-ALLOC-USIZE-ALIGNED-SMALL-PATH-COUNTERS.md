---
Status: Complete
Date: 2026-05-22
Scope: migrate one aligned-small path counter field group from `i64` to exact
  `usize`.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-25-HAKO-ALLOC-USIZE-PAGE-MAP-REALLOC-FAILURE-CONTRACT-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
  - tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
---

# 294x-26 Hako Alloc Usize Aligned-Small Path Counters

## Decision

Migrate only the `HakoAllocPageMapAlignedSmallPath` event/reject counter group:

- `alloc_count`
- `invalid_alignment_count`
- `oversized_count`
- `alloc_fail_count`
- `register_fail_count`
- `reject_count`

These fields are non-negative counters owned by the aligned-small path. They
classify M178 allocation outcomes and do not carry pointer, alignment, padded
size, or metadata-store count meaning.

## Stop Line

`meta_count`, `next_ptr`, `last_result_ptr`, `last_alignment`, and
`last_padded_size` remain `i64` in this row. `meta_count` mirrors the aligned
metadata store count and should migrate with that owner. `next_ptr` /
`last_result_ptr` are pointer-shaped; `last_alignment` and `last_padded_size`
are request metadata observers.

No metadata-store count migration, page-map entry pointer/id fields, aligned
metadata packed storage, provider activation, host allocator replacement,
hooks, or `#[global_allocator]` are opened by this row.

The guard also requires scalar return contracts on the aligned-small metadata
reader methods (`alignmentFor`, `alignmentAt`, `paddedSizeFor`,
`paddedSizeAt`). These annotations stabilize existing MIR route contracts only;
they do not migrate metadata-store storage.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
