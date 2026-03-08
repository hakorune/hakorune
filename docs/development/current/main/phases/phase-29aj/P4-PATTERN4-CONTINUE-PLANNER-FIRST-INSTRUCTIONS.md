# Phase 29aj P4: loop_continue_only planner-first via Facts（historical label 4, subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: loop_continue_only facts → planner candidate → single_planner planner-first（仕様不変）
Goal: loop_continue_only route を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- loop_continue_only（historical label 4）を Facts→Planner 経路に追加
- single_planner は loop_continue_only の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- loop_continue_only サブセット拡張
- ルール順序 SSOT の CandidateSet 移管
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: Facts SSOT 追加（loop_continue_only / historical label 4）

Files:
- `src/mir/builder/control_flow/plan/facts/loop_continue_only_facts.rs`

Facts:
- `LoopContinueOnlyFacts { loop_var, condition, continue_condition, carrier_updates, loop_increment }`

Extraction rules (Ok(None) fallback only):
- condition は `<var> < <int_lit>` のみ
- continue が 1 つ以上（recursive）
- break がない
- continue 条件は `if (COND) { continue }` を 1 つ見つける（else 付きは Ok(None)）
- carrier_updates は `var = var + X` のみ（loop_var は除外）
- loop_increment は `extract_loop_increment_plan` で取れるときのみ

Unit tests:
- success / break / continue 無し

### Step 2: LoopFacts に接続

Files:
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Changes:
- LoopFacts に `loop_continue_only` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `loop_continue_only` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき loop_continue_only route candidate を候補に追加
- historical rule token は inventory lane で追跡
- unit test 追加

### Step 4: single_planner を historical label 4 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind に historical label 4 を追加
- planner_opt が loop_continue_only route のとき採用
- それ以外は extractor へフォールバック

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
- `./tools/smokes/v2/run.sh --profile integration --filter "loop_continue_only"`（historical replay token は inventory lane）

## Commit

- `git add -A && git commit -m "phase29aj(p4): planner-first loop continue only subset"`

## Next (P5 candidate)

- TBD
