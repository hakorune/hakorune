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

## 73xA1 Evidence Result

- repro confirmed for both:
  - `lang/src/runner/stage1_cli_env.hako`
  - `lang/src/runner/stage1_cli.hako`
- current red:
  - `Unexpected token BOX, expected LBRACE`
  - merged origin: `lang/src/compiler/build/build_box.hako:4`
- preserved green:
  - `bash tools/selfhost/mainline/stage1_mainline_smoke.sh` PASS

## 73xA2 Target Ranking

1. selfhost-first merge/parser contract around `lang.compiler.build.build_box`
2. focused source fix in `lang/src/compiler/build/build_box.hako` if the contract issue collapses to file-local structure
3. route wrapper/tooling only if the repro stops pointing at merged `BuildBox`

## 73xA2 Read

- the file-context repro fails even in the reduced `env_source_only` case
- this lowers the probability that `stage1_cli_env.hako` or `stage1_cli.hako` themselves are the real owner
- `73xB1` should start from the `build_box` merge/parser seam, not from wrapper churn
