# mimalloc-page-map-release-proof

M172 proof app. It composes `HakoAllocPageMap.lookup(...)`,
`HakoAllocPageModel.releaseLocal(...)`, and `HakoAllocPageMap.unregister(...)`
through `HakoAllocPageMapReleaseSeam`.

Pointer registration remains owned by `HakoAllocPageMap.register(...)`; this
app uses it only to seed ownership entries before exercising the release seam.
The guard verifies VM behavior and MIR route contracts. Full pure-first EXE
parity is reserved for the later object-return/lowering row.

This row does not implement realloc, aligned allocation, huge allocation,
secure-list hardening, remote-free atomics, byte copy, OSVM release, provider
activation, hooks, or process allocator replacement.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
```
