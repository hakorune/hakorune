---
Status: Landed
Date: 2026-04-26
Scope: Add the ArrayHas CoreMethod carrier without pruning legacy has fallback rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-211-runtime-data-has-compat-contract-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-261-has-family-no-safe-prune-review-card.md
  - lang/src/runtime/meta/core_method_contract_box.hako
  - src/mir/core_method_op.rs
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-268 ArrayHas CoreMethod Carrier Card

## Goal

Land the smallest BoxCount step required by the remaining `has` cleanup
blocker: represent Array index-presence checks as `CoreMethodOp::ArrayHas`.

This card must not prune the legacy fallback rows. The purpose is to make new
MIR metadata carry the contract so a later card can judge deletion with exact
boundary evidence.

## Contract

```text
ArrayBox.has(index)
RuntimeDataBox.has(index) with receiver_origin_box=ArrayBox
  -> core_method.op=ArrayHas
  -> route_kind=array_contains_any
  -> helper_symbol=nyash.array.has_hh
  -> effect=probe.key
```

`array_contains_any` keeps the current any-key fail-safe behavior:

- valid integer index in range returns `1`
- missing/out-of-range/non-index key returns `0`
- no value containment semantics are introduced

## Boundary

- Add only the `ArrayHas` contract/carrier and metadata consumer.
- Keep `classify_generic_method_emit_kind(... "has" ...)`.
- Keep `classify_generic_method_has_route(... "ArrayBox" ...)`.
- Keep `classify_generic_method_has_route(... "RuntimeDataBox" ...)`.
- Keep MIR-call `MapBox + has` surface sentinels.

## Acceptance

```bash
python3 tools/core_method_contract_manifest_codegen.py --write
cargo test -q arrayhas
cargo test -q manifest_core_ops_are_known_by_mir_carrier
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_manifest_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

## Result

Landed.

- Added `ArrayBox.has/1 -> ArrayHas` to `CoreMethodContractBox` and regenerated
  `core_method_contract_manifest.json`.
- Added `CoreMethodOp::ArrayHas` and MIR route metadata for direct
  `ArrayBox.has` plus Array-origin `RuntimeDataBox.has`.
- Added `array_contains_any -> nyash.array.has_hh` to the generic `has` `.inc`
  metadata consumer and pure-compile need declarations.
- Updated the Array-origin RuntimeData has boundary fixture to carry
  `core_method.op=ArrayHas`.
- Kept all five no-growth rows pinned; the guard remains
  `classifiers=5 rows=5`.

Validated with:

```bash
python3 -m json.tool apps/tests/mir_shape_guard/runtime_data_array_has_missing_min_v1.mir.json >/dev/null
python3 tools/core_method_contract_manifest_codegen.py --write
cargo test -q arrayhas
cargo test -q manifest_core_ops_are_known_by_mir_carrier
cargo test -q build_mir_json_root_emits_generic_method_routes
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_manifest_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
git diff --check
```
