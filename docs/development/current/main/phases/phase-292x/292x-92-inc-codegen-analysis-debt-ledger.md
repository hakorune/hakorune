---
Status: Active
Date: 2026-04-23
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

UserBox Flag/PointF local scalar exact seed backend route:

- extended `FunctionMetadata.userbox_local_scalar_seed_route` to cover the
  current Flag BoolBox and PointF FloatBox local/copy scalar exact seed family
- added backend tag `userbox_flag_pointf_local_scalar`
- replaced the four `hako_llvmc_match_userbox_{flag,pointf}_*seed` C matchers
  with `hako_llvmc_consume_userbox_flag_pointf_local_scalar_route`
- deleted the old copy-only Flag/PointF matcher include files and kept the
  local Flag/PointF includes emitter-only
- updated Flag/PointF local/copy fixtures to carry
  `userbox_local_scalar_seed_route` and `exact_seed_backend_route`
- pinned route traces in `phase163x_boundary_user_box_metadata_keep_min.sh`
- lowered allowlist:
  - `hako_llvmc_ffi_user_box_micro_seed_flag_copy_local_bool.inc`: `9 -> 0`
    and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_flag_local_bool.inc`: `8 -> 0`
    and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_pointf_copy_local_f64.inc`: `9 -> 0`
    and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_pointf_local_f64.inc`: `8 -> 0`
    and removed from the baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `15 -> 11`
  - current analysis-debt baseline is `141`

UserBox loop micro exact seed backend route:

- added `FunctionMetadata.userbox_loop_micro_seed_route` for the current
  `point_add_micro` and `flag_toggle_micro` loop-count-coupled exact seed pair
- added backend tag `userbox_loop_micro`
- replaced the two `hako_llvmc_match_userbox_{point_add,flag_toggle}_micro_seed`
  C matchers with `hako_llvmc_consume_userbox_loop_micro_route`
- kept `point_add_micro` and `flag_toggle_micro` emit helpers as emitter-only
  include files
- pinned direct MIR metadata and boundary route traces in
  `phase163x_boundary_user_box_loop_micro_route_min.sh`
- lowered allowlist:
  - `hako_llvmc_ffi_user_box_micro_seed_point_add_micro.inc`: `4 -> 0` and
    removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_flag_toggle_micro.inc`: `4 -> 0` and
    removed from the baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `11 -> 9`
  - current analysis-debt baseline is `131`

UserBox known-receiver local method exact seed backend route:

- added `FunctionMetadata.userbox_known_receiver_method_seed_route` for the
  current `Counter.step/1` and `Point.sum/1` local/copy exact seed family
- added backend tag `userbox_known_receiver_method_seed`
- replaced the four
  `hako_llvmc_match_userbox_{counter_step,point_sum}_{local,copy}_i64_seed`
  C matchers with
  `hako_llvmc_consume_userbox_known_receiver_method_seed_route`
- deleted the old copy-only Counter.step / Point.sum matcher include files and
  kept the local Counter.step / Point.sum includes emitter-only
- updated the four local/copy known-receiver fixtures to carry
  `userbox_known_receiver_method_seed_route` and `exact_seed_backend_route`
- pinned route traces in
  `phase163x_boundary_user_box_method_known_receiver_min.sh`
- lowered allowlist:
  - `hako_llvmc_ffi_user_box_micro_seed_counter_step_copy_local_i64.inc`:
    `7 -> 0` and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_counter_step_local_i64.inc`:
    `7 -> 0` and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_point_sum_copy_local_i64.inc`:
    `7 -> 0` and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_point_sum_local_i64.inc`:
    `7 -> 0` and removed from the baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `9 -> 5`
  - current analysis-debt baseline is `99`

UserBox known-receiver chain/micro method exact seed backend route:

- extended `FunctionMetadata.userbox_known_receiver_method_seed_route` to cover
  `Counter.step_chain`, `Counter.step` micro, and `Point.sum` micro shapes
- deleted `hako_llvmc_match_userbox_counter_step_chain_micro_seed`
- replaced `hako_llvmc_match_userbox_counter_step_micro_seed` and
  `hako_llvmc_match_userbox_point_sum_micro_seed` with route metadata
  consumption while keeping their emit helpers
- updated the `Counter.step_chain` prebuilt fixture to carry
  `userbox_known_receiver_method_seed_route` and `exact_seed_backend_route`
- relaxed direct known-receiver smokes away from value-id pins and toward
  route payload / call-subject contracts
- lowered allowlist:
  - `hako_llvmc_ffi_user_box_micro_seed_counter_step_chain_micro.inc`:
    `19 -> 0` and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_counter_step_micro.inc`:
    `7 -> 0` and removed from the baseline
  - `hako_llvmc_ffi_user_box_micro_seed_point_sum_micro.inc`:
    `18 -> 0` and removed from the baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `5 -> 2`
  - current analysis-debt baseline is `52`

