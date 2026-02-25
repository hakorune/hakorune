---
Status: Complete
Scope: planner-first required (Pattern2 small set)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29bh/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29bi: planner-first required (Pattern2 small set)

## Goal

Pattern2 の小集合で、strict/dev で HAKO_JOINIR_PLANNER_REQUIRED=1 を有効にしても
planner-first が通る状態を増やす。release 既定は不変、JoinIR regression gate は常に緑維持。

## Non-goals

- release 既定挙動の変更
- silent fallback の復活
- by-name ハードコード

## Plan (P0-P3)

- P0: docs-first（phase doc + gate SSOT）
- P1: 対象選定（Pattern2 small set 4 本）
- P2: gate 追加（planner-required pack）
- P3: closeout

## Target set (Pattern2 small set, final)

- phase29aq_string_parse_integer_min_vm
- phase29aq_string_parse_integer_leading_zero_min_vm（理由: 先頭ゼロの境界で Pattern2 の分岐条件を踏む）
- phase29aq_string_parse_integer_ws_min_vm
- phase29aq_string_parse_integer_sign_min_vm

追加ルール: 1 本追加するたびに「なぜ pattern2 の代表か」を 1 行追記。

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh`

Note: `phase29bi_planner_required_pattern2_pack_vm.sh` は stdout を SSOT とし、exit code は 0-255 に丸められるため case-by-case の allow_rc を使う。

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29bi_planner_required_pattern2_pack_vm.sh` -> RC=0

## Policy

- HAKO_JOINIR_PLANNER_REQUIRED=1 は strict/dev gate のみで使用（既定OFF）
- planner miss は Freeze、silent fallback は禁止
- parse_integer 系は stdout が SSOT。OS exit code は 0-255 に丸められるため、gate は case-by-case で allow_rc を持つ（例: 12345 -> 57）。

Status note: phase29bi_planner_required_pattern2_pack_vm + phase29ae_regression_pack_vm が緑（post-change）。
