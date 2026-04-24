---
Status: Landed
Date: 2026-04-25
Scope: Delete the direct `ArrayBox` branch from the legacy `set` storage-route classifier after metadata-first route selection.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-182-core-method-set-storage-route-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-183 Set Array Storage-Route Mirror Prune Card

## Goal

Prune the direct ArrayBox storage-route mirror branch only after the set
storage-route metadata consumer landed:

```c
if (bname && !strcmp(bname, "ArrayBox")) {
  arrayish = 1;
}
```

The direct MapBox branch is not pruned in this card.

## Probe

First probe removed both direct `MapBox` and `ArrayBox` branches from
`classify_generic_method_set_route(...)`.

That failed:

```text
generic-method-set-policy-mirror-guard:
  ERROR: classify_generic_method_set_route no longer returns expected route enums:
  ['HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY']

s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm:
  FAIL ny-llvmc boundary emit rc=1
```

Then the direct `MapBox` branch was restored and only the direct `ArrayBox`
branch stayed removed. That passed the focused boundary checks.

## Implementation

- Remove the direct `ArrayBox` box-name branch from
  `classify_generic_method_set_route(...)`.
- Keep metadata-first set route selection from `291x-182`.
- Keep array-family fallback through receiver origin / existing runtime array
  route flags.
- Keep direct `MapBox` and `RuntimeDataBox` fallback branches.
- Remove the now-pruned `ArrayBox` set-route allowlist row.

## Result

The no-growth baseline drops by one classifier row. ArrayBox set storage-route
selection is now carried by metadata-first route selection or existing array
origin/route fallback, not by a direct `ArrayBox` branch in
`classify_generic_method_set_route(...)`.

The direct `MapBox` branch remains required until metadata-absent map set
boundary coverage exists.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