Array get/set micro exact seed backend route:

- added `FunctionMetadata.array_getset_micro_seed_route` for the current
  7-block `bench_kilo_micro_array_getset.hako` direct MIR shape
- kept the inner RMW legality proof owned by `array_rmw_window_routes`
- replaced `hako_llvmc_match_array_getset_micro_seed` with
  `hako_llvmc_consume_array_getset_micro_route`
- added `phase137x_direct_emit_array_getset_micro_contract.sh` to pin route
  payload, exact backend tag dispatch, and stack-array emitter selection
- lowered allowlist:
  - `hako_llvmc_ffi_array_micro_seed.inc`: `4 -> 0` and removed from the
    baseline
  - `hako_llvmc_ffi_pure_compile.inc`: `2 -> 1`
  - current analysis-debt baseline is `47`
- there are no remaining `hako_llvmc_match_*seed` definitions

Pure compile minimal ret/branch deletion:

- deleted `pure_compile_minimal_paths` path #1 `const* -> ret const`
- deleted `pure_compile_minimal_paths` path #2 const compare branch with merge
  ret
- kept the Hako LL daily owner and llvmlite monitor canaries green
- lowered allowlist:
  - `hako_llvmc_ffi_pure_compile_minimal_paths.inc`: `40 -> 27`
  - current analysis-debt baseline is `34`

Pure compile minimal Array path deletion:

- deleted `pure_compile_minimal_paths` path #4 `ArrayBox` constructor, `push`,
  `len/length/size`, `ret`
- path #3 `MapBox` set-size stays as fallback after a failed delete probe
- lowered allowlist:
  - `hako_llvmc_ffi_pure_compile_minimal_paths.inc`: `27 -> 21`
  - current analysis-debt baseline is `28`

Pure compile minimal Map path deletion:

- updated the pure-historical Map set-size smoke from legacy receiver-in-args
  Method MIR to canonical `box_name` / `method` / `receiver` Method MIR
- deleted `pure_compile_minimal_paths` path #3 `MapBox` constructor, `set`,
  `size/len`, `ret`
- lowered allowlist:
  - `hako_llvmc_ffi_pure_compile_minimal_paths.inc`: `21 -> 14`
  - current analysis-debt baseline is `21`

Pure compile minimal String const-eval deletion:

- deleted `pure_compile_minimal_paths` paths #5/#6 and removed
  `hako_llvmc_ffi_pure_compile_minimal_paths.inc`
- moved the surviving ownership to generic pure lowering by materializing
  skipped StringBox constants at the `newbox StringBox` boundary when a later
  method needs a handle
- updated string length / runtime-data length / indexOf boundary smokes from
  legacy seed wording to generic boundary wording
- lowered allowlist:
  - removed `hako_llvmc_ffi_pure_compile_minimal_paths.inc`: `14 -> 0`
  - current analysis-debt baseline is `7`

String loop seed copy-graph helper deletion:

- removed the unreferenced
  `hako_llvmc_ffi_string_loop_seed_copy_graph.inc` include from the string loop
  seed facade
- deleted the dead copy-chain / copy-graph helper file
- lowered allowlist:
  - removed `hako_llvmc_ffi_string_loop_seed_copy_graph.inc`: `2 -> 0`
  - current analysis-debt baseline is `5`
