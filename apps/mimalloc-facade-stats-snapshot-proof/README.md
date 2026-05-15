# mimalloc facade stats snapshot proof

This proof belongs to `MIMAP-018A`.

It proves that `HakoAllocObjectLifecycleFacade.objectLifecycleStatsSnapshot()`
returns a read-only snapshot of existing allocation/release observer counters
without adding purge policy, page-map lookup, provider hooks, or backend
shortcuts.
