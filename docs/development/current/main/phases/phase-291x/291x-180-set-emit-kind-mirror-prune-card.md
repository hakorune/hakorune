---
Status: Rejected
Date: 2026-04-25
Scope: Probe deleting the legacy generic `set` emit-kind mirror row after ArraySet/MapSet metadata-first selection.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-179-core-method-set-emit-kind-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-180 Set Emit-Kind Mirror Prune Card

## Goal

Test whether the legacy generic-method `set` emit-kind fallback can be deleted
after `generic_method.set` + `core_method.op=ArraySet/MapSet` metadata-first
selection landed.

## Probe

Temporarily removed:

```c
if (mname && !strcmp(mname, "set")) {
  return HAKO_LLVMC_GENERIC_METHOD_EMIT_SET;
}
```

Then rebuilt `libhako_llvmc_ffi.so` and ran the RuntimeData dispatch boundary
smoke.

## Evidence

Passing probe checks:

```text
core-method-contract-inc-no-growth-guard:
  NOTE: 1 allowlist rows no longer have classifiers; prune recommended
  ok classifiers=26 rows=26

generic-method-set-policy-mirror-guard:
  ok routes=5 demands=3
```

Failing boundary check:

```text
phase29x_runtime_data_dispatch_llvm_e2e_vm:
  failed to compile MIR JSON via selected driver
  unsupported pure shape for current backend recipe
```

After restoring the legacy row and rebuilding the C shim, the failing RuntimeData
dispatch smoke returned to green.

## Decision

Reject this prune.

`ArraySet` / `MapSet` metadata is not sufficient to delete the legacy `set`
mirror row because metadata-absent RuntimeData set boundary fixtures still reach
the legacy classifier. The deletion condition remains:

```text
replace-with-core-method-op-id-and-metadata-absent-set-mutating-boundary-contract
```

## Result

- Keep the 27-row no-growth baseline.
- Keep `set` metadata-first selection from `291x-179`.
- Keep legacy `set` emit-kind fallback until metadata-absent mutating boundary
  coverage exists.
- Treat storage-route cleanup as a separate card; do not mix it with emit-kind
  pruning.

## Acceptance

Restored-state checks:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
