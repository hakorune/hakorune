---
Status: Complete
Scope: planner-required expansion (loop_simple_while / loop_continue_only / loop_true_early_exit; legacy Pattern1/4/5 labels are traceability-only)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29bk/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29bl: planner-required expansion (loop_simple_while / loop_continue_only / loop_true_early_exit)

## Goal

loop_simple_while / loop_continue_only / loop_true_early_exit で、strict/dev gate において HAKO_JOINIR_PLANNER_REQUIRED=1 を有効にしても
planner-first が通る状態に拡張する。release 既定は不変、JoinIR regression gate は常に緑維持。

## Non-goals

- 既定挙動の変更
- by-name 分岐
- silent fallback の復活

## Plan (P0-P3)

- P0: docs-first（phase doc + gate SSOT）
- P1: 対象選定（loop_simple_while / loop_continue_only / loop_true_early_exit を各1本）
- P2: gate 追加（planner-required pack）
- P3: closeout

## Target set (loop_simple_while / loop_continue_only / loop_true_early_exit)

- loop_simple_while: phase29ap_stringutils_join_min_vm（理由: StringUtils 系の代表で回帰価値が高い）
- loop_continue_only: loop_continue_only_vm（compat stem: `phase29ap_pattern4_continue_min_vm`; 理由: continue 経路の代表で制御系の回帰価値が高い）
- loop_true_early_exit: phase286_pattern5_break_min_vm（理由: break 経路の代表で制御系の回帰価値が高い。fixture stem は legacy token）

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/core_loop_routes_planner_required_pack_vm.sh`

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/core_loop_routes_planner_required_pack_vm.sh` -> RC=0

## Policy

- HAKO_JOINIR_PLANNER_REQUIRED=1 は strict/dev gate のみで使用（既定OFF）
- planner miss は Freeze、silent fallback は禁止
- stdout が SSOT。exit code が 0-255 に丸められる場合は allow_rc を使う

P2 note: pattern5_break_min は stdout なし、RC=3 のため allow_rc を使用（cases では __EMPTY__ で表現）。
Status note: core_loop_routes_planner_required_pack_vm + phase29ae_regression_pack_vm が緑（post-change）。

P2 note: pattern5_break_min は stdout なし、RC=3 のため allow_rc を使用。
Status note: core_loop_routes_planner_required_pack_vm + phase29ae_regression_pack_vm が緑（post-change）。
