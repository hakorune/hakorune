---
Status: Landed
Date: 2026-04-23
Scope: Move the remaining UserBox known-receiver chain/micro exact seeds behind MIR-owned route metadata.
Related:
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-108-userbox-known-receiver-local-method-seed-route-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_counter_step_chain_micro.inc
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_counter_step_micro.inc
  - lang/c-abi/shims/hako_llvmc_ffi_user_box_micro_seed_point_sum_micro.inc
---

# 292x-109: UserBox Known-Receiver Chain/Micro Method Seed Route

## Intent

Finish the UserBox known-receiver method family by moving the remaining
chain/micro exact seed legality out of `.inc` and into
`FunctionMetadata.userbox_known_receiver_method_seed_route`.

Covered shapes:

- `counter_step_chain_local_i64`
- `counter_step_chain_micro`
- `counter_step_micro`
- `point_sum_micro`

This follows 292x-108, which already routed the local/copy `Counter.step/1`
and `Point.sum/1` shapes.

## Route Contract

- owner: `FunctionMetadata.userbox_known_receiver_method_seed_route`
- backend tag:
  `metadata.exact_seed_backend_route.tag = "userbox_known_receiver_method_seed"`
- selected source route: `userbox_known_receiver_method_seed_route`
- new route kinds:
  - `counter_step_chain_local_i64`
  - `counter_step_chain_micro`
  - `counter_step_micro`
  - `point_sum_micro`
- new proofs:
  - `userbox_counter_step_chain_local_i64_seed`
  - `userbox_counter_step_chain_micro_seed`
  - `userbox_counter_step_micro_seed`
  - `userbox_point_sum_micro_seed`
- consumer capability:
  - local helper shapes: `direct_userbox_known_receiver_method_local`
  - closed-loop micro helpers: `direct_userbox_known_receiver_method_micro`
- publication boundary: `none`

## C Boundary Rules

The C boundary may:

- validate route metadata fields,
- validate user-box declarations and existing thin-entry selections,
- emit the selected helper,
- skip or fail fast on inconsistent metadata.

The C boundary must not scan `blocks`, `instructions`, or raw `op` fields to
rediscover these shapes. Loop/block/value-id proof belongs to the MIR owner.

## Smoke Cleanup

The direct `Counter.step`, `Counter.step_chain`, and `Point.sum` smokes should
pin semantic route metadata and call subjects, not brittle value ids. Exact
block/value ids are not part of the C boundary contract.

## Acceptance

```bash
cargo fmt --check
cargo test -q userbox_known_receiver_method_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_method_known_receiver_min.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_counter_step_contract.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_counter_step_chain_contract.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_point_sum_contract.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Expected Cleanup

Delete or shrink:

- `hako_llvmc_match_userbox_counter_step_chain_micro_seed`
- `hako_llvmc_match_userbox_counter_step_micro_seed`
- `hako_llvmc_match_userbox_point_sum_micro_seed`

Keep emit helpers that are still consumed by metadata routes:

- `hako_llvmc_emit_userbox_counter_step_local_i64_ir`
- `hako_llvmc_emit_userbox_counter_step_micro_ir`
- `hako_llvmc_emit_userbox_point_sum_micro_ir`

## Result

Landed as an extension of
`FunctionMetadata.userbox_known_receiver_method_seed_route`.

- `counter_step_chain_local_i64`, `counter_step_chain_micro`,
  `counter_step_micro`, and `point_sum_micro` are matched in MIR, not in C.
- C consumes the existing known-receiver route, validates declarations /
  thin-entry prerequisites / payload fields, and emits the selected helper.
- `hako_llvmc_ffi_user_box_micro_seed_counter_step_chain_micro.inc` was
  deleted.
- `counter_step_micro` and `point_sum_micro` include files are emitter-only.
- Direct smokes now pin route payload and call subjects instead of exact value
  ids.
- Debt guard baseline moved from `9 files / 99 lines` to
  `6 files / 52 lines`.

The next unrelated exact seed matcher is `array_getset_micro`.

## Verification

```bash
cargo test -q userbox_known_receiver_method_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_user_box_method_known_receiver_min.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_counter_step_contract.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_counter_step_chain_contract.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_direct_emit_user_box_point_sum_contract.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
```
