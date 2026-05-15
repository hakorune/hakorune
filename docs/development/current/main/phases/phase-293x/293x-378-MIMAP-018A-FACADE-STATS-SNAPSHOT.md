# 293x-378 MIMAP-018A Facade Stats Snapshot

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-018A` integrates a read-only stats snapshot over the object-lifecycle
facade after allocation/release observer counters were stabilized by
`MIMAP-FACADE-CLEAN-001`.

## Scope

- Add `object_lifecycle_facade_stats_box.hako`.
- Add release success/failure counters to the facade release result capsule.
- Add `HakoAllocObjectLifecycleFacade.objectLifecycleStatsSnapshot()`.
- Keep the snapshot read-only and sourced only from existing facade observer
  counters.

## Non-goals

- No purge, reclaim, decommit, or page-source policy.
- No page-map lookup or arbitrary pointer-to-page resolution.
- No allocator behavior change.
- No provider activation, hooks, host allocator replacement, or backend
  shortcut.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Next

`MIMAP-019A` may now start as the next allocator row because stats and lifecycle
observers are stable.
