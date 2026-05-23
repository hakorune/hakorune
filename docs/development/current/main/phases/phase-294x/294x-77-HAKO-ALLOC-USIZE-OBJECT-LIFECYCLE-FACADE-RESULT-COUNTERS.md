---
Status: Landed
Date: 2026-05-23
Scope: object-lifecycle facade source-owner alloc/release counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-76-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-FACADE-RESULT-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako
  - lang/src/hako_alloc/memory/object_lifecycle_facade_stats_box.hako
  - apps/mimalloc-facade-small-alloc-stats-proof/main.hako
  - apps/mimalloc-facade-release-one-block-proof/main.hako
  - apps/mimalloc-facade-release-failfast-proof/main.hako
  - apps/mimalloc-facade-stats-snapshot-proof/main.hako
  - tools/checks/k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
---

# 294x-77 Hako Alloc Usize Object Lifecycle Facade Result Counters

## Decision

Migrate only the facade source-owner monotonic alloc/release counters in
`object_lifecycle_facade_result_box.hako` to exact `usize` storage:

- `HakoAllocObjectLifecycleAllocResult.attempt_count`
- `HakoAllocObjectLifecycleAllocResult.success_count`
- `HakoAllocObjectLifecycleAllocResult.failure_count`
- `HakoAllocObjectLifecycleAllocResult.reusable_success_count`
- `HakoAllocObjectLifecycleAllocResult.active_success_count`
- `HakoAllocObjectLifecycleReleaseResult.success_count`
- `HakoAllocObjectLifecycleReleaseResult.failure_count`

The acceptance bundle now asserts those exact source-owner fields directly
through typed-object-plan checks, direct facade getters, and the unchanged
stats-snapshot downstream mirror. `NUMERIC_FIELDS.md` now inventories both the
facade result owner and the still-signed stats snapshot owner so the later
mirror row can start from an explicit baseline.

## Stop Line

This row does not migrate:

- `last_page_id`, `last_block_id`, `last_reason`, or `last_ok`;
- any `HakoAllocObjectLifecycleAlignmentResult` field;
- any `HakoAllocObjectLifecycleReallocResult` field;
- `HakoAllocObjectLifecycleFacadeStatsSnapshot` counts or terminal total helpers;
- alignment requested/normalized observers, realloc ids/requested-size
  observers, failure vocabularies, page/block identity payloads, pointer-like
  fields, unrelated OSVM/bin seams, provider activation, hooks, TLS, atomics,
  or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
