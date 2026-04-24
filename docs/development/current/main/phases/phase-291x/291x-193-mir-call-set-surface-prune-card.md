---
Status: Landed
Date: 2026-04-25
Scope: Delete the MIR-call route-policy `set` method surface mirror row after direct set fixtures became metadata-bearing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-190-mir-call-set-need-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-192-direct-set-boundary-metadata-fixtures-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-193 MIR-Call Set Surface Prune Card

## Goal

Remove the MIR-call route-policy method-name mirror row for `set`:

```c
if (!strcmp(mname, "set")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET;
```

`291x-192` made the direct Array/Map pure-boundary set fixtures carry
`generic_method.set` CoreMethod metadata, so need-policy no longer has to
discover direct set needs through this method-surface row.

## Boundary

- Keep the `HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET` enum value because
  need-policy fallback still names it.
- Do not remove generic-method `set` emit-kind fallback.
- Do not remove `classify_generic_method_set_route(...)` MapBox or
  RuntimeDataBox fallback rows.
- Do not change RuntimeDataBox.set behavior or lowering.

## Implementation

- Delete only the `set` string row from
  `classify_mir_call_method_surface(...)`.
- Remove the matching no-growth allowlist row.
- Rely on `291x-190` metadata-first need-policy for direct ArraySet/MapSet
  pure-boundary payloads.

## Result

The MIR-call route-policy method surface classifier no longer carries the
direct `set` method-name mirror. Remaining `set` debt is limited to the
generic-method emit-kind and storage-route fallback rows, which are tracked by
their own fallback contracts.

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
