---
Status: Rejected
Date: 2026-04-25
Scope: Retry deleting the MIR-call route-policy `set` method surface row after the Set need-policy metadata consumer landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-190-mir-call-set-need-metadata-consumer-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-191 MIR-Call Set Surface Prune Retry Card

## Goal

Retry the `set` method surface prune after `291x-190` made direct
ArraySet/MapSet metadata feed MIR-call need-policy.

## Probe

Temporarily removed:

```c
if (!strcmp(mname, "set")) {
  return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET;
}
```

The enum value was kept because need-policy still names it for fallback.

## Evidence

Passing probe checks:

```text
build_hako_llvmc_ffi:
  PASS

phase29x_runtime_data_dispatch_llvm_e2e_vm:
  PASS

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

After restoring the `set` row and rebuilding, both direct Array and Map pure
canaries returned to green.

## Decision

Reject this prune again.

The need-policy metadata consumer is correct but not sufficient: direct pure
Array/Map set boundary fixtures still include metadata-absent routes that rely
on the MIR-call method surface classifier for helper needs. Future cleanup must
first add or prove coverage for metadata-absent direct set boundaries.

## Result

- Keep `classify_mir_call_method_surface(... "set")`.
- Keep `291x-190` metadata-first need consumer.
- Do not retry this prune until metadata-absent direct Array/Map set boundary
  coverage is resolved.

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
