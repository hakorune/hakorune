---
Status: Complete
Scope: planner-required scan_with_init / split_scan（legacy labels `6/7` are traceability-only）
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29bi/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29bj: planner-required scan_with_init / split_scan

## Goal

Scan/Split（ScanWithInit / SplitScan）で、strict/dev gate において
HAKO_JOINIR_PLANNER_REQUIRED=1 を有効にしても planner-first が通る状態にする。
release 既定は不変、JoinIR regression gate は常に緑維持。

## Non-goals

- 既定挙動の変更
- by-name 分岐
- silent fallback の復活

## Plan (P0-P3)

- P0: docs-first（phase doc + gate SSOT）
- P1: 対象選定（scan_with_init / split_scan を各1本）
- P2: gate 追加（planner-required pack）
- P3: closeout

## Target set (scan_with_init / split_scan)

- scan_with_init: phase29aq_string_index_of_min_vm（理由: scan_with_init route の代表で回帰価値が高い）
- split_scan: phase29aq_string_split_min_vm（理由: split_scan route の代表で回帰価値が高い）

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/scan_split_planner_required_pack_vm.sh`（exact compat wrapper is tracked in `joinir-smoke-legacy-stem-retirement-ssot.md`）

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/scan_split_planner_required_pack_vm.sh` -> RC=0

## Policy

- HAKO_JOINIR_PLANNER_REQUIRED=1 は strict/dev gate のみで使用（既定OFF）
- planner miss は Freeze、silent fallback は禁止
- stdout が SSOT。exit code が 0-255 に丸められる場合は allow_rc を使う

P2 note: index_of_min は stdout=2、RC=2（exit code 丸めのため allow_rc を使用）。
Status note: scan_split_planner_required_pack_vm + phase29ae_regression_pack_vm が緑（post-change）。
