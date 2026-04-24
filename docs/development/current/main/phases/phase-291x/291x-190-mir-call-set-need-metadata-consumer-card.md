---
Status: Landed
Date: 2026-04-25
Scope: Make MIR-call need-policy consume direct ArraySet/MapSet CoreMethod metadata before legacy surface fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-189-mir-call-route-policy-metadata-consumer-preflight-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# 291x-190 MIR-Call Set Need Metadata Consumer Card

## Goal

Feed MIR-owned direct set metadata into pure compile need-policy before legacy
box/method surface fallback:

```text
generic_method.set + core_method.op=ArraySet + route_kind=array_store_any
  -> HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET

generic_method.set + core_method.op=MapSet + route_kind=map_store_any
  -> HAKO_LLVMC_MIR_CALL_NEED_MAP_SET
```

This is the missing seam identified by the rejected `291x-187` prune probe.

## Boundary

- Do not remove the MIR-call `set` method surface row in this card.
- Do not change helper symbols or lowering.
- Do not change RuntimeDataBox.set metadata-absent fallback.
- Do not accept mismatched route metadata.
- Keep legacy need-policy fallback when metadata is missing or invalid.

## Implementation

- Move the shared CoreMethod metadata reader include before MIR-call
  route/need-policy includes.
- Add a metadata-first need-policy reader keyed by `block + instruction_index`.
- Validate `route_id`, `core_method.op`, `proof`, `lowering_tier`, and
  `route_kind`.
- Use the metadata-derived need kind before falling back to
  `classify_mir_call_method_need_kind(...)`.

## Result

Direct ArraySet/MapSet pure compile need flags no longer depend solely on the
method-name surface classifier. A later card may retry the MIR-call `set`
surface row prune with direct Array/Map pure boundary checks.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
