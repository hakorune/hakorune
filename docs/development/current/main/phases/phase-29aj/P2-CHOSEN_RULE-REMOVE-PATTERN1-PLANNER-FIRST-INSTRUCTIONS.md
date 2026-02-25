# Phase 29aj P2: chosen_rule 撤去 + Pattern1 planner-first（subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: planner outcome の整理 + Pattern1 Facts→Planner-first（仕様不変）  
Goal: unused outcome field を削除し、Pattern1 の extractor 依存を 1 つ減らす

## Objective

- `PlanBuildOutcome::chosen_rule` を撤去し、outcome SSOT を引き締める
- Pattern1（SimpleWhile）を Facts→Planner-first に移す（超保守 subset）
- 既定挙動・観測・エラー文字列は不変（P15 タグ維持）

## Non-goals

- Pattern3/4/5/8/9 の planner-first 化
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

### Step 2: Pattern1 Facts を追加（subset）

Files:
- `src/mir/builder/control_flow/plan/facts/pattern1_simplewhile_facts.rs`（新規）
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Subset 条件:
- condition: `<var> < <int_lit>` のみ
- body: break/continue/return なし
- if-else 禁止（if は許可）
- loop_increment: `var = var + <int_lit>`（PoC は `<int_lit> == 1`）

### Step 3: Planner candidate に Pattern1 を追加

Files:
- `src/mir/builder/control_flow/plan/planner/build.rs`

やること:
- `pattern1_simplewhile` が Some のとき `DomainPlan::Pattern1SimpleWhile` を候補に追加
- unit test を追加

### Step 4: single_planner を Pattern1 planner-first に

Files:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

やること:
- RuleKind に Pattern1 を追加
- planner_opt が `Pattern1SimpleWhile` のとき採用
- それ以外は `extract_pattern1_plan()` にフォールバック
- 既存の Pattern1 guard は維持

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

- `git add -A && git commit -m "phase29aj(p2): planner-first pattern1 simplewhile subset"`
