---
Status: Active
Decision: accepted
Date: 2026-04-30
Scope: post-EXE-direct `Program(JSON v0)` keeper closeout after phase-29ci P26.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ci/P26-NYLLVMC-ENTRY-ARGS-BIRTH-PURE-FIRST.md
  - tools/selfhost/README.md
---

# Phase 29cv: Program JSON v0 Keeper Closeout

## Goal

Finish the `Program(JSON v0)` cleanup after the normal
`tools/selfhost/selfhost_build.sh --exe` route stopped producing Stage-B
Program(JSON v0).

This phase is not a broad language or backend redesign. It is a BoxShape
closeout lane: inventory the remaining keepers, move each keeper behind the
right owner, and delete dead helper surface when the repo no longer calls it.

## Current Read

- `phase-29ci` closed the public wrapper and raw-compat caller cleanup, then
  P26 unblocked the direct source -> MIR(JSON) -> ny-llvmc EXE route.
- Normal `selfhost_build.sh --mir`, `--run`, and `--exe` are now direct
  MIR(JSON) routes.
- `selfhost_build.sh --keep-tmp` and `NYASH_SELFHOST_KEEP_RAW=1` are retired
  from the facade; explicit Program(JSON v0) artifact capture now lives in the
  dev probe.
- `Program(JSON v0)` remains as internal/compat/debug infrastructure only.
- The remaining work is keeper classification plus small delete slices, not a
  new acceptance-shape expansion.

## Keeper Buckets

1. Explicit Stage-B artifact diagnostic probe
   - `tools/dev/phase29cv_stageb_artifact_probe.sh`
   - `tools/lib/program_json_v0_compat.sh`
   - Kept for deliberate Program(JSON v0) artifact capture only.
     `selfhost_build.sh` must not own or source this route.
2. Explicit Program(JSON)->MIR bridge probes
   - `tools/selfhost/lib/stageb_program_json_capture.sh`
   - `tools/selfhost/lib/program_json_mir_bridge.sh`
   - `tools/selfhost_exe_stageb.sh`
   - `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
   - Kept for explicit compat/probe work only; not part of
     `selfhost_build.sh` mainline routing.
3. Stage1 contract keepers
   - `tools/selfhost/lib/stage1_contract.sh`
   - `tools/selfhost/compat/run_stage1_cli.sh`
   - Keep only for explicit contract/probe coverage.
   - Old root helpers `tools/stage1_debug.sh` and `tools/stage1_minimal.sh`
     are archived under `tools/archive/legacy-selfhost/stage1-cli/`.
4. JoinIR / MirBuilder fixture keepers
   - `tools/smokes/v2/lib/stageb_helpers.sh`
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*`
   - `tools/smokes/v2/profiles/integration/core/phase2160/stageb_*_canary_vm.sh`
   - Keep while those tests explicitly assert the Program(JSON)->.hako
     MirBuilder contract.
   - Stage-B stdout capture is shared through
     `tools/selfhost/lib/stageb_program_json_capture.sh`.
5. Rust public compat delete-last surface
   - `--emit-program-json-v0`
   - `src/runtime/deprecations.rs`
   - `src/stage1/program_json_v0*`
   - `src/runner/stage1_bridge/**`
   - Delete only after every shell/test keeper has a replacement or an archive
     owner.

## Non-goals

- Do not reintroduce mixed `--run` + Stage-B artifact execution. Use
  fail-fast and ask for either run or artifact diagnostics.
- Do not reintroduce mixed `--exe` + Stage-B artifact execution. Use direct
  EXE or artifact diagnostics.
- Do not reintroduce mixed `--mir` + Stage-B artifact output. Use direct MIR or
  artifact diagnostics.
- Do not reintroduce `selfhost_build.sh --keep-tmp` or
  `NYASH_SELFHOST_KEEP_RAW=1` as facade artifact routes. Use
  `tools/dev/phase29cv_stageb_artifact_probe.sh` for explicit diagnostics.
- Do not expand ny-llvmc pure-first acceptance shapes here.
- Do not revive `--hako-emit-program-json` or other retired public aliases.
- Do not treat fixture-only Program(JSON) producers as day-to-day bootstrap
  route authority.

## Acceptance

Each card in this phase should keep this minimum proof bundle green unless the
card narrows it further:

```bash
bash tools/checks/current_state_pointer_guard.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
git diff --check
```
