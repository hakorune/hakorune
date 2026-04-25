---
Status: Landed
Date: 2026-04-26
Scope: Prune the generic_method.set method-name emit-kind fallback while keeping RuntimeDataBox set-route pinned.
Related:
  - docs/development/current/main/phases/phase-291x/291x-179-core-method-set-emit-kind-metadata-card.md
  - docs/development/current/main/phases/phase-291x/291x-180-set-emit-kind-mirror-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-185-runtime-data-set-fallback-contract-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-264 Set Emit-Kind Fallback Prune Card

## Goal

Retry the legacy `set` emit-kind prune in the current post-cleanup state.

The target row was:

```c
if (mname && !strcmp(mname, "set")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_SET;
```

`Set` emit-kind selection should now come from `generic_method.set`
CoreMethod metadata for `ArraySet` / `MapSet`.

## Probe

Temporarily removed the `mname == "set"` emit-kind row and ran:

```bash
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
```

Result:

```text
PASS phase29x_runtime_data_dispatch_llvm_e2e_vm
```

This differs from the earlier rejected `291x-180` state; current metadata
coverage is sufficient for the active RuntimeData dispatch route.

## RuntimeData Set-Route Probe

Also probed deleting:

```c
if (bname && !strcmp(bname, "RuntimeDataBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY;
}
```

The RuntimeData dispatch smoke passed, but the dedicated policy mirror guard
correctly failed:

```text
generic-method-set-policy-mirror-guard:
  ERROR: C set route consumers no longer cover expected route enums:
  ['HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY']
```

Decision: restore the RuntimeDataBox set-route row. Removing it requires a
separate SSOT/guard update or RuntimeData route retirement decision, not a
method-name emit-kind cleanup.

## Change

- Removed the `set` method-name emit-kind fallback.
- Removed the corresponding no-growth allowlist row.
- Kept the RuntimeDataBox set-route fallback and allowlist row.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
```

Result after this card:

```text
core-method-contract-inc-no-growth-guard: ok classifiers=8 rows=8
generic-method-set-policy-mirror-guard: ok routes=5 demands=3
```

## Follow-Up

The remaining RuntimeDataBox set-route row is a facade compatibility sentinel.
Do not remove it by analogy with the method-name prune; it needs a dedicated
RuntimeData set route retirement / metadata card.

