---
Status: Landed
Date: 2026-04-23
Scope: Move UserBox `Point` local/copy scalar exact seeds from C-side MIR
  scanning to a function-level backend route tag.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md
---

# 292x-105: UserBox Point Local Scalar Seed Route

## Problem

The UserBox local scalar exact seed ladder is still mixed into the C boundary.
The narrowest remaining slice is the Point field-local pair:

- `hako_llvmc_match_userbox_point_local_i64_seed`
- `hako_llvmc_match_userbox_point_copy_local_i64_seed`

Both matchers rescan one-block MIR JSON for `Point.x` / `Point.y` field
set/get windows, even though the MIR metadata stack already records the
canonical field surface through:

- `thin_entry_selections`
- `placement_effect_routes`
- module-level `user_box_decls` / `user_box_field_decls`

The boundary should validate a MIR-owned route payload and emit the selected
exact helper without rediscovering the instruction window.

## Decision

Add one MIR-owned exact seed route for the Point pair:

- owner: `FunctionMetadata.userbox_local_scalar_seed_route`
- backend tag: `metadata.exact_seed_backend_route.tag =
  "userbox_point_local_scalar"`
- selected source route: `userbox_local_scalar_seed_route`
- proof: `userbox_point_field_local_scalar_seed`
- covered kinds:
  - `point_local_i64`
  - `point_copy_local_i64`

The route is intentionally Point-only. `Flag` / `PointF` local scalar routes
and the loop-count-coupled `point_add_micro` / `flag_toggle_micro` routes stay
as follow-up slices so Bool/F64 variance and multi-block loop contracts do not
mix with this first cut.

## Acceptance

```bash
cargo fmt --check
cargo test -q userbox_local_scalar_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_metadata_keep_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Boundary traces for migrated Point fixtures must show:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata`
- `stage=userbox_point_local_scalar result=emit reason=exact_match`

## Result

- Added `FunctionMetadata.userbox_local_scalar_seed_route`.
- Added `ExactSeedBackendRouteKind::UserBoxPointLocalScalar` with backend tag
  `userbox_point_local_scalar`.
- Replaced the Point local/copy raw C matchers with
  `hako_llvmc_consume_userbox_point_local_scalar_route`.
- Deleted the old copy-only matcher include file and kept
  `hako_llvmc_ffi_user_box_micro_seed_point_local_i64.inc` as an emitter-only
  slice.
- Updated the Point local/copy prebuilt MIR fixtures to carry
  `userbox_local_scalar_seed_route` and `exact_seed_backend_route`.
- `phase163x_boundary_user_box_metadata_keep_min.sh` now pins both exact-route
  hit and UserBox Point route emit traces for migrated Point fixtures.
- The analysis-debt baseline is now `19` files / `179` lines.
