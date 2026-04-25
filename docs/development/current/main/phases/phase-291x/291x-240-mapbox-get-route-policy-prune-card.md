---
Status: Landed
Date: 2026-04-25
Scope: Delete the generic-method get policy direct MapBox mirror row after direct MapGet boundaries became metadata-bearing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
  - tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
---

# 291x-240 MapBox Get Route Policy Prune Card

## Goal

Remove the last direct `MapBox` fallback branch from
`classify_generic_method_get_route(...)`:

```c
if (bname && !strcmp(bname, "MapBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY;
}
```

Direct `MapBox.get(...)` route selection is already carried by
`generic_method.get` CoreMethod metadata in the active pure-boundary canaries.
This card stops rediscovering that route from backend-local `box_name` mirrors.

## Boundary

- Delete only the direct `MapBox` branch from the legacy get route classifier.
- Keep the direct `ArrayBox` branch and the `RuntimeDataBox` fallback branch.
- Do not change get helper symbols, lowering, or MapLookup fusion handling.
- Do not add new fallback behavior.

## Implementation

- Remove the `MapBox` string branch from
  `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc`.
- Remove the matching no-growth allowlist row.

## Result

`generic_method.get` no longer selects `map_load_any` from a backend-local
`bname == "MapBox"` mirror row. Direct `MapBox.get(...)` now depends on the
existing CoreMethod route metadata contract, while `ArrayBox` and
`RuntimeDataBox` get fallbacks remain pinned by their own contracts.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
