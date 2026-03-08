# Phase 29aj P5: loop_true_early_exit planner-first via Facts（historical label 5, subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: loop_true_early_exit facts → planner candidate → single_planner planner-first（仕様不変）
Goal: loop_true_early_exit route を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- loop_true_early_exit（historical label 5）を Facts→Planner 経路に追加
- single_planner は loop_true_early_exit の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- loop_true_early_exit サブセット拡張（複数exit/複雑条件/複数carrier）
- ルール順序 SSOT の CandidateSet 移管
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: Facts SSOT 追加（loop_true_early_exit / historical label 5）

Files:
- `src/mir/builder/control_flow/plan/facts/loop_true_early_exit_facts.rs`

Facts:
- `LoopTrueEarlyExitFacts { loop_var, exit_kind, exit_condition, exit_value, carrier_var, carrier_update, loop_increment }`

Extraction rules (Ok(None) fallback only):
- condition は `loop(true)` のみ
- body 先頭が `if (cond) { return <expr> }` か `if (cond) { break }`（else 無し）
- then_body は単一要素のみ（Return / Break のみ）
- Break 版は carrier 1 個だけ許可し、`var = var + ...` 形のみ
- loop_increment は `extract_loop_increment_plan(body, loop_var)` が取れる場合のみ

Unit tests:
- return 版 / break 版の success
- else 付き / increment 無し → Ok(None)

### Step 2: LoopFacts に接続

Files:
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Changes:
- LoopFacts に `loop_true_early_exit` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `loop_true_early_exit` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき loop_true_early_exit route candidate を候補に追加
- historical rule token は inventory lane で追跡
- unit test 追加

### Step 4: single_planner を historical label 5 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind に historical label 5 を追加
- planner_opt が loop_true_early_exit route のとき採用
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
- `./tools/smokes/v2/run.sh --profile integration --filter "phase143_"`

## Commit

- `git add -A && git commit -m "phase29aj(p5): planner-first loop true early-exit subset"`

## Next (P6 candidate)

- TBD
