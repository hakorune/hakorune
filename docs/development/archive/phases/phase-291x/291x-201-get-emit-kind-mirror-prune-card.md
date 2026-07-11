---
Status: Landed
Date: 2026-04-25
Scope: Prune the legacy generic-method emit-kind `get` method-name mirror after direct and RuntimeData get carriers landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-199-direct-get-coremethod-carrier-card.md
  - docs/development/current/main/phases/phase-291x/291x-200-runtime-data-array-get-carrier-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-201 Get Emit-Kind Mirror Prune Card

## Goal

Remove this transitional mirror row:

```c
if (mname && !strcmp(mname, "get")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_GET;
```

`get` emit selection should come from MIR-owned CoreMethod metadata through
`classify_generic_method_emit_kind_from_core_method_metadata(...)`, not from a
C-local method-name classifier.

## Boundary

- Remove only the generic emit-kind `get` method mirror row.
- Do not prune MIR-call route-policy `METHOD_SURFACE_GET` in this card.
- Do not prune `ArrayBox` / `MapBox` receiver surface rows in this card.
- Do not add new lowering behavior.
- Do not add metadata-less fallback branches.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The legacy `get` method-name emit-kind mirror row is gone, and the no-growth
baseline dropped from `classifiers=24 rows=24` to `classifiers=23 rows=23`.
MIR-call route-policy `METHOD_SURFACE_GET` remains for a separate route-state
prune card.
