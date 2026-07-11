---
Status: Landed
Date: 2026-05-23
Scope: select the object-lifecycle facade stats snapshot mirror exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako
  - lang/src/hako_alloc/memory/object_lifecycle_facade_stats_box.hako
  - apps/mimalloc-facade-stats-snapshot-proof/main.hako
  - tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
---

# 294x-78 Hako Alloc Usize Object Lifecycle Facade Stats Snapshot Selection

## Decision

Select the downstream snapshot mirror owner in
`object_lifecycle_facade_stats_box.hako` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-101`.

Chosen fields:

- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_attempt_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_failure_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_reusable_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.alloc_active_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.release_success_count`
- `HakoAllocObjectLifecycleFacadeStatsSnapshot.release_failure_count`

These seven fields are a narrow mirror-only owner fed directly by the already
landed `HAKO-ALLOC-USIZE-FIELD-GROUP-099` facade result counters. They are
monotonic, non-negative, and consumed by the existing stats-snapshot proof
without widening page selection, lifecycle result, alignment, or realloc
contracts.

## Stop Line

The follow-on row must not migrate:

- any `HakoAllocObjectLifecycleAllocResult` or
  `HakoAllocObjectLifecycleReleaseResult` field beyond the already-landed
  source counters;
- any `last_*`, `last_reason`, or `last_ok` field;
- any `HakoAllocObjectLifecycleAlignmentResult` field;
- any `HakoAllocObjectLifecycleReallocResult` field;
- stats terminal helper methods, page/block identity payloads, pointer-like
  fields, unrelated lifecycle observer owners, OSVM/bin/provider/hook seams,
  TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
