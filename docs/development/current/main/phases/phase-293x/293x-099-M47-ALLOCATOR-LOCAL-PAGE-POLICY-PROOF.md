---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M47 allocator local page policy proof
---

# 293x-099 M47 Allocator Local Page Policy Proof

## Decision

`M47 allocator local page policy proof` is live-narrow.

M47 proves that the production-facing `HakoAllocProductionFacade` can own the
public allocator seam while local page allocation/free policy remains delegated
to the existing `HakoAllocHeap` page/free-list state.

The proof app:

```text
apps/hako-alloc-local-page-policy-proof
```

validates small/medium allocation, oversize rejection, release success,
double-free rejection, and local reuse/accounting shape through the facade.

## Owned

- production facade local page policy proof app.
- guard:
  `tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh`
- docs/taskboard/current pointers for M47.

## Not Owned

- Remote-free policy.
- OS VM page-source ownership.
- Backend allocator replacement hook.
- New MIR route rows.
- New NyRT exports.
- New `.inc` route emit behavior.
- Pointer `fetch_add`.
- Native pointer attrs.
- Native layout / `repr(C)` allocator metadata.

## Gate

```bash
bash tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the proof app uses `HakoAllocProductionFacade`;
- pure-first EXE output proves local page accounting through the facade;
- `.inc` does not branch on the app or facade name;
- pointer `fetch_add` and native pointer attrs remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_hako_alloc_local_page_policy_exe_guard.sh` passes.
