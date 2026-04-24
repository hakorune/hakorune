---
Status: Rejected
Date: 2026-04-25
Scope: Probe deleting the MIR-call route-policy `set` method surface mirror row.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-186-mir-call-set-surface-metadata-preflight-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-187 MIR-Call Set Surface Prune Card

## Goal

Test whether this MIR-call route-policy mirror row can be removed after set
emit-kind and storage-route metadata-first selection landed:

```c
if (!strcmp(mname, "set")) {
  return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET;
}
```

## Probe

Temporarily removed only the `set` row from
`classify_mir_call_method_surface(...)`. The enum value was kept because
`hako_llvmc_ffi_mir_call_need_policy.inc` still references
`HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET`.

## Evidence

Passing probe check:

```text
core-method-contract-inc-no-growth-guard:
  ok classifiers=25 rows=25
```

Failing boundary checks:

```text
s3_link_run_llvmcapi_pure_array_set_get_canary_vm:
  FAIL ny-llvmc boundary emit rc=1

s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm:
  FAIL ny-llvmc boundary emit rc=1
```

RuntimeData dispatch stayed green during the probe, but direct Array/Map pure
boundaries still require this route-policy surface row.

After restoring the row and rebuilding the C shim, both direct Array and Map
pure boundary canaries returned to green.

## Decision

Reject this prune.

The generic-method metadata consumer is not enough to delete the MIR-call
route-policy `set` method surface row. The deletion condition remains:

```text
replace-with-core-method-op-id-and-metadata-absent-set-mutating-boundary-contract
```

## Result

- Keep `classify_mir_call_method_surface(... "set")`.
- Keep the enum value for `HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET`.
- Treat future cleanup as a separate route-policy metadata contract, not as a
  side effect of generic-method set metadata.

## Acceptance

Restored-state checks:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
