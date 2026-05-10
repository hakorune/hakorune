---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M49 allocator OSVM page-source proof
---

# 293x-101 M49 Allocator OSVM Page-Source Proof

## Decision

`M49 allocator OSVM page-source proof` is live-narrow.

M49 moves the existing M25 OSVM reserve/commit/decommit substrate shape behind
the production-facing `HakoAllocProductionFacade`.

The production boundary is:

```text
HakoAllocProductionFacade
  -> HakoAllocPageSourcePolicy
  -> OsVmCoreBox
  -> existing hako.osvm reserve/commit/decommit substrate routes
```

The proof app:

```text
apps/hako-alloc-page-source-policy-proof
```

validates that facade methods can reserve, commit, and decommit one page-sized
range under pure-first EXE while the OSVM metal body remains in substrate/native
keep.

## Owned

- `lang/src/hako_alloc/memory/page_source_policy_box.hako`
- page-source methods on `HakoAllocProductionFacade`
- proof app:
  `apps/hako-alloc-page-source-policy-proof`
- guard:
  `tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh`
- docs/taskboard/current pointers for M49.

## Not Owned

- OS VM unreserve/release rows.
- New hako.osvm route rows.
- New NyRT exports.
- Native pointer attrs.
- Backend allocator replacement hook.
- Native layout / `repr(C)` allocator metadata.
- App-specific `.inc` route matching.

## Gate

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the app calls page-source operations through `HakoAllocProductionFacade`;
- `HakoAllocProductionFacade` delegates to `HakoAllocPageSourcePolicy`;
- `HakoAllocPageSourcePolicy` delegates to `OsVmCoreBox`;
- `OsVmCoreBox` owns the existing reserve/commit/decommit extern route facts;
- `.inc` does not branch on the app, facade, or policy name;
- OSVM unreserve/release rows remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_hako_alloc_page_source_policy_exe_guard.sh` passes.
