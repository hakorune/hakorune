---
Status: Rejected
Date: 2026-04-25
Scope: Probe deleting the legacy generic `push` emit-kind mirror row after ArrayPush metadata-first selection.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-175-core-method-push-emit-kind-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-176 Push Emit-Kind Mirror Prune Card

## Goal

Test whether the legacy generic-method `push` emit-kind fallback can be deleted
after `generic_method.push` + `core_method.op=ArrayPush` metadata-first
selection landed.

## Probe

Temporarily removed:

```c
if (mname && !strcmp(mname, "push")) {
  return HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH;
}
```

Then rebuilt `libhako_llvmc_ffi.so` and ran direct ArrayBox / RuntimeData push
boundary smokes.

## Evidence

Passing probe checks:

```text
core-method-contract-inc-no-growth-guard:
  NOTE: 1 allowlist rows no longer have classifiers; prune recommended
  ok classifiers=26 rows=26

phase29x_backend_owner_daily_runtime_data_array_push_min:
  PASS
```

Failing boundary checks:

```text
s3_link_run_llvmcapi_pure_array_get_ret_canary_vm:
  FAIL ny-llvmc boundary emit rc=1

phase29x_runtime_data_dispatch_llvm_e2e_vm:
  failed to compile MIR JSON via selected driver
  unsupported pure shape for current backend recipe
```

After restoring the legacy row and rebuilding the C shim, both failing smokes
returned to green.

## Decision

Reject this prune.

`ArrayPush` metadata is not sufficient to delete the legacy `push` mirror row
because metadata-absent direct ArrayBox and RuntimeData boundary fixtures still
reach the legacy classifier. The deletion condition remains:

```text
replace-with-core-method-op-id-and-metadata-absent-push-mutating-boundary-contract
```

## Result

- Keep the 27-row no-growth baseline.
- Keep `push` metadata-first selection from `291x-175`.
- Keep legacy `push` fallback until metadata-absent mutating boundary coverage
  exists.

## Acceptance

Restored-state checks:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
