# hako-alloc-production-facade-stress

Purpose: M50 proof that the existing `allocator-stress` shape reaches the
production allocator facade.

This app keeps `apps/allocator-stress` as regression coverage and adds a new
production-facade variant. It validates the same small/medium saturation,
release, reuse, oversize reject, double-free reject, and deterministic
accounting shape through `HakoAllocProductionFacade`.

It intentionally avoids:

- direct `HakoAllocHeap` use in the app
- remote-free policy
- OSVM page-source policy
- backend allocator replacement hooks
- pointer `fetch_add`
- native pointer attrs
- new `.inc` app/facade matchers

Gate:

```bash
bash tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh
```
