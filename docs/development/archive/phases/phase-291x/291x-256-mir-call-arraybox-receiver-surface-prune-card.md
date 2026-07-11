---
Status: Landed
Date: 2026-04-25
Scope: Prune the dead `mir_call_route_policy` ArrayBox receiver-surface classifier after constructor/birth emit-kind cleanup.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-256 MIR-Call ArrayBox Receiver-Surface Prune Card

## Goal

Remove the `ArrayBox` receiver-surface string classifier from the MIR-call
route-policy fallback layer.

This is a BoxShape cleanup. It removes a dead compat sentinel and does not add
a new accepted shape.

## Change

Removed from
[`hako_llvmc_ffi_mir_call_route_policy.inc`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc):

```text
classify_mir_call_receiver_surface(... "ArrayBox" ...)
HAKO_LLVMC_MIR_CALL_RECV_SURFACE_ARRAY_BOX
```

The no-growth allowlist baseline shrank to:

```text
classifiers=13
rows=13
```

## Why This Is Safe

After the `birth` emit-kind prune, the remaining route-policy switch has no
active `ArrayBox` receiver-surface route. `ArrayBox + has` already falls back,
and removing the `ArrayBox` surface classification keeps it on the same
fallback path.

Array-origin `RuntimeDataBox.has` / `ArrayBox.has` compatibility rows are still
tracked separately and were not touched.

## Boundary

- Do not prune `MapBox` / `RuntimeDataBox` receiver-surface rows in this card.
- Do not prune generic `has`, `len`, `push`, or `set` fallback rows here.
- Do not change CoreMethod metadata emission.
- Do not add hot lowering.

## Next Work

Proceed to the next task-order item:

```text
mir_call_receiver_surface MapBox prune probe
```

`MapBox` must be judged independently because `MapBox + has` still has an
active route-policy fallback arm.

## Acceptance

```bash
cargo check -q
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
