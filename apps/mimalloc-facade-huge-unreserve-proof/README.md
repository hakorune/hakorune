# mimalloc-facade-huge-unreserve-proof

Purpose: MIMAP-034A proof for the facade huge unreserve-after-decommit success
route.

The app allocates one page-source-backed huge handle, unregisters and decommits
it through `HakoAllocObjectLifecycleFacadeHugeDecommitRoute`, then unreserves
the same backing range through `HakoAllocPageSourceUnreserveAdapter`.

The route intentionally keeps duplicate/stale unreserve diagnostics, recommit,
provider activation, hooks, host allocator replacement, and `#[global_allocator]`
closed.

Gate:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_unreserve_exe_guard.sh
```
