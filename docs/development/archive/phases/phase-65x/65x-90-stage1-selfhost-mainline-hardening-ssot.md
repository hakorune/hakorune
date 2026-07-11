---
Status: Landed
Date: 2026-04-04
Scope: harden the current stage1/selfhost mainline owner cluster around `.hako` authority entry and shell contract seams.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/design/frontend-owner-proof-index.md
---

# 65x-90 Stage1/Selfhost Mainline Hardening SSOT

## Intent

- continue the repo's non-vm mainline after the rust-vm corridor
- focus on current authority owners, not on historical keep buckets
- keep `.hako` compiler/mainline movement aligned with current proofs

## Starting Read

- `stage1_cli_env.hako` is the current stage1 env-entry authority cluster
- `stage1_contract.sh` and `identity_routes.sh` own the exact shell contract
- `build_stage1.sh` remains the bootstrap artifact owner around the stage1 mainline path

## Owner Set

- `.hako` authority owners:
  - `lang/src/runner/stage1_cli_env.hako`
  - `lang/src/runner/stage1_cli.hako`
  - `lang/src/runner/launcher.hako`
- shell contract owners:
  - `tools/selfhost/lib/stage1_contract.sh`
  - `tools/selfhost/lib/identity_routes.sh`
  - `tools/selfhost/build_stage1.sh`
- proof owners:
  - `tools/selfhost/stage1_mainline_smoke.sh`
  - `tools/hakorune_emit_mir_mainline.sh`
  - `tools/selfhost/run_lane_a_daily.sh`

## Contract / Proof Lock

- authority contract:
  - `lang/src/runner/stage1_cli_env.hako` is the current stage1 env-entry authority cluster
  - `lang/src/runner/stage1_cli.hako` and `lang/src/runner/launcher.hako` stay as raw/facade keep, not the sole authority
  - `tools/selfhost/lib/stage1_contract.sh` and `tools/selfhost/lib/identity_routes.sh` own the exact shell contract
  - `tools/selfhost/build_stage1.sh` owns Stage1 bootstrap artifact production
- focused proof bundle:
  - `bash -n tools/selfhost/lib/stage1_contract.sh tools/selfhost/lib/identity_routes.sh tools/selfhost/build_stage1.sh` PASS
  - `bash tools/selfhost/stage1_mainline_smoke.sh` PASS
- blocker evidence:
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli_env.hako /tmp/stage1_cli_env_probe.mir.json`
    - FAIL: `Unexpected token BOX, expected LBRACE`
    - merged origin: `lang/src/compiler/build/build_box.hako:4`
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_probe.mir.json`
    - FAIL: same selfhost-first parse red

## Current Judgment

- this lane is justified:
  - current Stage1 mainline smoke is green
  - but focused mainline emit probes still fail inside the `.hako` compiler/build cluster
- therefore the next work should target:
  - runner authority cleanup first
  - shell contract cleanup second
  - proof refresh after that

## 65xB1 Progress

- `stage1_cli_env.hako`
  - emit-mir source/compat choreography moved behind same-file `Stage1EmitMirDispatchBox`
  - `Main` now delegates the emit-mir route split instead of carrying the branch body inline
- blocker status:
  - unchanged
  - focused mainline emit probes still remain red on the selfhost-first parse issue from `build_box.hako`

## 65xB2 Progress

- shell contract owner cleanup:
  - `tools/selfhost/lib/identity_routes.sh` now reuses `stage1_contract_artifact_kind(...)`
  - `tools/selfhost/build_stage1.sh` now reuses `stage1_contract_artifact_kind(...)`
  - duplicate `artifact_kind` readers are removed from shell owner surfaces
- proof:
  - `bash -n tools/selfhost/lib/stage1_contract.sh tools/selfhost/lib/identity_routes.sh tools/selfhost/build_stage1.sh` PASS
  - `bash tools/selfhost/stage1_mainline_smoke.sh` PASS

## 65xC1 Proof Refresh

- stable green bundle:
  - `cargo check --bin hakorune` PASS
  - `bash tools/selfhost/stage1_mainline_smoke.sh` PASS
  - `git diff --check` PASS
- focused blocker rerun:
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli_env.hako /tmp/stage1_cli_env_probe.mir.json`
    - FAIL, unchanged
  - `bash tools/hakorune_emit_mir_mainline.sh lang/src/runner/stage1_cli.hako /tmp/stage1_cli_probe.mir.json`
    - FAIL, unchanged
- blocker remains:
  - selfhost-first parse red at `lang/src/compiler/build/build_box.hako:4`
  - no regression introduced by `65xB1/B2`

## Big Tasks

1. `65xA1` stage1/selfhost owner inventory lock
2. `65xA2` mainline contract / proof lock
3. `65xB1` runner authority owner cleanup
4. `65xB2` shell contract owner cleanup
5. `65xC1` mainline proof bundle refresh
6. `65xD1` proof / closeout

## Handoff

- lane result:
  - stage1/selfhost owner cluster is narrower and better single-sourced than before
  - stable green mainline proof bundle is preserved
  - focused selfhost-first parse blocker remains outside this lane's narrow cleanup scope
- next lane:
  - `phase-66x next source lane selection`
