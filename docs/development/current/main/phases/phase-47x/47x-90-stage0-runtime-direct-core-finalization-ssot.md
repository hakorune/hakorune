---
Status: Active
Date: 2026-04-04
Owner: Codex
Scope: finalize the migration from live VM-backed helper defaults to stage0/runtime direct-core routes.
---

# 47x-90 Stage0/Runtime Direct-Core Finalization SSOT

## Goal

- keep `rust-vm` as proof/oracle/compat keep and stop helper-route defaults from re-growing it into a day-to-day owner
- produce `MIR(JSON v0)` on the `.hako`/stage1 side and hand it to stage0 `hakorune` through direct/core seams
- keep `Program(JSON v0)` explicit compat fallback, not mainline artifact authority

## Runtime Contract (A1)

- public day-to-day runtime entry is `tools/selfhost/run.sh --runtime`
- default runtime mode is `exe`
- `stage-a` is explicit compat-only and is not the day-to-day owner boundary
- `--backend vm` is kept as a helper/internal detail until later tasks drain it from default callers
- `run.sh --direct` stays proof-oriented and is not the default runtime boundary
- the A1 lock is about fixing these boundaries before helper-default bodies are changed

## Current Reading

- direct/core receiver already exists:
  - `src/runner/mod.rs --mir-json-file`
  - `src/runner/core_executor.rs`
- runtime temp-MIR default is now cut over:
  - `tools/selfhost/lib/selfhost_run_routes.sh` uses the temp-MIR handoff body for runtime `exe` by default
  - `stage-a` stays explicit compat-only and still uses the legacy VM path until `47xB3`
- non-VM builder now exists:
  - `src/runner/modes/common_util/selfhost/stage0_capture_route.rs` has the staged non-VM builder body for the upcoming Stage-A source->MIR cutover
- helper-route live defaults still leak VM ownership:
  - `src/runner/modes/common_util/selfhost/stage0_capture_route.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_route.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `tools/selfhost/lib/selfhost_build_stageb.sh`
- explicit proof-only keep already exists:
  - `tools/selfhost/run_stageb_compiler_vm.sh`

## Migration Rule

- remove live helper-route defaults before shrinking the remaining VM core tail again
- use `.hako` / stage1 source->MIR production where practical
- hand off to stage0 `hakorune` via `--mir-json-file` / `core_executor`
- keep `Program(JSON v0)` compat-only and non-growing
- keep proof-only VM gates explicit and opt-in
- preserve the user-facing `run.sh --runtime` contract while moving the implementation under it

## Ordered Tasks

1. `47xA1 runtime/default contract lock`
   - lock exact day-to-day runtime contract, route tags, and success criteria
2. `47xA2 stage1 source->MIR contract lock`
   - lock exact producer-side contract for source text -> validated MIR(JSON v0)
3. `47xA3 Stage-A direct/core contract lock`
   - lock Stage-A as source->MIR first and Program(JSON v0) explicit compat fallback
4. `47xB1 selfhost_run_routes.sh runtime temp-MIR handoff helper` (landed)
   - add route body that materializes MIR temp state and hands off through `--mir-json-file`
5. `47xB2 selfhost_run_routes.sh runtime default cutover` (landed)
   - remove day-to-day runtime default dependence on `--backend vm`
6. `47xB3 run.sh explicit vm compat mode lock` (landed)
   - keep VM route explicit-only on the facade and stop silent rediscovery
7. `47xC1 stage0_capture_route.rs non-VM builder add` (landed)
   - keep capture plumbing neutral and move route policy into selectable builders
8. `47xC2 stage_a_route.rs source->MIR first switch` (active)
   - move Stage-A first path to direct MIR capture/build
9. `47xC3 stage_a_compat_bridge.rs explicit Program(JSON) fallback shrink`
   - keep Program(JSON v0) bridge explicit compat only
10. `47xD1 selfhost_build_stageb.sh MIR mainline artifact contract lock`
   - define exact Stage-B mainline artifact contract before draining old callers
11. `47xD2 selfhost_build_stageb.sh default-caller drain`
   - stop default Stage-B paths from rediscovering `run_stageb_compiler_vm.sh`
12. `47xD3 run_stageb_compiler_vm.sh proof-only local keep`
   - keep the script but localize it to proof-only callers
13. `47xE1 proof / closeout`
   - prove direct/core defaults and hand off cleanly

## Acceptance

- `tools/selfhost/run.sh --runtime` day-to-day default no longer executes `--backend vm`
- Stage-A mainline uses source->MIR first
- `Program(JSON v0)` survives only as explicit compat fallback
- `run_stageb_compiler_vm.sh` is no longer a default caller dependency
- `cargo check --bin hakorune` remains green
