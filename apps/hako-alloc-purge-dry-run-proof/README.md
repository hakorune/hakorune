# hako-alloc-purge-dry-run-proof

Purpose: M193 proof for the `hako_alloc` purge/decommit dry-run observer.

The app builds a small modeled OSVM-backed heap shape with page/backing arrays,
observes a live page, releases it until it becomes empty/retired, and observes
the resulting purge candidate through `HakoAllocPurgeDryRunObserver`.

It intentionally avoids:

- OSVM decommit/unreserve/release execution
- OSVM reserve/commit setup calls
- heap/page mutation inside the dry-run observer
- provider activation, hooks, or process allocator replacement

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_purge_dry_run_guard.sh
```
