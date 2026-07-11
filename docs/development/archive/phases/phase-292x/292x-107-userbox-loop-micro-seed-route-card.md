---
Status: Landed
Date: 2026-04-23
Scope: Move UserBox point-add / flag-toggle loop micro exact seeds behind MIR-owned route metadata.
Related:
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-106-userbox-flag-pointf-local-scalar-seed-route-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_point_add_micro.inc
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_flag_toggle_micro.inc
---

# 292x-107: UserBox Loop Micro Seed Route

## Intent

Thin `.inc` codegen by moving the remaining loop-count-coupled UserBox micro
seed checks for `kilo_micro_userbox_point_add` and
`kilo_micro_userbox_flag_toggle` into MIR-owned route metadata.

The C boundary should only:

- validate `FunctionMetadata.userbox_loop_micro_seed_route`,
- validate the existing thin-entry and user-box declaration prerequisites,
- emit the selected helper,
- trace skip/emit with stable route tags.

It must not rescan raw `blocks` / `instructions` / `op` to rediscover these
benchmark shapes.

## Route Contract

- owner: `FunctionMetadata.userbox_loop_micro_seed_route`
- backend tag: `metadata.exact_seed_backend_route.tag = "userbox_loop_micro"`
- selected source route: `userbox_loop_micro_seed_route`
- route kinds:
  - `point_add_micro`
  - `flag_toggle_micro`
- proofs:
  - `userbox_point_add_loop_micro_seed`
  - `userbox_flag_toggle_loop_micro_seed`
- payload:
  - `ops = 2000000`
  - `flip_at = 1000000` for `flag_toggle_micro`

## Acceptance

```bash
cargo fmt --check
cargo test -q userbox_loop_micro_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_loop_micro_route_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Expected Trace

The point-add and flag-toggle benchmarks should show:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata
  extra=userbox_loop_micro`
- `stage=userbox_loop_micro result=emit reason=exact_match`

## Cleanup Boundary

Delete or shrink only the legacy C matchers covered by this card:

- `hako_llvmc_match_userbox_point_add_micro_seed`
- `hako_llvmc_match_userbox_flag_toggle_micro_seed`

Keep the actual emit helpers. Counter and PointSum known-receiver method seeds
remain separate work.

## Result

Landed as `FunctionMetadata.userbox_loop_micro_seed_route` plus
`metadata.exact_seed_backend_route.tag = "userbox_loop_micro"`.

- `point_add_micro` and `flag_toggle_micro` are matched in MIR, not in C.
- C now consumes `userbox_loop_micro_seed_route`, validates thin-entry /
  declaration prerequisites, and emits the existing helper.
- `hako_llvmc_match_userbox_point_add_micro_seed` and
  `hako_llvmc_match_userbox_flag_toggle_micro_seed` were removed.
- Debt guard baseline moved from `15 files / 141 lines` to
  `13 files / 131 lines`.

## Verification

```bash
cargo test -q userbox_loop_micro_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_loop_micro_route_min.sh
```
