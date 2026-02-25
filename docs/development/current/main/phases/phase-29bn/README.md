---
Status: Complete
Scope: planner-required Pattern3 (If-Phi) + dev gate expansion
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29bk/README.md
---

# Phase 29bn: planner-required Pattern3 (If‑Phi) + dev gate expansion

## Goal

- Pattern3（If‑Phi）で、strict/dev gate において `HAKO_JOINIR_PLANNER_REQUIRED=1` を有効にしても planner-first が通る状態にする。
- 既定挙動は不変（release default unchanged）。
- JoinIR regression pack（Phase 29ae）を常に緑維持。

## Non-goals

- by-name 分岐
- silent fallback の復活
- 新しい language feature の追加

## Plan (P0-P3)

- P0: docs-first（phase doc + gate SSOT）
- P1: Target selection（Pattern3 代表 1 本）
- P2: Pattern3 required pack gate 追加 → 実行 → 29ae green 確認
- P3: dev default gate を v2 に更新（既存 gate は残す）→ closeout

## Target (P1)

- Pattern3: `phase118_pattern3_if_sum_vm`（fixture: `apps/tests/phase118_pattern3_if_sum_min.hako`）
  - 理由: If‑Phi の最小代表、回帰価値が高い（regression pack に含まれる）

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bn_planner_required_pattern3_pack_vm.sh`（P2 で追加）

Default dev entry: `./tools/smokes/v2/profiles/integration/joinir/phase29bn_planner_required_dev_gate_v2_vm.sh`

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29bn_planner_required_pattern3_pack_vm.sh` -> RC=0

## Policy

- stdout が SSOT。必要なら case-by-case allow_rc を使う（0-255 丸めを扱う）。
- `HAKO_JOINIR_PLANNER_REQUIRED=1` は strict/dev gate のみで使用（既定OFF）。

Status note: phase29bn_planner_required_pattern3_pack_vm / phase29bn_planner_required_dev_gate_v2_vm / phase29ae_regression_pack_vm が緑（post-change）。
