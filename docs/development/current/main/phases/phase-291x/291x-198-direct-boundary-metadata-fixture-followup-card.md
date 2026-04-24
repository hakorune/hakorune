---
Status: Landed
Date: 2026-04-25
Scope: Add CoreMethod metadata to direct ArrayPush/MapHas/MapLen pure-boundary MIR JSON fixtures.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-196-mir-call-route-state-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-197-mir-call-need-metadata-consumer-card.md
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
  - tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
---

# 291x-198 Direct Boundary Metadata Fixture Follow-Up Card

## Goal

Convert remaining direct pure-boundary fixtures that already have landed
CoreMethod carriers and `.inc` consumers:

```text
ArrayBox.push -> ArrayPush
MapBox.has    -> MapHas
MapBox.size   -> MapLen
```

This increases metadata-bearing boundary coverage after `291x-196` and
`291x-197`.

## Boundary

- Do not convert `ArrayBox.get` in this card; direct `ArrayGet` route carrier /
  consumer wiring is not complete.
- Do not convert direct `MapBox.get` in this card; current `MapGet` carrier is
  RuntimeData-facade oriented.
- Do not prune allowlist rows in this card.
- Do not change expected return codes.

## Implementation

- Add `generic_method.push + core_method.op=ArrayPush` metadata to the array
  push/get pure boundary fixture.
- Add `generic_method.has + core_method.op=MapHas` metadata to the map
  set/has pure boundary fixture.
- Add `generic_method.len + core_method.op=MapLen` metadata to the map set/size
  pure boundary fixture.

## Result

The direct fixtures now exercise metadata-first route-state and need-policy
selection for ArrayPush, MapHas, and MapLen. Direct get surfaces remain tracked
as a separate carrier gap.

## Acceptance

```bash
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
