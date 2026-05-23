---
Status: Landed
Date: 2026-05-23
Scope: object-lifecycle facade stats snapshot mirror exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-78-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-FACADE-STATS-SNAPSHOT-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_stats_box.hako
  - lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako
  - apps/mimalloc-facade-small-alloc-stats-proof/main.hako
  - apps/mimalloc-facade-release-one-block-proof/main.hako
  - apps/mimalloc-facade-release-failfast-proof/main.hako
  - apps/mimalloc-facade-stats-snapshot-proof/main.hako
  - tools/checks/k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
---

# 294x-79 Hako Alloc Usize Object Lifecycle Facade Stats Snapshot Counters

## Decision

Migrate only the seven downstream snapshot mirror counters in
`object_lifecycle_facade_stats_box.hako` to exact `usize` storage:

- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_attempt_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_failure_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_reusable_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_active_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.release_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.release_failure_count`

The stats snapshot proof and guard now assert the mirror owner itself carries
exact `usize` storage while the small-alloc/release proofs continue to validate
the already-landed source-owner counters. The terminal total helpers stay as
derived arithmetic over those exact counters and do not introduce new stored
numeric seams.

## Stop Line

This row does not migrate:

- any `HakoAllocObjectLifecycleAllocResult` or
  `HakoAllocObjectLifecycleReleaseResult` field beyond the already-landed
  source-owner counters;
- any `last_*`, `last_reason`, or `last_ok` field;
- any `HakoAllocObjectLifecycleAlignmentResult` field;
- any `HakoAllocObjectLifecycleReallocResult` field;
- page/block identity payloads, pointer-like fields, totals helper storage,
  unrelated lifecycle observer owners, OSVM/bin/provider/hook seams, TLS,
  atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_release_failfast_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
