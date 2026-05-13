# hako-alloc-bounded-purge-decommit-scheduler-proof

Purpose: M212 proof for the bounded purge/decommit scheduler small path.

The app drives a small OSVM-backed heap through
`HakoAllocBoundedPurgeDecommitScheduler`.
The scheduler observes M207 lifecycle facts, classifies them through M211, and
delegates the first eligible page to the M199 state-aware decommit guard.

It intentionally avoids:

- unbounded scans
- more than one candidate attempt per invocation
- direct M197/M195/M196 calls
- direct page-source calls
- recommit
- unreserve or OSVM release
- provider activation, hooks, or process allocator replacement

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_bounded_purge_decommit_scheduler_guard.sh
```

