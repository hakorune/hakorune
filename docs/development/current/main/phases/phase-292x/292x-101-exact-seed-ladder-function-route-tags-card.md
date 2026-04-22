---
Status: Active
Date: 2026-04-22
Scope: next cleanup card for moving exact seed ladders toward function-level backend route tags.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
---

# 292x-101: Exact Seed Ladder Function Route Tags

Compact status for landed slices, guard baseline, and remaining backlog is
`292x-STATUS.toml`. This card keeps the design and result notes for the active
exact-seed migration family.

## Problem

Several exact seed ladders still enter `.inc` through helper-specific matcher
families. Even when the active route already has MIR-owned metadata, the
function boundary can still read like a list of backend-local exact shape
attempts instead of a single function-level route decision.

## Decision

Do not widen accepted MIR shapes in this card. Pick one exact seed ladder that
already has a MIR metadata owner, then move the function-level selection to a
single backend route tag.

First slice:

- exact seed ladder: `array_string_store_micro`
- existing route owner: `FunctionMetadata.array_string_store_micro_seed_route`
- new function-level tag: `metadata.exact_seed_backend_route.tag =
  "array_string_store_micro"`
- selected source route: `array_string_store_micro_seed_route`
- proof: `kilo_micro_array_string_store_8block`

Second slice:

- exact seed ladder: `concat_const_suffix_micro`
- existing route owner: `FunctionMetadata.concat_const_suffix_micro_seed_route`
- function-level tag: `metadata.exact_seed_backend_route.tag =
  "concat_const_suffix_micro"`
- selected source route: `concat_const_suffix_micro_seed_route`
- proof: `kilo_micro_concat_const_suffix_5block`

Third slice:

- exact seed ladder: `substring_views_only_micro`
- existing route owner: `FunctionMetadata.substring_views_micro_seed_route`
- function-level tag: `metadata.exact_seed_backend_route.tag =
  "substring_views_only_micro"`
- selected source route: `substring_views_micro_seed_route`
- proof: `kilo_micro_substring_views_only_5block`

Fourth slice:

- exact seed ladder: `substring_concat_loop_ascii`
- existing route owner: `FunctionMetadata.string_kernel_plans[*].loop_payload`
- selected value: `metadata.exact_seed_backend_route.selected_value`
  points at the preselected `StringKernelPlan` key
- function-level tag: `metadata.exact_seed_backend_route.tag =
  "substring_concat_loop_ascii"`
- selected source route: `string_kernel_plans.loop_payload`
- proof: `string_kernel_plan_concat_triplet_loop_payload`

Fifth slice:

- exact seed ladder: `array_rmw_add1_leaf`
- new route owner: `FunctionMetadata.array_rmw_add1_leaf_seed_route`
- relationship to existing route metadata:
  - `array_rmw_window_routes` still owns the inner
    `array.get(i) -> +1 -> array.set(i, ...)` legality proof
  - `array_rmw_add1_leaf_seed_route` owns the whole-function exact seed
    payload (`size`, `ops`, init push loop, and final first/last reads)
- function-level tag: `metadata.exact_seed_backend_route.tag =
  "array_rmw_add1_leaf"`
- selected source route: `array_rmw_add1_leaf_seed_route`
- proof: `kilo_leaf_array_rmw_add1_7block`

`.inc` may keep only:

- metadata reader / field validation
- selected helper emission
- fail-fast on inconsistent route metadata

## Acceptance

Pin the active slices with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
cargo test -q exact_seed_backend_route
cargo test -q build_mir_json_root_emits_exact_seed_backend_route
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_concat_const_suffix_contract.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_views_contract.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_route_contract.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_phi_merge_contract.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_post_sink_shape.sh
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_rmw_add1_leaf_contract.sh
```

Each boundary smoke must observe:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata`
- its selected exact seed emitter with `result=emit reason=exact_match`

## First Slice Result

- `FunctionMetadata.exact_seed_backend_route` selects the existing
  `array_string_store_micro_seed_route`.
- `compile_json_compat_pure` consumes the function-level tag before walking the
  remaining compatibility ladder.
- `hako_llvmc_ffi_array_string_store_seed.inc` no longer contributes to the
  `hako_llvmc_match_*seed` debt baseline.

## Second Slice Result

- `ExactSeedBackendRouteKind` includes `concat_const_suffix_micro`.
- `compile_json_compat_pure` dispatches that tag before the compatibility
  ladder.
- `hako_llvmc_ffi_concat_const_suffix_seed.inc` no longer contributes to the
  `hako_llvmc_match_*seed` debt baseline.
- `phase137x_direct_emit_concat_const_suffix_contract.sh` pins the direct MIR
  route tag and the exact emitter trace.

## Third Slice Result

- `ExactSeedBackendRouteKind` includes `substring_views_only_micro`.
- `compile_json_compat_pure` dispatches that tag before the compatibility
  ladder.
- `hako_llvmc_ffi_string_loop_seed_views_only.inc` no longer contributes to the
  `hako_llvmc_match_*seed` debt baseline.
- `phase137x_direct_emit_substring_views_contract.sh` pins the direct MIR
  route tag and the exact emitter trace.

## Fourth Slice Result

- `ExactSeedBackendRouteKind` includes `substring_concat_loop_ascii`.
- `exact_seed_backend_route.selected_value` selects the concrete
  `StringKernelPlan` entry so `.inc` does not rediscover the plan by scanning
  every `string_kernel_plans` key.
- `compile_json_compat_pure` dispatches that tag before the compatibility
  ladder.
- The redundant `substring_concat_len_ascii_seed` wrapper is removed; the
  selected substring-concat consumer still chooses the len emitter when MIR
  metadata exposes `stable_length_scalar`.
- `hako_llvmc_ffi_string_loop_seed_substring_concat.inc` no longer contributes
  to the `hako_llvmc_match_*seed` debt baseline.
- `phase137x_direct_emit_substring_concat_route_contract.sh` pins the direct
  MIR route tag, selected plan value, and exact emitter trace.
- The older substring-concat shape smokes now pin current metadata invariants
  instead of stale hard-coded value ids.

## Fifth Slice Result

- Added `FunctionMetadata.array_rmw_add1_leaf_seed_route`.
- The route requires the current 7-block direct MIR shape and the already-proven
  `array_rmw_window_routes[*].proof = "array_get_add1_set_same_slot"` inner
  window.
- `ExactSeedBackendRouteKind` includes `array_rmw_add1_leaf`.
- `hako_llvmc_match_array_rmw_add1_leaf_seed` was converted into
  `hako_llvmc_consume_array_rmw_add1_leaf_route`, which validates metadata and
  emits the selected helper without scanning raw MIR JSON blocks.
- `hako_llvmc_ffi_array_micro_seed.inc` still carries
  `array_getset_micro_seed` raw scan debt; that is a separate owner card.
- The route is pinned with
  `phase137x_direct_emit_array_rmw_add1_leaf_contract.sh`.
- The analysis-debt baseline is now `297` lines.
