---
Status: Landed
Date: 2026-04-25
Scope: Prune the MIR-call route-policy `push` method-surface mirror after ArrayPush metadata carriers cover representative boundaries.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-204-push-emit-kind-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-205 Push Route-Surface Mirror Prune Card

## Goal

Remove the MIR-call route-policy method-name classifier row for `push`:

```c
if (!strcmp(mname, "push")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_PUSH;
```

Array push route-state should come from `generic_method_routes` CoreMethod
metadata.

## Boundary

- Remove only the `push` method-surface classifier row.
- Do not remove enum/fallback branch skeletons in this card.
- Do not prune receiver-surface rows.

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

The MIR-call route-policy `push` method-surface classifier row is gone, and the
no-growth baseline dropped from `classifiers=21 rows=21` to
`classifiers=20 rows=20`.
