---
Status: Complete
Scope: planner-required expansion (bool_predicate_scan / accum_const_loop)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29bn/README.md
---

# Phase 29bo: planner-required expansion (bool_predicate_scan / accum_const_loop)

## Goal

- bool_predicate_scan / accum_const_loop の代表ケースで、strict/dev gate において `HAKO_JOINIR_PLANNER_REQUIRED=1` を有効にしても planner-first が通る状態にする。
- 既定挙動は不変（release default unchanged）。
- JoinIR regression pack（Phase 29ae）と dev gate v2 を常に緑維持。

## Non-goals

- by-name 分岐
- silent fallback の復活
- 既定挙動の変更

## Plan (P0-P3)

- P0: docs-first（phase doc + gate SSOT）
- P1: Target selection（bool_predicate_scan / accum_const_loop の代表ケースを 1 本ずつ選定）
- P2: planner-required pack 追加 → 実行 → 29ae/dev gate v2 green 確認
- P3: closeout（post-change green + SSOT更新）

## Target (P1)

- BoolPredicateScan route: `phase269_p0_pattern8_frag_min_vm`（legacy fixture pin token; fixture: `apps/tests/phase269_p0_pattern8_frag_min.hako`）
  - 理由: bool_predicate_scan 系の最小POC、planner-required の代表として固定できる
- AccumConstLoop route: `phase286_pattern9_frag_poc_vm`（legacy fixture pin token; fixture: `apps/tests/phase286_pattern9_frag_poc.hako`）
  - 理由: accum_const_loop 系の最小POC、planner-required の代表として固定できる

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_pattern8_9_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_dev_gate_v3_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29bo_planner_required_dev_gate_v3_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0

## Policy

- stdout が SSOT。必要なら case-by-case allow_rc を使う（0-255 丸めを扱う）。
- `HAKO_JOINIR_PLANNER_REQUIRED=1` は strict/dev gate のみで使用（既定OFF）。

Status note: `phase29bo_planner_required_pattern8_9_pack_vm.sh` + `phase29bo_planner_required_dev_gate_v3_vm.sh` + `phase29ae_regression_pack_vm.sh` are green.
