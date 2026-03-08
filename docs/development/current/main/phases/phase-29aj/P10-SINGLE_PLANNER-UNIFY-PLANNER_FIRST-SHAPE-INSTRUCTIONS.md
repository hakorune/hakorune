# Phase 29aj P10: Unify single_planner planner-first shape

Date: 2025-12-29  
Status: Ready for execution  
Scope: single_planner の分岐形を一本化（挙動不変の整形リファクタ）＋ docs 更新  
Goal: 入口の形を最終形に寄せて「もう historical numbered label ごとの if を増やさない」状態にする

## Objective

- historical numbered labels 1/2/3/4/5/6/7/8/9 の順序 SSOT を維持しつつ、全 route family で
  1. planner_opt が該当 DomainPlan なら採用
  2. そうでなければ extractor にフォールバック
  の共通形に統一する
- loop_simple_while guard と bool_predicate_scan static box filter はそのまま維持
- ログ・観測は不変（historical route token 以外増やさない）

## Non-goals

- extractor の削除 / legacy の撤去
- rule order SSOT を CandidateSet に移管
- planner に ctx（pattern_kind/in_static_box）を渡す

## Implementation Steps

### Step 1: RuleKind を historical numbered label 1..9 に統一

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind を historical label 1..9 の列挙に整理
- rules テーブルは name/kind のみ維持
- try_take_planner / fallback_extract の helper を追加して共通形に統一

### Step 2: log_none の意味を SSOT 化

- planner 採用時: log_none=false
- extractor フォールバック時: log_none=true

### Step 3: docs / CURRENT_TASK 更新

Files:
- `docs/development/current/main/phases/phase-29aj/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29aj(p10): unify single_planner planner-first shape"`
