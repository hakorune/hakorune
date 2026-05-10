---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M48 allocator remote-free policy proof
---

# 293x-100 M48 Allocator Remote-Free Policy Proof

## Decision

`M48 allocator remote-free policy proof` is live-narrow.

M48 moves the M43 remote-free retry-loop shape behind the production-facing
`HakoAllocProductionFacade`.

The production boundary is:

```text
HakoAllocProductionFacade
  -> HakoAllocRemoteFreePolicy
  -> existing hako.atomic pointer store/load/CAS substrate routes
```

The proof app:

```text
apps/hako-alloc-remote-free-policy-proof
```

validates that facade methods can initialize a remote-free head, push two blocks
through a bounded CAS retry loop, expose peek helpers for proof, and preserve the
existing local allocate/release facade accounting.

## Owned

- `lang/src/hako_alloc/memory/remote_free_policy_box.hako`
- remote-free methods on `HakoAllocProductionFacade`
- proof app:
  `apps/hako-alloc-remote-free-policy-proof`
- guard:
  `tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh`
- docs/taskboard/current pointers for M48.

## Not Owned

- Pointer `fetch_add`.
- New pointer atomic route rows.
- New hako.mem or hako.atomic NyRT exports.
- Native pointer attrs.
- OS VM page-source ownership.
- Backend allocator replacement hook.
- Native layout / `repr(C)` allocator metadata.
- App-specific `.inc` route matching.

## Gate

```bash
bash tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the app calls remote-free operations through `HakoAllocProductionFacade`;
- `HakoAllocProductionFacade` delegates to `HakoAllocRemoteFreePolicy`;
- `HakoAllocRemoteFreePolicy` owns the existing pointer store/load/CAS route
  composition;
- `.inc` does not branch on the app, facade, or policy name;
- pointer `fetch_add` and native pointer attrs remain inactive.

## Result

Result on 2026-05-10:
`k2_wide_hako_alloc_remote_free_policy_exe_guard.sh` passes.
