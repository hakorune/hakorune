---
Status: Complete
Scope: Planner-first single-case shift (pattern -> Skeleton+Feature -> CorePlan)
Related:
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/selfhost-coreplan-unblocking-policy.md
---

# Phase 29bh: Planner-first single-case shift

## Goal

JoinIR regression gate を緑維持したまま、代表 1 ケースを “pattern 依存” から
“Skeleton+Feature -> CorePlan 合成” 側へ寄せる（仕様不変、strict/dev で可視化）。

## Non-goals

- 新しい language feature の追加
- 既定挙動や恒常ログの変更
- by-name ハードコードや一時しのぎの分岐

## Plan

- P0: docs-first（phase doc + gate SSOT）
- P1: ターゲット選定（1ケース）
- P2: Planner-first を dev/strict で通す（silent fallback 禁止）
- P3: coverage を 1->2 へ広げるか判断（または closeout）

Target: phase29aq_string_parse_integer_sign_min_vm（理由: 直近の仮説ズレ/境界/回帰価値が高い）
P3 target-2: phase29aq_string_parse_integer_ws_min_vm（理由: parse_integer 系の同族、planner-first coverage を増やす）

## Gate (SSOT)

- `./tools/hako_check_loopless_gate.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bh_planner_first_cases_vm.sh`（`HAKO_JOINIR_PLANNER_REQUIRED=1` で silent fallback 禁止、対象は `tools/smokes/v2/profiles/integration/joinir/planner_first_cases.tsv`）

## Acceptance criteria (RC)

- `./tools/hako_check_loopless_gate.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` -> RC=0
- `./tools/smokes/v2/profiles/integration/joinir/phase29bh_planner_first_cases_vm.sh` -> RC=0

Status note: JoinIR regression pack is green post-change (planner-first tag + planner-required gate).
