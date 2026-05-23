---
Status: Landed
Date: 2026-05-24
Scope: aligned-small path padded-size observer exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-181
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-181-HAKO-ALLOC-USIZE-HUGE-THRESHOLD-OBSERVER-DEFER-ALIGNED-PADDED-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
  - tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
---

# 294x-182 Hako Alloc Usize Aligned-Small Padded Size

## Decision

Migrate only the selected aligned-small path padded-size observer in
`HakoAllocPageMapAlignedSmallPath` to exact `usize` storage:

- `last_padded_size`

The aligned-small path, huge-threshold routing, request-path, and downstream
huge/OSVM comparison guards verify this remains accepted in both VM/MIR and the
pure-first EXE path that reads the owner through the larger comparison slice.

## Stop Line

This row does not migrate:

- `HakoAllocHugeThresholdRouter.last_padded_size`;
- `HakoAllocHugeThresholdRouter.last_good_size`;
- `HakoAllocHugeThresholdRouter.last_huge_threshold`;
- `HakoAllocPageMapAlignedSmallPath.next_ptr`;
- `HakoAllocPageMapAlignedSmallPath.last_result_ptr`;
- `HakoAllocPageMapAlignedSmallPath.last_alignment`;
- aligned-small metadata store payloads;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
bash tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
bash tools/checks/k2_wide_mimalloc_request_path_usize_guard.sh
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
