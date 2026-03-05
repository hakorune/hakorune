---
Status: Complete
Scope: planner-required packs as a single default dev gate
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29bi/README.md
- docs/development/current/main/phases/phase-29bj/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29bk: planner-required packs as a single default dev gate

## Goal

Pattern2 + Scan/Split の planner-required pack を 1 コマンドで回せる dev gate に統合する。
release 既定は不変、JoinIR regression gate は常に緑維持。

## Non-goals

- 既定挙動の変更
- by-name 分岐
- silent fallback の復活

## Plan (P0-P2)

- P0: docs-first（phase doc + gate SSOT）
- P1: 対象 gate と入口を確定
- P2: 統合 runner 追加

## Target gates

- `./tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bj_planner_required_scan_split_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bk_planner_required_dev_gate_vm.sh`

Default dev entry: `./tools/smokes/v2/profiles/integration/joinir/phase29bk_planner_required_dev_gate_vm.sh`

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29bk_planner_required_dev_gate_vm.sh` -> RC=0

## Policy

- 既存の pack gate は作り直さない（統合 runner から呼ぶだけ）
- planner-required は strict/dev gate のみで使用（既定OFF）
- 失敗時は最後のログ位置を出して再実行を容易にする

Status note: `phase29bk_planner_required_dev_gate_vm.sh` green (includes Pattern2/6/7 required packs + JoinIR regression pack; post-change).
