---
Status: Landed
Date: 2026-04-25
Scope: Make MIR-call route-state selection consume CoreMethod route metadata before legacy box/method surface classifiers.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-188-remaining-inc-mirror-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-189-mir-call-route-policy-metadata-consumer-preflight-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
---

# 291x-196 MIR-Call Route-State Metadata Consumer Card

## Goal

Move `GenericMethodRouteState` selection toward MIR-owned route metadata:

```text
generic_method_routes[block + instruction_index]
  -> core_method.op + route_kind
  -> GenericMethodRouteState flags
  -> legacy box/method surface classifier only when metadata is absent
```

This is the route-state half of the seam described in `291x-189`.

## Boundary

- Do not remove route-policy allowlist rows in this card.
- Do not change helper symbols or lowering.
- Do not change RuntimeData fallback behavior.
- Accept only manifest-proven CoreMethod metadata with the expected
  lowering-tier and route-kind pair.
- Fall back to existing box/method classifiers when metadata is absent or
  unsupported.

## Implementation

- Add a metadata reader inside `hako_llvmc_ffi_mir_call_route_policy.inc`.
- Map validated route metadata to existing route-state kinds:
  - `MapGet + runtime_data_load_any` -> `runtime_map_get`
  - `MapHas + map_contains_*` -> `runtime_map_has`
  - `MapLen + map_entry_count` -> `runtime_map_size`
  - `ArrayLen + array_slot_len` -> `runtime_array_len`
  - `ArrayPush + array_append_any` -> `runtime_array_push`
  - `StringLen + string_len` -> `runtime_string`
  - `StringSubstring + string_substring` -> `runtime_string`
- Keep set routes metadata-owned by the existing emit-kind / storage-route /
  need-policy consumers; they do not need route-state flags in this card.

## Result

Metadata-bearing generic-method routes can now populate route-state flags
without reclassifying backend-local box and method strings. Remaining
route-policy rows stay as fallback until metadata-absent RuntimeData/direct
boundary coverage is either converted or explicitly retained.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
