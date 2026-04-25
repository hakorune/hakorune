---
Status: Landed
Date: 2026-04-26
Scope: Prune the RuntimeDataBox set storage-route fallback and retire the RuntimeDataStoreAny route vocabulary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-185-runtime-data-set-fallback-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-264-set-emit-kind-fallback-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-265-runtime-data-get-route-fallback-prune-card.md
  - lang/src/runtime/collections/method_policy_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
  - tools/checks/generic_method_set_policy_mirror_guard.sh
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-266 RuntimeData Set Route Fallback Prune Card

## Goal

Remove the remaining `RuntimeDataBox` set storage-route fallback:

```c
if (bname && !strcmp(bname, "RuntimeDataBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY;
}
```

After `291x-265`, the active RuntimeData dispatch E2E fixture has explicit
`generic_method.set` / `MapSet` metadata for its Map-origin set boundary. Array
origin RuntimeData set paths already select the ArrayStore route from receiver
origin / runtime-array plan facts.

## Boundary

- Do not change public `RuntimeDataBox.set` behavior.
- Do not add a replacement method-name or box-name classifier.
- Keep direct ArrayStore / MapStore metadata routes unchanged.
- Keep `nyash.runtime_data.set_hhh` declarations that belong to older pure
  compile support out of scope.

## Implementation

- Removed the `RuntimeDataBox` fallback branch from
  `classify_generic_method_set_route(...)`.
- Removed `RuntimeDataStoreAny` from `CollectionMethodPolicyBox`.
- Removed `HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY` from the
  C route enum and lowering switch.
- Updated `generic_method_set_policy_mirror_guard.sh` so the set route vocabulary
  SSOT no longer expects RuntimeDataStoreAny.
- Removed the RuntimeData set row from the no-growth allowlist.

## Result

The set route vocabulary now contains only active storage routes:

```text
MapStoreAny
ArrayStoreI64
ArrayStoreString
ArrayStoreAny
```

The no-growth guard shrank again:

```text
classifiers=6 rows=6
```

The set policy mirror guard now reports:

```text
ok routes=4 demands=3
```

## Verification

```bash
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
cargo check -q
git diff --check
```

Observed:

```text
PASS phase29x_runtime_data_dispatch_llvm_e2e_vm
PASS phase291x_mapbox_hako_set_multiarg_vm
PASS phase291x_mapbox_hako_write_return_vm
PASS phase137x_direct_emit_array_store_string_contract
PASS phase137x_boundary_array_string_len_insert_mid_source_only_min
PASS phase137x_boundary_array_string_len_piecewise_concat3_source_only_min
PASS s3_link_run_llvmcapi_pure_array_set_get_canary_vm
PASS s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm
core-method-contract-inc-no-growth-guard ok classifiers=6 rows=6
generic-method-set-policy-mirror-guard ok routes=4 demands=3
```

## Next

Remaining no-growth rows are now has/push/MIR-surface only:

```text
classify_generic_method_emit_kind method has
classify_generic_method_has_route box ArrayBox
classify_generic_method_has_route box RuntimeDataBox
classify_generic_method_push_route box RuntimeDataBox
classify_mir_call_receiver_surface box MapBox
classify_mir_call_method_surface method has
```
