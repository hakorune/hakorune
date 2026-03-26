---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P35` after landing the generated selfhost builder runner seam split in `tools/hakorune_emit_mir.sh`; close-sync the broader build-contract helper `tools/selfhost/selfhost_build.sh` against the accepted `phase-29ci` helper-local reading.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P35-BYN-MIN5-EMIT-MIR-SELFHOST-RUNNER-SEAM-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - tools/selfhost/selfhost_build.sh
  - tools/smokes/v2/lib/test_runner.sh
---

# P36: BYN-min5 Selfhost-Build Helper Inventory

## Purpose

- close-sync `tools/selfhost/selfhost_build.sh` with the accepted `phase-29ci` helper-local reading
- keep `tools/smokes/v2/lib/test_runner.sh` out of this slice
- stop helper fan-out under `phase-29cl`; do not open a new `P37` helper bucket unless a fresh exact seam appears

## Current Truth

1. `tools/selfhost/selfhost_build.sh` is a broader build-contract helper, not a narrow direct `MirBuilderBox.emit_from_program_json_v0(...)` pipeline
2. the helper still owns `BuildBox.emit_program_json_v0(...)` keep, Program(JSON)->MIR conversion, Core-direct run, and Program(JSON)->EXE routing
3. the exact helper-local acceptance for this helper already lives in `phase-29ci`:
   - `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
   - `docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md`
4. raw `tools/selfhost/selfhost_build.sh --in ...` whole-script routes remain upstream Stage-B diagnostics, not the helper-local reopen trigger
5. this bucket must stay separate from `tools/smokes/v2/lib/test_runner.sh`, which remains near-thin-floor / monitor-only under the accepted `phase-29ci` scope
6. current judgment: `tools/selfhost/selfhost_build.sh` is monitor-only under the current `phase-29cl` scope, so there is no new exact helper-local execution bucket here

## Next Exact Front

1. none under the current `phase-29cl` helper scope
2. reopen only if a fresh exact helper-local seam appears, or if hard delete / broad internal removal explicitly resumes
