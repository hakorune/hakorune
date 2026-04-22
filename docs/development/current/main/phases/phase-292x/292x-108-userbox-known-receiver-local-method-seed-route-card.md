---
Status: Landed
Date: 2026-04-23
Scope: Move the local/copy UserBox known-receiver method exact seeds behind MIR-owned route metadata.
Related:
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-107-userbox-loop-micro-seed-route-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_counter_step_local_i64.inc
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_point_sum_local_i64.inc
---

# 292x-108: UserBox Known-Receiver Local Method Seed Route

## Intent

Thin `.inc` codegen by moving the local/copy exact seeds for
`Counter.step/1` and `Point.sum/1` into MIR-owned route metadata.

This slice is intentionally narrower than the whole known-receiver family:

- included:
  - `counter_step_local_i64`
  - `counter_step_copy_local_i64`
  - `point_sum_local_i64`
  - `point_sum_copy_local_i64`
- deferred:
  - `counter_step_chain_micro`
  - `counter_step_micro`
  - `point_sum_micro`

The deferred shapes still carry loop/block/value-id assumptions and should get
their own route proof before their C matchers are removed.

## Route Contract

- owner: `FunctionMetadata.userbox_known_receiver_method_seed_route`
- backend tag:
  `metadata.exact_seed_backend_route.tag = "userbox_known_receiver_method_seed"`
- selected source route: `userbox_known_receiver_method_seed_route`
- route kinds:
  - `counter_step_local_i64`
  - `counter_step_copy_local_i64`
  - `point_sum_local_i64`
  - `point_sum_copy_local_i64`
- proofs:
  - `userbox_counter_step_local_i64_seed`
  - `userbox_point_sum_local_i64_seed`
- consumer capability: `direct_userbox_known_receiver_method_local`
- publication boundary: `none`

## C Boundary Rules

The C boundary may:

- validate the route metadata fields,
- validate user-box declarations,
- validate the existing `thin_entry_selections` /
  `placement_effect_routes` prerequisites,
- emit the existing local helper.

The C boundary must not scan `blocks`, `instructions`, or raw `op` fields to
rediscover these four shapes.

## Acceptance

```bash
cargo fmt --check
cargo test -q userbox_known_receiver_method_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_method_known_receiver_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Expected Trace

The four local/copy fixtures should show:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata
  extra=userbox_known_receiver_method_seed`
- `stage=userbox_known_receiver_method_seed result=emit reason=exact_match`

The `Counter.step_chain` fixture remains green through the legacy matcher in
this slice and is not expected to advertise the new route yet.

## Cleanup Boundary

Delete or shrink only the legacy C matchers covered by this card:

- `hako_llvmc_match_userbox_counter_step_local_i64_seed`
- `hako_llvmc_match_userbox_counter_step_copy_local_i64_seed`
- `hako_llvmc_match_userbox_point_sum_local_i64_seed`
- `hako_llvmc_match_userbox_point_sum_copy_local_i64_seed`

Keep the actual emit helpers for `counter_step_local_i64` and
`point_sum_local_i64`. Do not touch the chain/micro method matchers in this
slice.

## Result

Landed as `FunctionMetadata.userbox_known_receiver_method_seed_route` plus
`metadata.exact_seed_backend_route.tag =
"userbox_known_receiver_method_seed"`.

- `counter_step_local_i64`, `counter_step_copy_local_i64`,
  `point_sum_local_i64`, and `point_sum_copy_local_i64` are matched in MIR,
  not in C.
- C now consumes `userbox_known_receiver_method_seed_route`, validates
  thin-entry / declaration prerequisites, and emits the existing local helper.
- The two copy-only matcher include files were deleted.
- The two local include files are emitter-only.
- The `Counter.step_chain`, `Counter.step` micro, and `Point.sum` micro
  matchers remain as the next UserBox cleanup slice.
- Debt guard baseline moved from `13 files / 131 lines` to
  `9 files / 99 lines`.

## Verification

```bash
cargo test -q userbox_known_receiver_method_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_method_known_receiver_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
```
