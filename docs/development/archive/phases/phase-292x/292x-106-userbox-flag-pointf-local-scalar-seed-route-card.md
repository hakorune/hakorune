---
Status: Landed
Date: 2026-04-23
Scope: Move UserBox Flag/PointF local scalar exact seeds behind MIR-owned route metadata.
Related:
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-105-userbox-point-local-scalar-seed-route-card.md
  - src/mir/userbox_local_scalar_seed_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed.inc
---

# 292x-106: UserBox Flag/PointF Local Scalar Seed Route

## Intent

Thin `.inc` codegen by moving the remaining UserBox local/copy scalar seed
shape checks for `Flag.enabled: BoolBox` and `PointF.x: FloatBox` into
`FunctionMetadata.userbox_local_scalar_seed_route`.

The C boundary should only:

- validate the MIR-owned route metadata,
- validate the already-existing helper prerequisites
  (`thin_entry_selections` and user-box declarations),
- emit the selected helper,
- trace skip/emit with stable route tags.

It must not rescan raw `blocks` / `instructions` / `op` to rediscover these
four seed shapes.

## Route Contract

- owner: `FunctionMetadata.userbox_local_scalar_seed_route`
- backend tag: `metadata.exact_seed_backend_route.tag =
  "userbox_flag_pointf_local_scalar"`
- selected source route: `userbox_local_scalar_seed_route`
- route kinds:
  - `flag_local_bool`
  - `flag_copy_local_bool`
  - `pointf_local_f64`
  - `pointf_copy_local_f64`
- proofs:
  - `userbox_flag_field_local_scalar_seed`
  - `userbox_pointf_field_local_scalar_seed`

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

## Expected Trace

Flag and PointF local/copy fixtures should show:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata
  extra=userbox_flag_pointf_local_scalar`
- `stage=userbox_flag_pointf_local_scalar result=emit reason=exact_match`

Point local/copy fixtures stay on:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata
  extra=userbox_point_local_scalar`
- `stage=userbox_point_local_scalar result=emit reason=exact_match`

## Cleanup Boundary

Delete or shrink only the legacy C matchers covered by the four route kinds
above:

- `hako_llvmc_match_userbox_flag_local_bool_seed`
- `hako_llvmc_match_userbox_flag_copy_local_bool_seed`
- `hako_llvmc_match_userbox_pointf_local_f64_seed`
- `hako_llvmc_match_userbox_pointf_copy_local_f64_seed`

Keep the actual emit helpers. Later loop-count-sensitive UserBox micro seeds
remain separate work.

## Result

- Added `Flag` / `PointF` route kinds to
  `FunctionMetadata.userbox_local_scalar_seed_route`.
- Added backend tag `userbox_flag_pointf_local_scalar`.
- Added `hako_llvmc_consume_userbox_flag_pointf_local_scalar_route`.
- Deleted the copy-only Flag/PointF matcher includes and reduced the local
  includes to emitter-only helpers.
- Updated the four Flag/PointF fixtures to carry
  `userbox_local_scalar_seed_route` and `exact_seed_backend_route`.
- Pinned exact-route trace assertions in
  `phase163x_boundary_user_box_metadata_keep_min.sh`.
- Lowered `.inc` analysis-debt baseline from 19 files / 179 lines to
  15 files / 141 lines.
