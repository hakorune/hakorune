---
Status: Accepted (probing)
Decision: accepted
Date: 2026-03-09
Scope: `stage1-cli` artifact 時に Stage2 build が default bootstrap に落ちる依存を、docs-first で削減する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cf/README.md
  - docs/development/current/main/phases/phase-29cf/P0-VM-FALLBACK-AND-BOOTSTRAP-BOUNDARY-INVENTORY.md
  - docs/development/current/main/phases/phase-29cc/29cc-260-derust-task-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - tools/selfhost_identity_check.sh
  - tools/selfhost/build_stage1.sh
---

# Phase 29cg: Stage2 Bootstrap Reduction

## Goal

`phase-29cf` で `future retire target` に固定した

- `stage1-cli artifact is emit-route entry only; using default bootstrap for Stage2 build`

を、実際に削減するための専用レーンを切る。

## Why a separate phase

1. `phase-29cf` は inventory / keep authority の正本であり、実削減とは責務が違う
2. Stage2 dependency は bootstrap boundary の具体的な 1 blocker なので、単独 phase にした方が追いやすい
3. `VM fallback compat lane` と混ぜると判断がぶれる

## Current Target

- current reduction target:
  - [selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh)
  - artifact-kind=`stage1-cli` のとき、Stage2 build が default bootstrap に落ちる点
- non-goal:
  - `phase-29cf` の caller bucket をやり直すこと
  - `compat-fallback` lane をこの phase で触ること

## Fixed Order

1. Stage2 default-bootstrap dependency を exact owner / exact condition で inventory 化する
2. `stage1-cli` artifact で Stage2 build を stage1-first に寄せるための contract を定義する
3. reduction を 1 箇所だけ切る acceptance を決める

## Contract Snapshot

- `launcher-exe` artifact:
  - raw `NYASH_BIN=<stage1>` bootstrap contract is valid
  - `tools/selfhost_identity_check.sh` already uses this path for Stage2 build
- `stage1-cli` artifact:
  - raw direct invocation is not a valid bootstrap contract for Stage2 build
  - exact probe:
    - `target/selfhost/hakorune.stage1_cli emit program-json ...` returns `97`
    - `target/selfhost/hakorune.stage1_cli --emit-mir-json ...` returns `97`
  - helper-driven stage1 bridge contract is only partially valid
    - `stage1_contract_exec_mode <stage1-cli> emit-program ...` succeeds
    - the emitted Program(JSON v0) for `stage1_cli_env.hako` still has `defs=[]`
    - `stage1_contract_exec_mode <stage1-cli> emit-mir ...` currently fails with `96` and emits an empty MIR file when fed that `defs`-less Program(JSON)
  - experimental bootstrap probe:
    - `build_stage1.sh` can now attempt a `stage1-cli bridge-first` Stage2 build when `NYASH_BIN` itself is a `stage1-cli` artifact
    - `lang/src/runner/stage1_cli_env.hako` has a lower-risk `_find_matching_pair_inline` CFG now, but bridge-first reduction is still blocked earlier
    - current exact blocker in that path is `emit-program` missing helper defs for `stage1_cli_env.hako`; `build_stage1.sh` now fails fast on that exact condition before `emit-mir`
- therefore `phase-29cg` does not treat `stage1-cli` as a drop-in `NYASH_BIN`; it targets a narrower reduction:
  - lift the stage1-bridge helper contract into the Stage2 build path for one reduced case

## Acceptance

- `phase-29cf` とは別に、Stage2 reduction target が独立レーンとして読める
- `tools/selfhost_identity_check.sh` の current fallback note がどの条件で出るか docs から一意に読める
- checklist に `owner / blocker / acceptance / non-goal` が揃っている
- `stage1-cli` reduction target is stated as `bridge-first Stage2 build`, not as `raw NYASH_BIN replacement`
- current G1 route is unchanged until the bridge-first probe clears the stage1-cli registration metadata blocker
