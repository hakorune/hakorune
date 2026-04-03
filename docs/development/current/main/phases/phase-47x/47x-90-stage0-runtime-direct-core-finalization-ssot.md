---
Status: Active
Date: 2026-04-03
Owner: Codex
Scope: finalize the migration from live VM-backed helper defaults to stage0/runtime direct-core routes.
---

# 47x-90 Stage0/Runtime Direct-Core Finalization SSOT

## Goal

- keep `rust-vm` as proof/oracle/compat keep and stop helper-route defaults from re-growing it into a day-to-day owner
- produce `MIR(JSON v0)` on the `.hako`/stage1 side and hand it to stage0 `hakorune` through direct/core seams
- keep `Program(JSON v0)` explicit compat fallback, not mainline artifact authority

## Current Reading

- direct/core receiver already exists:
  - `src/runner/mod.rs --mir-json-file`
  - `src/runner/core_executor.rs`
- helper-route live defaults still leak VM ownership:
  - `tools/selfhost/lib/selfhost_run_routes.sh`
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

## Ordered Tasks

1. `47xA1 runtime/default contract lock`
   - lock exact day-to-day runtime contract and success criteria
2. `47xA2 stage1 source->MIR contract lock`
   - lock exact producer-side contract for source->MIR handoff
3. `47xB1 selfhost_run_routes.sh runtime direct-core cutover`
   - remove day-to-day runtime default dependence on `--backend vm`
4. `47xB2 run.sh explicit vm compat mode lock`
   - keep VM route explicit-only on the facade
5. `47xC1 stage0_capture_route.rs non-VM builder add`
   - keep capture plumbing neutral and move route policy into selectable builders
6. `47xC2 stage_a_route.rs source->MIR first switch`
   - move Stage-A first path to direct MIR capture
7. `47xC3 stage_a_compat_bridge.rs explicit Program(JSON) fallback shrink`
   - keep Program(JSON v0) bridge explicit compat only
8. `47xD1 selfhost_build_stageb.sh default-caller drain`
   - stop default Stage-B paths from rediscovering `run_stageb_compiler_vm.sh`
9. `47xD2 run_stageb_compiler_vm.sh proof-only local keep`
   - keep the script but localize it to proof-only callers
10. `47xE1 proof / closeout`
   - prove direct/core defaults and hand off cleanly

## Acceptance

- `tools/selfhost/run.sh --runtime` day-to-day default no longer executes `--backend vm`
- Stage-A mainline uses source->MIR first
- `Program(JSON v0)` survives only as explicit compat fallback
- `run_stageb_compiler_vm.sh` is no longer a default caller dependency
- `cargo check --bin hakorune` remains green
