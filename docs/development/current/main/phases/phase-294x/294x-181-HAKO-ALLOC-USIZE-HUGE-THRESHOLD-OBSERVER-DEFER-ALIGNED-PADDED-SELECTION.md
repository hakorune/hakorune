---
Status: Landed
Date: 2026-05-24
Scope: defer huge-threshold router observer migration and select the aligned-small padded-size dependency.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-180
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-180-HAKO-ALLOC-USIZE-HUGE-THRESHOLD-OBSERVER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
  - lang/src/hako_alloc/memory/huge_threshold_router_box.hako
  - tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
---

# 294x-181 Hako Alloc Usize Huge-Threshold Observer Defer / Aligned Padded Selection

## Decision

Do not migrate `HakoAllocHugeThresholdRouter` size observers in isolation.

Reason: the narrow VM/MIR huge-threshold routing guard accepted the direct
router observer migration, but the downstream pure-first huge/OSVM comparison
EXE path failed during execution. Keep the router observer group selected but
deferred until the backend-facing exact-`usize` object-field path is stable for
that consumer.

Select the upstream aligned-small padded-size observer as the next safe exact
`usize` row instead:

- `HakoAllocPageMapAlignedSmallPath.last_padded_size`

This field is reset to `0` or assigned after `padded_request_size(...)` has
been validated as non-negative. It does not carry pointer, alignment, status,
reason, or identity semantics.

## Stop Line

The follow-on row must not migrate:

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
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
