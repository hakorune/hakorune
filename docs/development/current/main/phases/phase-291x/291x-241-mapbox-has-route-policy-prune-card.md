---
Status: Landed
Date: 2026-04-25
Scope: Delete the generic-method has policy direct MapBox mirror row after direct MapHas boundaries became metadata-bearing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-211-runtime-data-has-compat-contract-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-223-generic-has-route-no-growth-guard-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
---

# 291x-241 MapBox Has Route Policy Prune Card

## Goal

Remove the direct `MapBox` fallback branch from
`classify_generic_method_has_route(...)`:

```c
if (bname && !strcmp(bname, "MapBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY;
}
```

Direct `MapBox.has(...)` route selection is already carried by
`generic_method.has` route metadata in the active MapHas canaries. This card
stops rediscovering that route from backend-local `box_name` mirrors.

## Boundary

- Delete only the direct `MapBox` branch from the legacy has route classifier.
- Keep the `ArrayBox` and `RuntimeDataBox` fallback branches.
- Do not introduce `ArrayHas` or widen RuntimeDataBox.has semantics.
- Do not change helper symbols, map lookup fusion, or MIR-call route policy.

## Implementation

- Remove the `MapBox` string branch from
  `lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc`.
- Remove the matching no-growth allowlist row.

## Result

`generic_method.has` no longer selects direct `MapBox` probe routes from a
backend-local `bname == "MapBox"` mirror. Direct `MapBox.has(...)` now depends
on existing route metadata, while Array-origin and RuntimeData compatibility
fallbacks stay pinned by their own contracts.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
