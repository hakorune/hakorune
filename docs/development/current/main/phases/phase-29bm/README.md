---
Status: Complete
Scope: Facts→Planner coverage hardening (planner-required)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29bk/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29bm: Facts→Planner coverage hardening (planner-required)

## Goal

HAKO_JOINIR_PLANNER_REQUIRED=1 を dev/strict gate で使う前提のまま、
planner miss の Freeze を減らす（Facts/Planner が plan を返せる範囲を増やす）。
既存 pack gate と JoinIR regression gate は常に緑維持。

## Non-goals

- release 既定挙動の変更
- by-name 分岐
- silent fallback の復活

## Plan (P0-P3)

- P0: docs-first（phase doc + gate SSOT）
- P1: dev gate で Freeze/タグ欠落のターゲットを 1 本選定
- P2: Facts→Planner coverage を追加（by-name 禁止）
- P3: closeout（post-change green + SSOT更新）

## Target

- Added coverage (scan_with_init route; historical label: Pattern6): `phase29aq_string_last_index_of_min_vm`（理由: index_of と対になる ScanWithInit 代表ケース）

## Status note

- `scan_split_planner_required_pack_vm.sh` / `phase29bk_planner_required_dev_gate_vm.sh` / `phase29ae_regression_pack_vm.sh` are green post-change.

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29bk_planner_required_dev_gate_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0

## Policy

- 既存 pack gate を増やさず、dev gate の FAIL をそのままターゲット化
- planner miss は Freeze、silent fallback は禁止
