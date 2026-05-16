# mimalloc-facade-huge-unreserve-failfast-proof

Purpose: MIMAP-035A proof for facade-level duplicate/stale huge unreserve
diagnostics.

The proof allocates one page-source-backed huge handle, unregisters/decommits
and unreserves it through `HakoAllocObjectLifecycleFacadeHugeUnreserveRoute`,
then records that backing range in the fail-fast owner. Duplicate and stale
unreserve attempts are rejected before a second
`HakoAllocPageSourceUnreserveAdapter` call.

Non-goals:

- recommit / purge scheduler behavior
- remote-free / TLS behavior
- provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`
- backend `.inc` matcher shortcuts

Run:

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_unreserve_failfast_exe_guard.sh
```
