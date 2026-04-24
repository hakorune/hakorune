---
Status: Landed
Date: 2026-04-25
Scope: Pin the direct `MapBox` set storage-route fallback contract after the partial storage-route prune.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-183-set-array-storage-route-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-184 Set Map Storage-Route Fallback Contract Card

## Goal

Record why the direct `MapBox` branch in
`classify_generic_method_set_route(...)` remains after the ArrayBox branch was
pruned.

```c
if (bname && !strcmp(bname, "MapBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY;
}
```

## Evidence

The combined prune probe removed both direct `MapBox` and direct `ArrayBox`
branches. ArrayBox coverage survived, but MapBox did not:

```text
generic-method-set-policy-mirror-guard:
  ERROR: classify_generic_method_set_route no longer returns expected route enums:
  ['HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY']

s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm:
  FAIL ny-llvmc boundary emit rc=1
```

Restoring only the `MapBox` branch and keeping the `ArrayBox` branch removed
returned the guard and pure map boundary to green.

## Decision

Keep the direct `MapBox` set storage-route branch.

Direct `MapSet` metadata exists, but metadata-absent pure map set boundary
coverage still reaches the legacy storage-route classifier. The deletion
condition remains:

```text
replace-with-core-method-op-route-metadata-and-metadata-absent-map-set-boundary-contract
```

## Result

- `ArrayBox` storage-route mirror branch stays pruned from `291x-183`.
- `MapBox` storage-route mirror branch remains as an explicit fallback.
- Further cleanup must add metadata-absent map set boundary coverage or a
  RuntimeData/set contract before pruning this branch.

## Acceptance

```bash
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
