---
Status: Complete
Scope: Loop を構造箱化して nested loop を合成可能にする
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/compiler-expressivity-first-policy.md
- docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md
- docs/development/current/main/phases/phase-29br/README.md
---

# Phase 29bs: Loop structural box v1

## Goal

- Loop を “構造箱” として扱い、`Loop.body` に `CorePlan`（木）を持てるようにする。
- `Break/Continue(depth)` を LoopFrame で解決できるようにする（最内=1）。
- nested loop の合成が自然にできる形へ寄せる（release 既定は不変）。

## Non-goals

- 任意ラベル / goto の導入
- raw rewrite（AST/CFG の実行コード書き換え）
- 既定挙動の変更・silent fallback 復活

## Plan

### P0: 設計SSOT

- LoopFrame と depth 解決の SSOT を固定（`coreloop-generic-loop-v0-ssot.md` 参照）。
- 既存の CorePlan/ExitKind 契約と矛盾しない範囲で v1 の箱境界を定義する。
- LoopFrame v1 SSOT（詳細）: `docs/development/current/main/design/coreloop-loopframe-v1-ssot.md`

### P1: Verifier 契約

- `Loop.body` に許可される `CorePlan` 構造を明文化する。
- `Break/Continue(depth)` の不変条件を verifier に固定する（fail-fast）。

### P2: Lowerer（loop frame stack）

- LoopFrame stack を導入し、break/continue の target を stack で解決する。
- 既存の lowerer から v0 と v1 を段階的に切り替えられる構成にする。

### P3: Gates

- 新しい最小 nested loop fixture を追加（strict/dev で planner-required tag が出る）。
- 既存 gate を green 維持（phase29bp / phase29ae）。

## Gate (SSOT)

- Planner-required dev gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
- JoinIR regression pack: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- LoopFrame v1 nested loop gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bs_loopframe_v1_nested_loop_strict_gate_vm.sh`
- Fast iteration gate (default during implementation): `./tools/smokes/v2/profiles/integration/joinir/phase29bs_fast_gate_vm.sh`
  - Default: 29bs only (fast)
  - `--full`: 29bs → 29bp (29bp includes 29ae)
  - `--full` log (latest): `/tmp/phase29bs_fast_gate_1754928_29bp.log` / `/tmp/phase29bs_fast_gate_1754928_29bs.log`
  - `--only {29bs|29bp|29ae}`

## Acceptance criteria (RC)

- `phase29bp_planner_required_dev_gate_v4_vm.sh` が RC=0。
- `phase29ae_regression_pack_vm.sh` が RC=0。
- 最小 nested loop fixture 追加（strict/dev で planner-required tag 出力を確認）。

## Fixture / Gate details

- Fixture: `apps/tests/phase29bs_nested_loop_break_continue_depth_min.hako`
  - Expected stdout: `1`
  - Allowed RC: `0`
- Gate: `./tools/smokes/v2/profiles/integration/joinir/phase29bs_loopframe_v1_nested_loop_strict_gate_vm.sh`
 - Post-change green: `phase29bs_loopframe_v1_nested_loop_strict_gate_vm.sh` + `phase29bp_planner_required_dev_gate_v4_vm.sh` + `phase29ae_regression_pack_vm.sh`

## Contract (minimal set)

- LoopFrame は `break_bb` / `continue_bb` を提供（ID は debug 用のみ）。
- ExitKind は `Break(depth)` / `Continue(depth)` を持つ（`depth=1` が最内）。
- label は AST では許容しても、freeze/verify で depth に解決する（CorePlan に by-name を持ち込まない）。
- Verifier 不変条件（最小）:
  - `depth` は `1..=loop_depth`
  - loop body は CorePlan 木（Seq/If/Loop/Effects/Exit）を持てる（v1）
  - cleanup/defer は ExitKind 経由（将来項目として予定）

## Verifier scope (targets)

- `src/mir/builder/control_flow/plan/verifier.rs`
- `src/mir/builder/control_flow/plan/lowerer.rs`
- `src/mir/builder/control_flow/plan/*`（ExitKind/PlanFrag 周辺）

## Example (why depth is needed)

- outer loop 内で inner loop から `break 2` / `continue 2` 相当を出す必要がある。
- depth が無いと “どの loop に脱出するか” を表現できず、構造箱合成が破綻する。

## Policy reminder

- raw rewrite 禁止（analysis-only view 方針と矛盾しないこと）。
