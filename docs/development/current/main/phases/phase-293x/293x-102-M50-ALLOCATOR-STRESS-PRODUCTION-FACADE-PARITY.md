---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M50 allocator stress production-facade parity
---

# 293x-102 M50 Allocator Stress Production-Facade Parity

## Decision

`M50 allocator stress production-facade parity` is live-narrow.

M50 adds a production-facade variant of the existing allocator stress shape:

```text
apps/hako-alloc-production-facade-stress
```

The existing `apps/allocator-stress` app remains regression coverage over the
lower page/free-list seam. The new app proves that the same small/medium
saturation, release, reuse, oversize reject, double-free reject, and accounting
shape is reachable through `HakoAllocProductionFacade`.

## Owned

- production-facade stress proof app.
- thin page-stat accessors on `HakoAllocProductionFacade`.
- guard:
  `tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh`
- docs/taskboard/current pointers for M50.

## Not Owned

- Replacing the process allocator.
- Repointing or deleting `apps/allocator-stress`.
- New MIR route rows.
- New NyRT exports.
- Pointer `fetch_add`.
- Native pointer attrs.
- OSVM unreserve/release rows.
- App-specific `.inc` route matching.

## Gate

```bash
bash tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the stress app calls allocator operations through `HakoAllocProductionFacade`;
- the stress app does not call `HakoAllocHeap` / `HakoAllocPage` directly;
- the existing `apps/allocator-stress` lower-seam regression app remains present;
- facade methods still delegate local allocation/release to `HakoAllocHeap`;
- expected stress accounting matches the existing allocator-stress shape;
- `.inc` does not branch on the app or facade name;
- pointer `fetch_add` and native pointer attrs remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_hako_alloc_production_facade_stress_exe_guard.sh` passes.
