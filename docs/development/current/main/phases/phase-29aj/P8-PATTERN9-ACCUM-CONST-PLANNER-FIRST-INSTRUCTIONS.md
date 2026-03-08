# Phase 29aj P8: accum_const_loop planner-first via Facts（historical label 9, subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: accum_const_loop facts → planner candidate → single_planner planner-first（仕様不変）
Goal: accum_const_loop route を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- accum_const_loop（historical label 9）を LoopFacts に追加し、planner が該当 route candidate を返せるようにする
- single_planner は accum_const_loop の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- accum_const_loop サブセット拡張（acc_update の複雑化、複数acc、step=-1 等）
- ルール順序 SSOT の CandidateSet 移管
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: Facts SSOT 追加（accum_const_loop / historical label 9）

Files:
- `src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs`

Facts:
- `AccumConstLoopFacts { loop_var, acc_var, condition, acc_update, loop_increment }`

Extraction rules (Ok(None) fallback only):
- condition は `<loop_var> < <int_lit>` のみ
- body に break/continue/if-else が無い
- acc update は `acc = acc + <int_lit>` のみ
- loop_increment は `extract_loop_increment_plan(body, loop_var)` が取れる
- acc_var != loop_var

Unit tests:
- const accumulation 成功
- break/continue/if-else 混入は Ok(None)
- `sum = sum + i` は Ok(None)

### Step 2: LoopFacts に接続

Files:
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Changes:
- LoopFacts に `accum_const_loop` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `accum_const_loop` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき accum_const_loop route candidate を候補に追加
- historical rule token は inventory lane で追跡
- unit test 追加

### Step 4: single_planner を historical label 9 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind に historical label 9 を追加
- planner_opt が accum_const_loop route のとき採用
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
- `./tools/smokes/v2/run.sh --profile integration --filter "accum_const_loop"` (任意)

## Commit

- `git add -A && git commit -m "phase29aj(p8): planner-first accum const loop subset"`

## Next (P9 candidate)

- TBD
