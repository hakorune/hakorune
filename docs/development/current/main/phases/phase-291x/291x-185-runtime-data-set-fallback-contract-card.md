---
Status: Landed
Date: 2026-04-25
Scope: Pin RuntimeDataBox.set fallback requirements before any further `set` mirror pruning.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-180-set-emit-kind-mirror-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-184-set-map-storage-route-fallback-contract-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-185 RuntimeData Set Fallback Contract Card

## Goal

Make the remaining RuntimeData set fallback explicit before attempting any
further `set` mirror pruning.

The currently required fallback is:

```c
if (bname && !strcmp(bname, "RuntimeDataBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY;
}
```

## Evidence

`291x-180` temporarily removed the legacy generic `set` emit-kind row after
direct ArraySet/MapSet metadata-first selection landed. That failed the
RuntimeData dispatch boundary:

```text
phase29x_runtime_data_dispatch_llvm_e2e_vm:
  failed to compile MIR JSON via selected driver
  unsupported pure shape for current backend recipe
```

Restoring the legacy row returned the RuntimeData dispatch smoke to green.

`291x-183` then pruned only the direct ArrayBox storage-route branch. RuntimeData
was intentionally left untouched and remains covered by the same RuntimeData
dispatch smoke.

## Decision

Keep the direct `RuntimeDataBox` set storage-route fallback branch.

`RuntimeDataBox.set` is metadata-absent by design in the current carrier slice.
It cannot be deleted by direct ArraySet/MapSet metadata alone. Future deletion
requires either:

- a RuntimeData set contract with explicit MIR metadata coverage, or
- a proven route where metadata-absent RuntimeData set no longer reaches this
  C-side fallback.

## Implementation

- Keep the RuntimeData set branch in
  `classify_generic_method_set_route(...)`.
- Tighten the no-growth allowlist deletion condition for the RuntimeData set
  branch so future cleanup must prove the metadata-absent RuntimeData boundary.

## Result

The remaining `set` storage-route mirror rows are now split:

- `ArrayBox`: pruned in `291x-183`.
- `MapBox`: retained by `291x-184` until metadata-absent map set coverage exists.
- `RuntimeDataBox`: retained by this card until RuntimeData set coverage exists.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
