---
Status: Accepted (queued)
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` closeout-ready 後に、Rust-owned `Program(JSON v0)` bootstrap boundary の本体 retirement を 1 owner ずつ進める separate phase pointer。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs
  - src/runner/stage1_bridge/program_json/mod.rs
---

# Phase 29cj: Rust-Owned Program JSON v0 Retirement Wave

## Goal

`phase-29ci` で caller/delete-order を closeout-ready に固定したあと、

- `build surrogate keep`
- `future-retire bridge`

の Rust-owned buckets を 1 owner-local wave ずつ薄くして、
`Program(JSON v0)` bootstrap boundary の本体 retirement を進める。

この phase は helper/smoke-tail collapse を再実行する場所ではない。
shared helper / smoke-tail 側は `phase-29ci` で closeout-ready に固定し、
ここでは Rust-owned boundary 本体だけを主語にする。

## Entry Conditions

1. `phase-29ci` が closeout-ready
   - shared helper / smoke-tail collapse is documented
   - remaining explicit registry keep is direct-lower probe only
2. proof bundle is still green
   - `bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
   - `bash tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build`
3. no new `.hako` workaround or shell-contract widening is introduced

## Fixed Order

1. `build surrogate keep`
   - [crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs)
2. `future-retire bridge`
   - [src/runner/stage1_bridge/program_json/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/mod.rs)
   - [src/runner/stage1_bridge/program_json_entry/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/mod.rs)
3. only after Rust-owned buckets shrink, reconsider boundary deletion itself

## Non-goals

- reopening `phase-29ci` helper/smoke-tail collapse
- pulling `registry_optin_method_arraymap_direct_canary_vm.sh` into helper retirement
- widening `.hako` live/bootstrap caller contracts
- mixing authority migration back into `phase-29ch`

## Immediate Next

1. treat `build_surrogate.rs` as near thin floor unless an exact disappearing leaf is obvious first
2. continue `future-retire bridge` retirement inside the inner bridge cluster only
   - `program_json/` is now `mod.rs` + `read_input.rs` + `writeback.rs`
   - `program_json_entry/` is now `mod.rs` + `request.rs`
3. do not take `program_json_entry/request.rs` next unless bridge-local-only leaves are truly exhausted
4. keep the direct-lower probe as explicit evidence until one Rust-owned bucket actually disappears
5. while this phase keeps the Rust-owned retirement order, do not confuse it with the primary pure-`.hako` blocker
   - the real current blocker is now the exact `Program(JSON v0) -> MIR(JSON)` lowering owner in `src/host_providers/mir_builder/lowering/program_json.rs`
   - pinned live callers:
     - `src/host_providers/mir_builder/user_box_decls.rs`
     - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
     - `src/runtime/mirbuilder_emit.rs`
   - therefore, a future authority-removal slice should narrow those callers before broad cleanup elsewhere
6. exact next ladder after `authority.rs` retirement:
   - landed: `src/host_providers/mir_builder/user_box_decls.rs::source_to_mir_json_with_user_box_decls(...)` is retired
   - next target `src/host_providers/mir_builder/lowering/program_json.rs::lower_program_json_to_module(...)`
   - keep `src/stage1/program_json_v0/authority.rs` frozen as the strict source-authority core while those host-provider slices are still live

## Retreat Finding

- `phase-29ci` already closed the helper-side collapse, so further progress now depends on Rust-owned buckets moving, not on more shell cleanup
- `registry_optin_method_arraymap_direct_canary_vm.sh` is no longer “cleanup debt”; it is an explicit probe keep and should stay outside the shared-helper accounting
- the first productive slice already removed the shared route-table keep by moving surrogate route matching into `build_surrogate.rs`; current review treats that bucket as near thin floor rather than the next automatic shrink target
- `future-retire bridge` is now smaller on both sides: `program_json/emit_payload.rs`, `program_json/pipeline.rs`, and `program_json_entry/exit.rs` are gone, so the remaining inner bridge leaves are concentrated in `program_json/mod.rs` and `program_json_entry/request.rs`
- because `program_json_entry/request.rs` still touches env alias precedence and outer-caller-facing request extraction, it is not the default next slice; prefer bridge-local-only collapse before touching that contract leaf
- current authority is now exact enough to avoid hand-wavy blocker accounting: `src/host_providers/mir_builder/user_box_decls.rs` owns the shared source-route handoff, `src/host_providers/mir_builder/lowering/program_json.rs` owns `Program(JSON v0) -> MIR(JSON)`, and `src/stage1/program_json_v0/authority.rs` remains the strict source-authority owner behind them
- worker order decision is now pinned: retire the dedicated `authority.rs` adapter and then move `src/host_providers/mir_builder/user_box_decls.rs` before reopening the kernel Program(JSON) route; treat the kernel route as near thin floor unless an exact disappearing leaf appears
- the test-only transient `(Program JSON, MIR JSON)` tuple helper still lives only in the `src/host_providers/mir_builder.rs` façade test surface
- the dedicated `src/host_providers/mir_builder/authority.rs` adapter is gone, and the extra `user_box_decls.rs::source_to_mir_json_with_user_box_decls(...)` leaf is gone too; live source-route callers now enter through `src/host_providers/mir_builder.rs::source_to_mir_json(...)` and use `user_box_decls.rs` only for shared Program(JSON) shaping
- worker audit also raised the next non-Rust wave order after the current Rust-owned front: `lang/src/mir/builder/MirBuilderBox.hako` first, then runner owners `lang/src/runner/{stage1_cli_env.hako,stage1_cli.hako,launcher.hako}`, with shared producer `lang/src/compiler/build/build_box.hako` immediately behind that same wave; touching `build_box.hako` before those owner-local callers would be the highest-blast-radius move
- the kernel `emit_from_program_json_v0` / `emit_from_source_v0` pair now also shares same-file gate/decode/freeze helpers, so the remaining kernel work is explicitly thin-floor support code rather than a fresh authority-removal front
- the nearby future-retire bridge shim is now split out to `src/stage1/program_json_v0/bridge_shim.rs`, so `src/stage1/program_json_v0/authority.rs` no longer mixes bridge-specific error wrapping with strict source authority
- the next pure-`.hako-only` removal wave should not start by shaving `build_surrogate.rs` more; it should start when the live caller trio of `lowering/program_json.rs` can shrink
- runtime/plugin `env.mirbuilder.emit` is now concentrated in `src/runtime/mirbuilder_emit.rs`; `extern_provider.rs` and `plugin_loader_v2/enabled/extern_functions.rs` are thin callers, and `calls/global.rs` no longer owns a separate direct lowering branch
- runtime/plugin `env.mirbuilder.emit` also no longer counts as a live caller of `src/host_providers/mir_builder/lowering/program_json.rs`; that helper now lowers through `runner::json_v0_bridge::parse_json_v0_to_module_with_imports(...)` and reuses only shared MIR(JSON) emission
- worker audit agreed the safest next Rust-owned slice was the kernel/plugin Program(JSON) route in `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`; that narrowing is now landed, and the remaining kernel-side leaf is no longer the local `user_box_decls` splice because that responsibility now lives in shared owner `src/host_providers/mir_builder/user_box_decls.rs`
- after this slice, the kernel/plugin Program(JSON) route is close to thin floor: route-local gate/decode/encode remain, but host-provider call selection and `user_box_decls` shaping no longer live there
