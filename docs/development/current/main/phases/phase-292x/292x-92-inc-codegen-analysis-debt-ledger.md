---
Status: Active
Date: 2026-04-22
Scope: `.inc` codegen analysis-debt ledger for Phase 292x.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/investigations/phase137x-inc-codegen-thin-tag-inventory-2026-04-22.md
  - tools/checks/inc_codegen_thin_shim_guard.sh
  - tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv
---

# 292x-92: `.inc` Codegen Analysis-Debt Ledger

## Baseline

Current no-growth baseline:

- `.inc` files: 76
- `.inc` lines: 19,533
- analysis-debt files: 24
- analysis-debt lines: 297

The baseline is not a permission slip to add more C analysis. It is a deletion
ledger. Reductions are expected as route families move to MIR-owned metadata.

## Debt Pattern

The guard counts lines matching:

- `analyze_*candidate`
- `hako_llvmc_match_*seed`
- `window_candidate`
- raw `yyjson_obj_get(... "blocks" | "instructions" | "op")` access

## Rule

- new debt files fail
- per-file debt growth fails
- reductions pass and should prune the allowlist
- deleted analyzers must reduce the baseline in the same commit

## First Reduction Target

`array_rmw_window`:

- debt file: `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_window.inc`
- current C function: `analyze_array_rmw_window_candidate`
- target: route tag emitted by MIR, consumed by
  `hako_llvmc_ffi_generic_method_get_lowering.inc`

## Landed Reduction

`array_string_len_window`:

- deleted C function: `analyze_array_string_len_window_candidate`
- deleted trace fallback: `trace_array_string_len_window_candidate`
- lowered allowlist:
  - `hako_llvmc_ffi_generic_method_get_lowering.inc`: `4 -> 2`
  - `hako_llvmc_ffi_generic_method_get_window.inc`: `6 -> 3`

`array_rmw_window`:

- deleted C function: `analyze_array_rmw_window_candidate`
- deleted trace fallback: `trace_array_rmw_window_candidate`
- removed now-clean allowlist rows:
  - `hako_llvmc_ffi_generic_method_get_lowering.inc`
  - `hako_llvmc_ffi_generic_method_get_window.inc`

`string_direct_set_window_routes`:

- deleted hidden C matcher:
  - `ArrayStringPiecewiseDirectSetSourceReuseMatch`
  - `match_array_string_piecewise_concat3_direct_set_source_reuse`
- note: this matcher was not counted by the no-growth regex baseline, so the
  analysis-debt line count stays `314`; the `.inc` total shrank to `19,234`.

`generic_method.has`:

- moved the first generic method route-policy leaf to MIR metadata
  (`GenericMethodRoute`)
- note: this adds metadata reader / validation glue, not raw MIR analysis; the
  analysis-debt baseline stayed `314` for that card.

`array_string_store_micro` exact seed backend route:

- added function-level `exact_seed_backend_route` metadata for the already
  MIR-owned `array_string_store_micro_seed_route`
- renamed the backend consumer away from the legacy `hako_llvmc_match_*seed`
  debt pattern
- lowered allowlist:
  - removed now-clean `hako_llvmc_ffi_array_string_store_seed.inc`
  - `hako_llvmc_ffi_pure_compile.inc`: `34 -> 33`
  - current analysis-debt baseline is `312`

`concat_const_suffix_micro` exact seed backend route:

- extended `exact_seed_backend_route` to select the already MIR-owned
  `concat_const_suffix_micro_seed_route`
- renamed the backend consumer away from the legacy `hako_llvmc_match_*seed`
  debt pattern
- pinned the route with
  `phase137x_direct_emit_concat_const_suffix_contract.sh`
- lowered allowlist:
  - removed now-clean `hako_llvmc_ffi_concat_const_suffix_seed.inc`
  - `hako_llvmc_ffi_pure_compile.inc`: `33 -> 32`
  - current analysis-debt baseline is `310`

`substring_views_only_micro` exact seed backend route:

- extended `exact_seed_backend_route` to select the already MIR-owned
  `substring_views_micro_seed_route`
- renamed the backend consumer away from the legacy `hako_llvmc_match_*seed`
  debt pattern
- pinned the route with `phase137x_direct_emit_substring_views_contract.sh`
- lowered allowlist:
  - removed now-clean `hako_llvmc_ffi_string_loop_seed_views_only.inc`
  - `hako_llvmc_ffi_pure_compile.inc`: `32 -> 31`
  - current analysis-debt baseline is `308`

`substring_concat_loop_ascii` exact seed backend route:

- extended `exact_seed_backend_route` to select a concrete
  `string_kernel_plans.loop_payload` entry via `selected_value`
- removed the redundant substring-concat seed wrapper; the selected metadata
  consumer chooses the len emitter from MIR-owned stable-length metadata
- pinned the route with
  `phase137x_direct_emit_substring_concat_route_contract.sh`
- current analysis-debt baseline after this slice was `302`

`array_rmw_add1_leaf` exact seed backend route:

- added `FunctionMetadata.array_rmw_add1_leaf_seed_route` for the current
  whole-function 7-block direct MIR seed shape
- kept the inner RMW legality delegated to
  `array_rmw_window_routes[*].proof = "array_get_add1_set_same_slot"`
- converted `hako_llvmc_match_array_rmw_add1_leaf_seed` into
  `hako_llvmc_consume_array_rmw_add1_leaf_route`
- pinned the route with
  `phase137x_direct_emit_array_rmw_add1_leaf_contract.sh`
- lowered allowlist:
  - `hako_llvmc_ffi_array_micro_seed.inc`: `8 -> 4`
  - `hako_llvmc_ffi_pure_compile.inc`: `29 -> 28`
  - current analysis-debt baseline is `297`
