---
Status: Active
Date: 2026-04-04
Scope: track the focused `emit_mir_mainline` parse red that still points at `build_box.hako`.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-65x/README.md
  - docs/development/current/main/phases/phase-72x/README.md
---

# 73x-90 emit_mir_mainline Blocker Follow-Up SSOT

## Intent

- fix the tracked focused blocker without reopening broad stage1/selfhost ownership work
- keep the fix narrow and source-backed
- preserve existing mainline green checks while restoring the focused probe

## Known Red

- probe:
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli_env.hako /tmp/stage1_cli_env_probe.mir.json`
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_probe.mir.json`
- current origin:
  - `lang/src/compiler/build/build_box.hako`
- current error:
  - `Unexpected token BOX, expected LBRACE`

## Decision Rule

- reproduce first
- rank the narrowest plausible source fix
- prove against both the focused probe and existing stage1/selfhost mainline checks
