---
Status: Active
Date: 2026-04-04
Scope: remove the remaining live `--backend vm` helper-route defaults from stage0/runtime and keep `rust-vm` on explicit proof/oracle/compat rails.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-46x/README.md
  - docs/development/current/main/phases/phase-46x/46x-90-next-source-lane-selection-ssot.md
  - docs/development/current/main/phases/phase-46x/46x-91-task-board.md
---

# Phase 47x: Stage0/Runtime Direct-Core Finalization

## Goal

- move day-to-day stage0/runtime ownership from `hakorune --backend vm` to `hakorune` direct/core routes
- keep `.hako` source->MIR production on the producer side and `core_executor` on the terminal owner side
- leave `Program(JSON v0)` and VM gates as explicit compat/proof keeps only

## Plain Reading

- `phase-46x` selected this lane because the highest remaining feature tax lives in helper-route defaults, not in the already-shrunk VM core tail.
- the first job is to stop `tools/selfhost/lib/selfhost_run_routes.sh` from using `--backend vm` as the day-to-day runtime default.
- the second job is to make Stage-A source->MIR first, so compat `Program(JSON v0)` stays explicit fallback only.
- the third job is to drain `run_stageb_compiler_vm.sh` out of default Stage-B callers without deleting the proof gate.

## A1 Runtime Contract

- user-facing day-to-day runtime entry is `tools/selfhost/run.sh --runtime`.
- default runtime mode is `exe`; `stage-a` is an explicit compat-only mode.
- the contract for this lane is that future helper changes must preserve the facade shape and success semantics while the implementation is moved under it.
- `--backend vm` is not a day-to-day caller contract for this lane; if it still appears in implementation, it is an intermediate/helper detail to be drained later.
- `run.sh --direct` remains a proof-oriented Stage-B route and is not the default runtime boundary.

## A2 Stage1 Source->MIR Contract

- producer entry is `lang/src/runner/stage1_cli_env.hako` `emit-mir`.
- source text comes from `STAGE1_SOURCE_TEXT`.
- the mainline emit path calls `MirBuilderBox.emit_from_source_v0(...)`.
- `STAGE1_EMIT_MIR_JSON=1` is the stage1 env contract for this route.
- `emit-mir-program` and raw Program(JSON) wrappers remain compat-only keeps.
- legacy routes discovered while locking this lane:
  - `emit program-json` is diagnostics/compat only
  - Program(JSON)->MIR fallback stays explicit-only and non-growing

## A3 Stage-A Direct/Core Contract

- Stage-A first path is direct MIR acceptance when MIR payload already exists.
- `Program(JSON v0)` remains explicit compat fallback and is not the day-to-day owner boundary.
- legacy route discovered while locking this lane:
  - `src/runner/modes/common_util/selfhost/stage_a_route.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - both still route through `stage0_capture_route.rs` while helper defaults are being drained
- this lane is a contract lock, not the helper-default cutover itself
- `47xB1` now adds the opt-in temp-MIR handoff helper under `NYASH_SELFHOST_RUNTIME_TEMP_MIR=1`; `47xB2` will make that body the day-to-day default caller.

## Success Conditions

- runtime contract is explicit and stable before helper-default cutover begins
- runtime default no longer executes `hakorune --backend vm` for day-to-day selfhost runs
- Stage-A first path is source->MIR first and `Program(JSON v0)` remains explicit compat fallback
- default Stage-B callers no longer depend on `run_stageb_compiler_vm.sh`
- `run_stageb_compiler_vm.sh` remains explicit proof-only keep
- `cargo check --bin hakorune` stays green

## Failure Patterns

- moving VM defaults around without actually draining them from helper routes
- widening `core.hako` or raw compat to cover new direct/core work
- treating proof-only VM gates as default producers again
- trying to archive `vm.rs` before helper-route defaults are drained

## Big Tasks

1. `47xA` contract lock
   - `47xA1` runtime/default contract lock
   - `47xA2` stage1 source->MIR contract lock
   - `47xA3` Stage-A direct/core contract lock
2. `47xB` runtime default cutover
   - `47xB1` `selfhost_run_routes.sh` runtime temp-MIR handoff helper
   - `47xB2` `selfhost_run_routes.sh` runtime default cutover
   - `47xB3` `run.sh` explicit vm compat mode lock
3. `47xC` Stage-A source->MIR first
   - `47xC1` `stage0_capture_route.rs` non-VM builder add
   - `47xC2` `stage_a_route.rs` source->MIR first switch
   - `47xC3` `stage_a_compat_bridge.rs` explicit Program(JSON) fallback shrink
4. `47xD` Stage-B caller drain
   - `47xD1` `selfhost_build_stageb.sh` MIR mainline artifact contract lock
   - `47xD2` `selfhost_build_stageb.sh` default-caller drain
   - `47xD3` `run_stageb_compiler_vm.sh` proof-only local keep
5. `47xE` closeout
   - `47xE1` proof / closeout
