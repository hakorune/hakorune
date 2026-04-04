---
Status: Active
Date: 2026-04-04
Scope: harden the current stage1/selfhost mainline owner cluster after rust-vm retirement was frozen as residual explicit keep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/design/frontend-owner-proof-index.md
  - docs/development/current/main/phases/phase-64x/README.md
  - docs/development/current/main/phases/phase-64x/64x-90-next-source-lane-selection-ssot.md
---

# Phase 65x: Stage1/Selfhost Mainline Hardening

## Goal

- tighten the direct `.hako` / Stage1 mainline owner cluster
- keep the current mainline route readable from:
  - `stage1_cli_env.hako`
  - `stage1_contract.sh`
  - `identity_routes.sh`
  - `build_stage1.sh`
- avoid reopening rust-vm broad-owner work

## Big Tasks

1. `65xA1` stage1/selfhost owner inventory lock
2. `65xA2` mainline contract / proof lock
3. `65xB1` runner authority owner cleanup
4. `65xB2` shell contract owner cleanup
5. `65xC1` mainline proof bundle refresh
6. `65xD1` proof / closeout

## Current Read

- `65xA1` landed:
  - stage1/selfhost owner inventory is fixed around the `.hako` authority cluster and shell contract owners
- current front:
  - `65xB1 runner authority owner cleanup`
- current proof read:
  - `bash -n tools/selfhost/lib/stage1_contract.sh tools/selfhost/lib/identity_routes.sh tools/selfhost/build_stage1.sh` PASS
  - `bash tools/selfhost/stage1_mainline_smoke.sh` PASS
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli_env.hako /tmp/stage1_cli_env_probe.mir.json` FAIL
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_probe.mir.json` FAIL
- immediate blocker:
  - selfhost-first parse red at merged `build_box.hako` (`Unexpected token BOX, expected LBRACE`)
- `65xB1` progress:
  - `stage1_cli_env.hako` now delegates emit-mir source/compat choreography to same-file `Stage1EmitMirDispatchBox`
  - next is shell contract owner cleanup while the focused parse red stays tracked
