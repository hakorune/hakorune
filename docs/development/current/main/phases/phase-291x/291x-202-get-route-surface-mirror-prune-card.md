---
Status: Landed
Date: 2026-04-25
Scope: Prune the MIR-call route-policy `get` method-surface mirror after GET emit-kind selection moved to CoreMethod metadata.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-201-get-emit-kind-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-202 Get Route-Surface Mirror Prune Card

## Goal

Remove the MIR-call route-policy method-name classifier row for `get`:

```c
if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET;
```

GET route-state selection should be carried by `generic_method_routes`
CoreMethod metadata, not rediscovered from method names in `.inc`.

## Boundary

- Remove only the `get` method-surface classifier row.
- Do not remove the enum variant or old fallback branch structure in this card;
  those are dead-code cleanup candidates after all method-surface rows are gone.
- Do not prune receiver-surface rows.
- Do not add new fallback behavior.

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

The MIR-call route-policy `get` method-surface classifier row is gone, and the
no-growth baseline dropped from `classifiers=23 rows=23` to
`classifiers=22 rows=22`. The enum/fallback branch skeleton remains until more
method-surface rows are pruned.
