# hako-alloc-lifecycle-stats-observer-surface-proof

Purpose: M209 proof for the `hako_alloc` lifecycle stats observer surface.

The app drives the existing M207 lifecycle observer through the M208 heap reuse
priority policy, then snapshots the observer and policy counters through
`HakoAllocLifecycleStatsObserverSurface`.

It intentionally avoids:

- allocator behavior changes
- mutable allocator options
- environment variables or CLI toggles
- new decommit/recommit execution seams
- provider activation, hooks, or process allocator replacement
- compiler/runtime packed record vocabulary

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_lifecycle_stats_observer_surface_guard.sh
```
