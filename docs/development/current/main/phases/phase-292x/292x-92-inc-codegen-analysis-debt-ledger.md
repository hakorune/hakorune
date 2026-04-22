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

- `.inc` files: 75
- `.inc` lines: 18,153
- analysis-debt files: 21
- analysis-debt lines: 206

Compact current status is mirrored in `292x-STATUS.toml`. This ledger keeps
the deletion history and the reason for baseline changes.

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

Sum `variant_tag` exact seed backend route:

- added `FunctionMetadata.sum_variant_tag_seed_route` for the current local and
  copy `variant_tag` exact seed family
- converted the five `hako_llvmc_match_variant_tag_*seed` C matchers into
  `hako_llvmc_consume_sum_variant_tag_seed_route`
- updated metadata-bearing tag fixtures to carry `sum_variant_tag_seed_route`
  and `exact_seed_backend_route`
- pinned route traces in `phase163x_boundary_sum_metadata_keep_min.sh`
- lowered allowlist:
  - `hako_llvmc_ffi_sum_local_seed_matchers_tag.inc`: `35 -> 0` and removed
    from the baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `28 -> 23`
  - current analysis-debt baseline is `257`

Sum `variant_project` exact seed backend route:

- added `FunctionMetadata.sum_variant_project_seed_route` for the current
  local and copy `variant_project` exact seed family
- replaced the six `hako_llvmc_match_variant_project_*seed` C matchers with
  `hako_llvmc_consume_sum_variant_project_seed_route`
- deleted the old project local/copy matcher include files and added
  `hako_llvmc_ffi_sum_local_seed_project_route.inc`
- updated metadata-bearing project fixtures to carry
  `sum_variant_project_seed_route` and `exact_seed_backend_route`
- pinned route traces in `phase163x_boundary_sum_metadata_keep_min.sh`
- lowered allowlist:
  - `hako_llvmc_ffi_sum_local_seed_matchers_project_copy.inc`: `24 -> 0` and
    removed from the baseline
  - `hako_llvmc_ffi_sum_local_seed_matchers_project_local.inc`: `21 -> 0` and
    removed from the baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `23 -> 17`
  - current analysis-debt baseline is `206`

UserBox Point local scalar exact seed backend route:

- added `FunctionMetadata.userbox_local_scalar_seed_route` for the current
  Point local/copy scalar exact seed pair
- replaced the two `hako_llvmc_match_userbox_point_*seed` C matchers with
  `hako_llvmc_consume_userbox_point_local_scalar_route`
- deleted the old copy-only Point matcher include and kept the local Point
  include emitter-only
- updated Point local/copy fixtures to carry `userbox_local_scalar_seed_route`
  and `exact_seed_backend_route`
- pinned route traces in `phase163x_boundary_user_box_metadata_keep_min.sh`
- lowered allowlist:
  - `hako_llvmc_ffi_user_box_micro_seed_point_copy_local_i64.inc`: `13 -> 0`
    and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_point_local_i64.inc`: `12 -> 0`
    and removed from the baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `17 -> 15`
  - current analysis-debt baseline is `179`
