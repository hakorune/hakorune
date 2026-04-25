---
Status: Landed
Date: 2026-04-25
Scope: Prune the generic-method get route-policy direct ArrayBox mirror row after direct ArrayGet boundaries are already metadata-bearing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-199-direct-get-coremethod-carrier-card.md
  - docs/development/current/main/phases/phase-291x/291x-200-runtime-data-array-get-carrier-card.md
  - docs/development/current/main/phases/phase-291x/291x-240-mapbox-get-route-policy-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-247 ArrayBox Get Route Policy Prune Card

## Goal

Remove the remaining direct `ArrayBox` fallback branch from
`classify_generic_method_get_route(...)`:

```c
if ((bname && !strcmp(bname, "ArrayBox")) ||
    (plan && (plan->runtime_array_get || plan->runtime_array_string))) {
  return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY;
}
```

Direct `ArrayBox.get(...)` route selection is already carried by
`generic_method.get` CoreMethod metadata in the active direct-boundary
fixtures. This card stops rediscovering that route from backend-local
`box_name` mirrors.

## Boundary

- Delete only the direct `ArrayBox` branch from the legacy get route classifier.
- Keep the `RuntimeDataBox` fallback branch pinned.
- Do not change get helper symbols, lowering, or MapLookup fusion handling.
- Do not add new fallback behavior.

## Implementation

- Remove the `ArrayBox` string branch from
  `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc`.
- Remove the matching no-growth allowlist row.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The direct `ArrayBox` fallback branch was removed from
`classify_generic_method_get_route(...)`, and the no-growth allowlist now only
keeps the pinned `RuntimeDataBox` get fallback. Direct ArrayBox get fixtures
continue to resolve through CoreMethod metadata, so the route-policy mirror is
no longer needed for that path.
