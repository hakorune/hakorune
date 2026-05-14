# 293x-345 MIMAP-011 Facade Lifecycle Route Pilot

Status: landed.
Decision: accepted.

## Goal

Expose the lifecycle-aware page selection policy through `HakoAllocProductionFacade`
and prove the route with LLVM/EXE as the primary acceptance backend.

## Landing shape

- `HakoAllocProductionFacade` owns a `HakoAllocLifecyclePageQueue` field.
- The facade exposes scalar lifecycle selection methods.
- The proof app exercises `decommitted skip -> reusable retired select -> active select -> miss` through the facade.
- VM object-heavy queue/facade retention remains out of scope.

## Guard

```bash
bash tools/checks/k2_wide_mimalloc_facade_lifecycle_route_exe_guard.sh
```

## Explicit non-goals

- no provider activation
- no host allocator replacement
- no allocator hooks
- no OSVM execution
- no page object retention inside queue-like boxes

Next selected row: `MIMAP-012 object-backed lifecycle queue LLVM route pilot`.
