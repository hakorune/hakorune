---
Status: Landed
Date: 2026-04-25
Scope: Make generic-method `push` emit-kind selection prefer MIR CoreMethod metadata before legacy fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-174-core-method-push-route-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# 291x-175 CoreMethod Push Emit-Kind Metadata Card

## Goal

Move direct ArrayBox push emit-kind selection to the metadata-first boundary:

```text
generic_method_routes[].route_id = generic_method.push
core_method.op = ArrayPush
  -> dispatch selects HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH
  -> legacy push classifier remains fallback only
```

This is a one-family consumer card. It does not delete allowlist rows and does
not change push helper selection or lowering.

## Boundary

- Do not remove the `push` allowlist row.
- Do not change `nyash.array.slot_append_hh` or `nyash.runtime_data.push_hh`.
- Do not add hot inline lowering.
- Do not infer mutating legality from method names in the new selector; only
  consume MIR-owned CoreMethod metadata.
- Keep metadata-absent push routes on the legacy fallback.

## Implementation

- Extend the generic emit-kind metadata selector to accept
  `route_id=generic_method.push`.
- Accept only `core_method.op=ArrayPush` with
  `proof=core_method_contract_manifest` and
  `lowering_tier=cold_fallback`.

## Result

`emit_mir_call_dispatch(...)` can now select `EMIT_PUSH` from valid MIR
`ArrayPush` CoreMethod metadata before the legacy method-name classifier. The
legacy fallback remains required until a separate prune probe proves
metadata-absent mutating boundary fixtures are covered.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_route
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
