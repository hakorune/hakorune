---
Status: Landed
Date: 2026-04-25
Scope: Make generic-method `set` emit-kind selection prefer MIR CoreMethod metadata before legacy fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-178-core-method-set-route-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# 291x-179 CoreMethod Set Emit-Kind Metadata Card

## Goal

Move direct ArrayBox/MapBox set emit-kind selection to the metadata-first
boundary:

```text
generic_method_routes[].route_id = generic_method.set
core_method.op = ArraySet | MapSet
  -> dispatch selects HAKO_LLVMC_GENERIC_METHOD_EMIT_SET
  -> legacy set classifier remains fallback only
```

This is a consumer card. It does not delete the method-name mirror row and does
not change storage-route selection.

## Boundary

- Do not remove the generic `set` emit-kind mirror row.
- Do not change `classify_generic_method_set_route(...)`.
- Do not change `nyash.array.slot_store_*`, `nyash.map.slot_store_hhh`, or
  `RuntimeDataBox.set` fallback behavior.
- Do not add hot inline lowering.
- Do not infer mutating legality from method names in the new selector; only
  consume MIR-owned CoreMethod metadata.
- Keep metadata-absent set routes on the legacy fallback.

## Implementation

- Extend the generic emit-kind metadata selector to accept
  `route_id=generic_method.set`.
- Accept only `core_method.op=ArraySet` or `core_method.op=MapSet` with
  `proof=core_method_contract_manifest` and
  `lowering_tier=cold_fallback`.
- Leave storage-route planning in the existing set-route classifier for now.

## Result

`emit_mir_call_dispatch(...)` can now select `EMIT_SET` from valid MIR
`ArraySet` / `MapSet` CoreMethod metadata before the legacy method-name
classifier. The legacy fallback remains required until a separate prune probe
proves metadata-absent mutating boundary fixtures are covered.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_set_multiarg_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_write_return_vm.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
