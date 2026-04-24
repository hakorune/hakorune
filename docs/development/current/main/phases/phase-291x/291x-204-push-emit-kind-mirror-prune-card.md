---
Status: Landed
Date: 2026-04-25
Scope: Prune the generic-method emit-kind `push` method-name mirror after direct and RuntimeData ArrayPush carriers landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-203-runtime-data-array-push-carrier-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-204 Push Emit-Kind Mirror Prune Card

## Goal

Remove this transitional mirror row:

```c
if (mname && !strcmp(mname, "push")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH;
```

`push` emit selection should come from MIR-owned `ArrayPush` CoreMethod
metadata, not from a C-local method-name classifier.

## Boundary

- Remove only the generic emit-kind `push` method mirror row.
- Do not prune MIR-call route-policy `METHOD_SURFACE_PUSH` in this card.
- Do not add new Array method semantics.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The legacy `push` method-name emit-kind mirror row is gone, and the no-growth
baseline dropped from `classifiers=22 rows=22` to `classifiers=21 rows=21`.
MIR-call route-policy `METHOD_SURFACE_PUSH` remains for a separate prune card.
