# Phase 29aj P5: Pattern5 infinite early-exit planner-first via Facts (subset)

Date: 2025-12-29  
Status: Ready for execution  
Scope: Pattern5 facts → planner candidate → single_planner planner-first（仕様不変）  
Goal: Pattern5 を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- Pattern5（loop(true) + early exit）を Facts→Planner 経路に追加
- single_planner は Pattern5 の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- Pattern5 サブセット拡張（複数exit/複雑条件/複数carrier）
- ルール順序 SSOT の CandidateSet 移管
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: Facts SSOT 追加（Pattern5）

Files:
- `src/mir/builder/control_flow/plan/facts/pattern5_infinite_early_exit_facts.rs` (new)

Facts:
- `Pattern5InfiniteEarlyExitFacts { loop_var, exit_kind, exit_condition, exit_value, carrier_var, carrier_update, loop_increment }`

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
- LoopFacts に `pattern5_infinite_early_exit` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `pattern5_infinite_early_exit` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき `DomainPlan::Pattern5InfiniteEarlyExit` を候補に追加
- rule 名は `loop/pattern5_infinite_early_exit`
- unit test 追加

### Step 4: single_planner を Pattern5 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind::Pattern5 を追加
- planner_opt が `Pattern5InfiniteEarlyExit` のとき採用
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

- `git add -A && git commit -m "phase29aj(p5): planner-first pattern5 infinite early-exit subset"`

## Next (P6 candidate)

- TBD
