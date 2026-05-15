# 293x-377 MIMAP-FACADE-CLEAN-001 Result Capsules

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-FACADE-CLEAN-001` closes by moving facade scalar observer state into
small owned result capsules. The public facade observer methods remain stable;
the facade stays the orchestration owner.

## Scope

- Add `object_lifecycle_facade_result_box.hako`.
- Move allocation, release, alignment, and realloc last-result fields/counters
  out of `object_lifecycle_facade_box.hako`.
- Keep `objectLifecycleAlloc*`, `objectLifecycleRelease*`,
  `objectLifecycleAlignment*`, and `objectLifecycleRealloc*` methods as the
  public observer API.
- Update facade guards so field ownership is checked in the result capsule file
  while route proofs continue to check facade behavior.

## Non-goals

- No allocator behavior change.
- No public observer method rename.
- No page-map lookup or arbitrary pointer-to-page resolution.
- No byte copy, native aligned placement, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or backend shortcut.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_cleanup_reason_ssot_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_alignment_metadata_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_aligned_alloc_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_realloc_shrink_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_realloc_grow_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
