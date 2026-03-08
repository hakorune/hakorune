# Phase 29aj P2: chosen_rule 撤去 + loop_simple_while planner-first（historical label 1, subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: planner outcome の整理 + loop_simple_while facts→planner-first（仕様不変）
Goal: unused outcome field を削除し、loop_simple_while route の extractor 依存を 1 つ減らす

## Objective

- `PlanBuildOutcome::chosen_rule` を撤去し、outcome SSOT を引き締める
- loop_simple_while（historical label 1）を Facts→Planner-first に移す（超保守 subset）
- 既定挙動・観測・エラー文字列は不変（P15 タグ維持）

## Non-goals

- if_phi_join / loop_continue_only / loop_true_early_exit / bool_predicate_scan / accum_const_loop の planner-first 化
- ルール順序 SSOT の CandidateSet への移管
- 新しい env var 追加
- 追加ログ

## Implementation Steps

### Step 1: chosen_rule を削除

Files:
- `src/mir/builder/control_flow/plan/planner/outcome.rs`

やること:
- `PlanBuildOutcome` から `chosen_rule` を削除
- `build_plan_with_facts()` の返却からも削除

### Step 2: loop_simple_while facts を追加（historical label 1 subset）

Files:
- `src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs`
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_types.rs`

Subset 条件:
- condition: `<var> < <int_lit>` のみ
- body: break/continue/return なし
- if-else 禁止（if は許可）
- loop_increment: `var = var + <int_lit>`（PoC は `<int_lit> == 1`）

### Step 3: Planner candidate に loop_simple_while route を追加

Files:
- `src/mir/builder/control_flow/plan/planner/build.rs`

やること:
- `loop_simple_while` が Some のとき対応 route candidate を候補に追加
- unit test を追加

### Step 4: single_planner を historical label 1 planner-first に

Files:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

やること:
- RuleKind に historical label 1 を追加
- planner_opt が loop_simple_while route のとき採用
- それ以外は current extractor lane にフォールバック
- 既存の loop_simple_while guard は維持

### Step 5: docs / CURRENT_TASK 更新

Files:
- `docs/development/current/main/phases/phase-29aj/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Acceptance Criteria

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `rg -n "chosen_rule" src/mir/builder/control_flow/plan/planner/outcome.rs` がヒットしない

## Commit

- `git add -A && git commit -m "phase29aj(p2): planner-first loop simple while subset"`
