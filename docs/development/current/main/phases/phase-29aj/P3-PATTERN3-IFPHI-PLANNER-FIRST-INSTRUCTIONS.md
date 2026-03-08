# Phase 29aj P3: if_phi_join planner-first via Facts（historical label 3, subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: if_phi_join facts → planner candidate → single_planner planner-first（仕様不変）
Goal: if_phi_join route を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- if_phi_join（historical label 3）を Facts→Planner 経路に追加
- single_planner は if_phi_join の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- loop_continue_only / loop_true_early_exit / bool_predicate_scan / accum_const_loop の planner-first 化
- ルール順序 SSOT の CandidateSet 移管
- Freeze/Fail-Fast の新規導入

## Implementation Steps

### Step 1: Facts SSOT 追加（if_phi_join / historical label 3）

Files:
- `src/mir/builder/control_flow/plan/facts/if_phi_join_facts.rs`

Facts:
- `IfPhiJoinFacts { loop_var, carrier_var, condition, if_condition, then_update, else_update, loop_increment }`

Extraction rules (Ok(None) fallback only):
- condition は比較演算（left が Variable）
- if-else が存在し、then/else 両方で同一変数に代入
- return / break / continue / nested-if は Ok(None)
- loop_increment は `extract_loop_increment_plan` で取れるときのみ

### Step 2: LoopFacts に接続

Files:
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Changes:
- LoopFacts に `if_phi_join` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `if_phi_join` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき if_phi_join route candidate を候補に追加
- historical rule token は inventory lane で追跡
- unit test 追加

### Step 4: single_planner を historical label 3 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind に historical label 3 を追加
- planner_opt が if_phi_join route のとき採用
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

## Commit

- `git add -A && git commit -m "phase29aj(p3): planner-first if-phi join facts subset"`

## Next (P4 candidate)

- loop_continue_only を Facts→Planner-first に寄せる（historical label 4 subset）
