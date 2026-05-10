---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M46 hako_alloc production facade boundary
---

# 293x-098 M46 Hako Alloc Production Facade Boundary

## Decision

`M46 hako_alloc production facade boundary` is live-narrow.

M46 creates the production-facing allocator facade under `hako_alloc`:

```text
lang/src/hako_alloc/memory/allocator_facade_box.hako
HakoAllocProductionFacade
```

This facade is a boundary name and public seam only. It delegates to the
existing `HakoAllocHeap` page/free-list policy-state row. It does not replace
the process allocator and does not claim native allocator fast-path ownership.

## Owned

- `HakoAllocProductionFacade.allocate(size)`
- `HakoAllocProductionFacade.release(handle)` returning scalar status `1/0`
- facade-level accounting for accepted allocation calls, accepted releases, and
  rejected operations.
- proof app:
  `apps/hako-alloc-production-facade-proof`
- guard:
  `tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh`

## Not Owned

- New MIR route rows.
- New NyRT exports.
- New `.inc` route emit behavior.
- Backend allocator replacement hook.
- Pointer `fetch_add`.
- Native pointer attrs.
- OS VM page-source policy.
- Remote-free policy.
- Local page policy widening beyond the existing `HakoAllocHeap`.

## Gate

```bash
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the facade module is exported from `selfhost.hako_alloc`;
- the proof app routes through `HakoAllocProductionFacade`;
- pure-first EXE output proves basic allocate/release/reject behavior;
- `.inc` does not branch on the app or facade name;
- pointer `fetch_add` and native pointer attrs remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_hako_alloc_production_facade_exe_guard.sh` passes.
