---
Status: Landed
Date: 2026-04-25
Scope: Delete the generic-method set storage-route direct MapBox mirror row after MapSet metadata fixture coverage.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-184-set-map-storage-route-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-194-map-set-boundary-metadata-fixtures-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - tools/checks/generic_method_set_policy_mirror_guard.sh
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-195 Set Map Storage-Route Prune Card

## Goal

Remove the direct `MapBox` fallback branch from
`classify_generic_method_set_route(...)`:

```c
if (bname && !strcmp(bname, "MapBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY;
}
```

`MapSet` route selection is now covered by `generic_method_routes` metadata for
the known direct pure-boundary Map set fixtures.

## Boundary

- Do not remove the RuntimeDataBox set storage-route fallback.
- Do not remove generic `set` emit-kind fallback.
- Do not change Map set helper symbols or lowering.
- Do not weaken ArrayStoreString demand checks.

## Implementation

- Delete only the direct `MapBox` branch from the legacy set storage-route
  classifier.
- Remove the matching no-growth allowlist row.
- Update `generic_method_set_policy_mirror_guard.sh` so route coverage may come
  from either the legacy classifier or the CoreMethod metadata route consumer.

## Result

`MapStoreAny` is no longer rediscovered from backend-local `box_name=MapBox` in
the generic set storage-route policy. It is consumed from MapSet metadata by
`hako_llvmc_ffi_generic_method_match.inc`.

The remaining generic set storage-route fallback is `RuntimeDataBox`, which is
still pinned by its own RuntimeData fallback contract.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
