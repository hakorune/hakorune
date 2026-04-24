---
Status: Landed
Date: 2026-04-25
Scope: Add metadata-first CoreMethod carriers for direct ArrayBox.get and MapBox.get without changing hot lowering policy.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-198-direct-boundary-metadata-fixture-followup-card.md
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
---

# 291x-199 Direct Get CoreMethod Carrier Card

## Goal

Close the direct `get` carrier gap left by `291x-198`:

```text
ArrayBox.get -> ArrayGet
MapBox.get   -> MapGet
```

The purpose is compiler-boundary cleanup, not a new hot optimization. MIR owns
the CoreMethod route decision; `.inc` consumes typed route metadata and keeps
the existing lowering behavior.

## Boundary

- Do not add MapGet hot lowering in this card.
- Do not rewrite RuntimeDataBox.get semantics.
- Do not use direct `slot_load_hi` for RuntimeData mixed-return MapGet.
- Do not prune remaining `get` mirror rows in this card.
- Do not add per-benchmark or source-shape matching.

## Implementation

- Add direct `get` route kinds:
  - `map_load_any` -> `nyash.map.slot_load_hh`
  - `array_slot_load_any` -> `nyash.array.slot_load_hi`
- Emit direct CoreMethod carriers from MIR:
  - `MapGet + warm_direct_abi` for `MapBox.get`
  - `ArrayGet + warm_direct_abi` for `ArrayBox.get`
- Extend existing generic GET metadata consumers to validate the direct routes
  and map them onto existing `runtime_map_get` / `runtime_array_get` flags.
- Update representative direct Array/Map pure-boundary fixtures so they exercise
  the metadata-first `get` route instead of legacy method-surface fallback.

## Result

Direct `ArrayBox.get` and direct `MapBox.get` now have CoreMethod route carriers
and GET consumers. RuntimeData `MapGet` remains a cold facade route; this card
does not introduce MapGet hot lowering.

## Acceptance

```bash
cargo test -q generic_method_routes
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
