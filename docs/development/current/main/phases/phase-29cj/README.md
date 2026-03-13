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
     - `src/host_providers/mir_builder/authority.rs`
     - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
     - `src/backend/mir_interpreter/handlers/extern_provider.rs`
   - therefore, a future authority-removal slice should narrow those callers before broad cleanup elsewhere

## Retreat Finding

- `phase-29ci` already closed the helper-side collapse, so further progress now depends on Rust-owned buckets moving, not on more shell cleanup
- `registry_optin_method_arraymap_direct_canary_vm.sh` is no longer “cleanup debt”; it is an explicit probe keep and should stay outside the shared-helper accounting
- the first productive slice already removed the shared route-table keep by moving surrogate route matching into `build_surrogate.rs`; current review treats that bucket as near thin floor rather than the next automatic shrink target
- `future-retire bridge` is now smaller on both sides: `program_json/emit_payload.rs`, `program_json/pipeline.rs`, and `program_json_entry/exit.rs` are gone, so the remaining inner bridge leaves are concentrated in `program_json/mod.rs` and `program_json_entry/request.rs`
- because `program_json_entry/request.rs` still touches env alias precedence and outer-caller-facing request extraction, it is not the default next slice; prefer bridge-local-only collapse before touching that contract leaf
- current authority is now exact enough to avoid hand-wavy blocker accounting: `src/host_providers/mir_builder/authority.rs` owns `source -> Program(JSON v0)`, `src/host_providers/mir_builder/lowering/program_json.rs` owns `Program(JSON v0) -> MIR(JSON)`, and `src/stage1/program_json_v0/authority.rs` remains the strict source-authority owner behind them
- the next pure-`.hako-only` removal wave should not start by shaving `build_surrogate.rs` more; it should start when the live caller trio of `lowering/program_json.rs` can shrink
- runtime `env.mirbuilder.emit` is now concentrated in `src/backend/mir_interpreter/handlers/extern_provider.rs`; `calls/global.rs` no longer owns a separate direct lowering branch, so future runtime-side narrowing can stay on one owner
