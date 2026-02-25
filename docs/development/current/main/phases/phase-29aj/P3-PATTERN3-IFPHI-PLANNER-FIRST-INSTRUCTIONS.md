# Phase 29aj P3: Pattern3 (If-Phi) planner-first via Facts (subset)

Date: 2025-12-29  
Status: Ready for execution  
Scope: Pattern3 facts → planner candidate → single_planner planner-first（仕様不変）  
Goal: Pattern3 を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- Pattern3（Loop with If-Else PHI）を Facts→Planner 経路に追加
- single_planner は Pattern3 の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- Pattern4/5/8/9 の planner-first 化
- ルール順序 SSOT の CandidateSet 移管
- Freeze/Fail-Fast の新規導入

## Implementation Steps

### Step 1: Facts SSOT 追加（Pattern3）

Files:
- `src/mir/builder/control_flow/plan/facts/pattern3_ifphi_facts.rs` (new)

Facts:
- `Pattern3IfPhiFacts { loop_var, carrier_var, condition, if_condition, then_update, else_update, loop_increment }`

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
- LoopFacts に `pattern3_ifphi` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `pattern3_ifphi` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき `DomainPlan::Pattern3IfPhi` を候補に追加
- rule 名は `loop/pattern3_ifphi`
- unit test 追加

### Step 4: single_planner を Pattern3 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind::Pattern3 を追加
- planner_opt が `Pattern3IfPhi` のとき採用
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

- `git add -A && git commit -m "phase29aj(p3): planner-first pattern3 if-phi facts subset"`

## Next (P4 candidate)

- Pattern4（Continue）を Facts→Planner-first に寄せる（subset）
