# Legacy Retirement Ledger

Status: active cleanup ledger
Date: 2026-05-08

## Policy

- Active real-app EXE bringup must use TypedObjectPlan / LoweringPlan facts.
- Do not add new `.hako` app-specific box, method, or field names to C shim
  legacy seed files.
- Legacy exact seeds stay only as historical fixture bridges until an
  equivalent TypedObjectPlan / LoweringPlan route covers the fixture.

## User-Box Micro Seeds

These files still contain narrow user-box fixture knowledge and are not an
extension point:

- `hako_llvmc_ffi_user_box_micro_seed_helpers.inc`
- `hako_llvmc_ffi_user_box_micro_seed_point_route.inc`
- `hako_llvmc_ffi_user_box_micro_seed_flag_pointf_route.inc`
- `hako_llvmc_ffi_user_box_micro_seed_loop_micro_route.inc`
- `hako_llvmc_ffi_user_box_micro_seed_known_receiver_method_route.inc`
- `hako_llvmc_ffi_user_box_micro_seed_counter_step_local_i64.inc`
- `hako_llvmc_ffi_user_box_micro_seed_point_sum_local_i64.inc`
- `hako_llvmc_ffi_user_box_micro_seed_counter_step_micro.inc`
- `hako_llvmc_ffi_user_box_micro_seed_point_sum_micro.inc`
- `hako_llvmc_ffi_user_box_micro_seed_point_local_i64.inc`
- `hako_llvmc_ffi_user_box_micro_seed_flag_local_bool.inc`
- `hako_llvmc_ffi_user_box_micro_seed_pointf_local_f64.inc`
- `hako_llvmc_ffi_user_box_micro_seed_point_add_micro.inc`
- `hako_llvmc_ffi_user_box_micro_seed_flag_toggle_micro.inc`

Known embedded fixture vocabulary:

- `Point.x`, `Point.y`
- `Flag.enabled`
- `PointF.x`
- `Counter.value`
- seed route tags: `userbox_local_scalar_seed_route`,
  `userbox_loop_micro_seed_route`,
  `userbox_known_receiver_method_seed_route`

## Retirement Order

1. Keep all real-app typed-object work on `typed_object_plans` and
   `user_box_method_routes`.
2. For each legacy exact seed, add or identify the replacement route fact in
   MIR-owned metadata.
3. Move any fixture gate to the replacement route.
4. Delete the matching C seed file or reduce it to a route reader with no
   app-specific field names.
5. Remove the route tag from `hako_llvmc_exact_seed_backend_route_supported`.

## Current Non-Goal

Do not remove these files opportunistically while real-app EXE parity is in
progress. Removal is allowed only after the replacement fixture/gate is green.
