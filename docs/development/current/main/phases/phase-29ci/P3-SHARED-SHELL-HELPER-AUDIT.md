---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` の shared shell helper keep 3本を exact role 付きで audit し、smoke tail / probe keep と分離した delete-order を固定する。
Related:
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - CURRENT_TASK.md
---

# P3 Shared Shell Helper Audit

## Goal

shared shell helper keep として残っている 3 file について、

- 何の contract を持っているか
- どれが最初に audit しやすいか
- smoke tail / diagnostics とどう分離するか

を delete-order の SSOT として固定する。

## Exact Helper Roles

### `tools/hakorune_emit_mir.sh`

- role:
  - explicit dev/helper entry for `.hako -> Program(JSON v0) -> MIR(JSON)` emission
  - direct `MirBuilderBox.emit_from_program_json_v0(...)` helper call
- contract shape:
  - helper-local pipeline script
  - not the shared selfhost build contract
- audit priority:
  - highest in the helper trio
  - owner is narrow and caller surface is explicit

### `tools/selfhost/selfhost_build.sh`

- role:
  - shared selfhost build contract
  - optional `HAKO_USE_BUILDBOX=1` lane still uses `BuildBox.emit_program_json_v0(...)`
- contract shape:
  - build pipeline helper
  - touches build output / stageb command description / raw capture
- audit priority:
  - second
  - broader than `hakorune_emit_mir.sh`, so keep separate

### `tools/smokes/v2/lib/test_runner.sh`

- role:
  - shared smoke/runtime helper
  - fallback/full MirBuilder lane still uses `MirBuilderBox.emit_from_program_json_v0(...)`
- contract shape:
  - common test harness
  - tightly connected to smoke tail callers
- audit priority:
  - last in the helper trio
  - must not be mixed with the helper-local slices above

## Fixed Delete Order

1. audit `tools/hakorune_emit_mir.sh`
2. audit `tools/selfhost/selfhost_build.sh`
3. audit `tools/smokes/v2/lib/test_runner.sh`
4. only then collapse the 43-file smoke tail that depends on the test runner
5. diagnostics/probe keep remains after live/helper caller audit

## Guardrails

- do not audit `tools/smokes/v2/lib/test_runner.sh` in the same patch as `tools/hakorune_emit_mir.sh`
- do not fold the 43-file smoke tail into the helper-trio patch
- keep `tools/selfhost/selfhost_build.sh` separate from `tools/hakorune_emit_mir.sh`; both are shared helpers but have different contracts
- current authority and `.hako` live/bootstrap owners are out of scope here

## Retreat Finding

- helper trio is not homogeneous:
  - `hakorune_emit_mir.sh` is a narrow helper-local pipeline
  - `selfhost_build.sh` is a build contract helper
  - `test_runner.sh` is a shared smoke harness tied to the 43-file tail
- therefore, the safest next helper audit is `tools/hakorune_emit_mir.sh`
- `tools/hakorune_emit_mir.sh` can keep shrinking by localizing its embedded selfhost/provider runner generation; this is helper-local structure work and does not require touching the shared build/test contracts
- `tools/hakorune_emit_mir.sh` still owns `Stage-B Program(JSON) production + imports normalize + Program→MIR fallback funnel`, so the next safe helper-local slice is the Stage-B Program(JSON) production block itself; do not mix that with direct-emit fallback or legacy delegate retirement in the same patch
- `tools/hakorune_emit_mir.sh` now keeps Stage-B Program(JSON) production + imports normalize behind `emit_stageb_program_json_v0()`, so the remaining helper-local funnel is narrower and the next slice can focus on direct-emit fallback or delegate tail in isolation
- `tools/hakorune_emit_mir.sh` now keeps the provider-first delegate funnel behind `emit_mir_json_from_program_json_delegate_chain()`, with the legacy CLI fallback isolated in `try_legacy_program_json_delegate()`, so the remaining helper-local tail is the direct-emit fallback lane rather than mixed delegate wiring
- `tools/hakorune_emit_mir.sh` now also keeps the duplicated Stage-B fail/invalid -> direct MIR emit fallback behind `exit_after_direct_emit_fallback()`, so the script-local fallback funnel is split into exact helper lanes instead of repeated top-level branches
- `tools/smokes/v2/lib/test_runner.sh` should be treated as the bridge between helper keep and smoke tail, not as “just another helper script”
- `tools/selfhost/selfhost_build.sh` now keeps its Stage-B Program(JSON) raw-production split behind `emit_stageb_program_json_raw()`, with `emit_program_json_v0_via_buildbox()` and `emit_program_json_v0_via_stageb_compiler()` isolating the two live lanes; this keeps `HAKO_USE_BUILDBOX=1` as an explicit build-contract keep without leaving the top-level branch duplicated
- `tools/selfhost/selfhost_build.sh` no longer shows the old `hello_simple_llvm` freeze split, and both the helper's default `compiler.hako --stage-b --stage3` lane and the explicit `HAKO_USE_BUILDBOX=1` emit-only keep are healthy again on that fixture (`Extern(log 42) + Return(Int 0)`)
- `tools/selfhost/selfhost_build.sh` now pins that keep behind `buildbox_emit_only_keep_requested()`, so the exact live-contract predicate (`HAKO_USE_BUILDBOX=1` + emit-only + no EXE lane) is SSOT in code as well as docs
- `tools/selfhost/selfhost_build.sh` now also keeps its post-emit raw/extract funnel behind `extract_program_json_v0_from_raw()`, `persist_stageb_raw_snapshot()`, and `exit_after_stageb_emit_failure()`, so build-helper cleanup can talk about exact lanes instead of one long post-emit block
- `tools/selfhost/selfhost_build.sh` now keeps the source-direct `--mir` consumer behind `emit_mir_json_from_source()`, so downstream consumer audit can proceed one lane at a time without mixing `--exe` or `--run`
- `tools/selfhost/selfhost_build.sh` now also keeps the Core-Direct `--run` consumer behind `run_program_json_v0_via_core_direct()`, so the remaining downstream helper-local work is the Program(JSON)->MIR->EXE lane rather than mixed run/EXE cleanup
- `tools/selfhost/selfhost_build.sh` now also keeps the Program(JSON)->MIR->EXE consumer behind `emit_exe_from_program_json_v0()`, so the downstream consumer lanes are all owner-local helpers instead of inline top-level branches
- `tools/selfhost/selfhost_build.sh --mir` is still green on `apps/tests/hello_simple_llvm.hako` because it uses the source-direct route
- `tools/selfhost/selfhost_build.sh --run` is green on the repaired payload
- `tools/selfhost/selfhost_build.sh --exe` is green on that same repaired payload
- for this fixture, `HAKO_USE_BUILDBOX=1` is still an explicit keep contract in code, but it no longer distinguishes success from failure; delete/retire arguments need caller-inventory proof rather than malformed-producer proof from `hello_simple_llvm`
- `tools/smokes/v2/lib/test_runner.sh` is now safe to thin one lane at a time inside `verify_program_via_builder_to_core()`: the full `MirBuilderBox.emit_from_program_json_v0(...)` fallback now lives behind `emit_mir_json_via_full_mirbuilder()`, so the next helper-local tail is the Rust CLI fallback lane rather than the direct full-mirbuilder call itself
- do not mix that `test_runner.sh` lane work with the 43-file smoke tail; the shared harness still stays the owner and the tail remains caller-audit-only
- forced full-mirbuilder canary `tools/smokes/v2/profiles/integration/core/phase2044/mirbuilder_provider_emit_core_exec_canary_vm.sh` is still blocked by `[Phase 88] Ring0Context not initialized`; treat that as a separate runtime/entry blocker, not as a reason to undo or widen the helper-local split

## Immediate Next

1. keep `tools/hakorune_emit_mir.sh` monitor-only while `selfhost_build.sh` downstream audit is active
2. keep `tools/selfhost/selfhost_build.sh` monitor-only unless a new helper-local split inside the already isolated consumer helpers becomes necessary
3. keep `tools/smokes/v2/lib/test_runner.sh` on helper-local slices only: thin `verify_program_via_builder_to_core()` one fallback lane at a time, without touching the smoke tail yet
4. if `phase2044/mirbuilder_provider_emit_core_exec_canary_vm.sh` must go green, handle `[Phase 88] Ring0Context not initialized` as a separate runtime/entry owner and do not mix it into the next delete-order patch
