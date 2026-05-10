# hako-alloc-production-facade-proof

Purpose: M46 proof for the `hako_alloc` production-facing allocator facade
boundary.

The app calls `HakoAllocProductionFacade`, which delegates to the existing
`HakoAllocHeap` page/free-list policy-state row. This fixture proves the public
facade name and routing seam only.
`release(handle)` returns scalar status `1/0` so the facade stays in the current
pure-first i64 status lane.

It intentionally avoids:

- backend allocator replacement hooks
- new MIR route rows
- new NyRT exports
- direct `.inc` app/facade matchers
- pointer `fetch_add`
- native pointer attrs
- OS VM page-source ownership
- remote-free policy

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
```
