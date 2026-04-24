---
Status: Landed
Date: 2026-04-25
Scope: Make generic-method `set` storage-route selection prefer valid MIR route_kind metadata before legacy fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-181-set-storage-route-metadata-preflight-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
---

# 291x-182 CoreMethod Set Storage-Route Metadata Card

## Goal

Consume direct ArrayBox/MapBox set route metadata for the existing storage-route
enum:

```text
generic_method.set + core_method.op=MapSet + route_kind=map_store_any
  -> HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY

generic_method.set + core_method.op=ArraySet + route_kind=array_store_any
  -> HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_{I64,STRING,ANY}
```

This moves another decision seam from method/box-name discovery toward MIR-owned
metadata without changing lowering.

## Boundary

- Keep `classify_generic_method_set_route(...)` as fallback.
- Keep the generic `set` emit-kind mirror row.
- Keep `RuntimeDataBox.set` metadata-absent.
- Preserve ArrayBox value-shape discrimination:
  `array_store_any` chooses I64, String, or Any using the existing value checks.
- Reject mismatched metadata such as `ArraySet + map_store_any` or
  `MapSet + array_store_any` by falling back.
- Do not change helper symbols, ABI calls, publication behavior, or hot
  lowering.

## Implementation

- Add a metadata-first set-route reader keyed by `block + instruction_index`.
- Accept only:
  - `route_id=generic_method.set`
  - `core_method.op=ArraySet|MapSet`
  - `proof=core_method_contract_manifest`
  - `lowering_tier=cold_fallback`
  - `route_kind=array_store_any|map_store_any`
- Store the selected route in `GenericMethodEmitPlan`.
- Reuse the existing route if present; otherwise call
  `classify_generic_method_set_route(...)`.

## Result

Direct ArrayBox/MapBox set lowering now uses MIR CoreMethod route metadata to
choose the storage-route family first. Metadata-absent RuntimeData and invalid
metadata still use the legacy fallback.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
