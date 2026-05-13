# hako-alloc-stats-surface-proof

Purpose: M191 proof for the `hako_alloc` production facade stats surface.

The app exercises the existing `HakoAllocProductionFacade` allocation/release
path, then calls `statsSnapshot()` and checks that the returned
`HakoAllocStatsSnapshot` matches the existing facade/page observers.

It intentionally avoids:

- allocator behavior changes
- mutable allocator options
- environment variables or CLI toggles
- purge/decommit execution
- provider activation, hooks, or process allocator replacement
- compiler/runtime packed record vocabulary

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_stats_surface_guard.sh
```
