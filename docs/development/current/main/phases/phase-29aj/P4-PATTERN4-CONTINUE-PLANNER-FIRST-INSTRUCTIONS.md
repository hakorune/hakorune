# Phase 29aj P4: Pattern4 Continue planner-first via Facts (subset)

Date: 2025-12-29  
Status: Ready for execution  
Scope: Pattern4 facts → planner candidate → single_planner planner-first（仕様不変）  
Goal: Pattern4 を Facts→Planner に乗せ、extractor 依存を 1 本減らす

## Objective

- Pattern4（Loop with Continue）を Facts→Planner 経路に追加
- single_planner は Pattern4 の型一致時のみ planner-first 採用
- 既定挙動・観測・エラー文字列は不変

## Non-goals

- Pattern4 サブセット拡張
- ルール順序 SSOT の CandidateSet 移管
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: Facts SSOT 追加（Pattern4）

Files:
- `src/mir/builder/control_flow/plan/facts/pattern4_continue_facts.rs` (new)

Facts:
- `Pattern4ContinueFacts { loop_var, condition, continue_condition, carrier_updates, loop_increment }`

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
- LoopFacts に `pattern4_continue` を追加
- `try_build_loop_facts()` に抽出を追加
- all-none 判定に `pattern4_continue` を含める

### Step 3: Planner candidate 追加

File:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Changes:
- facts が Some のとき `DomainPlan::Pattern4Continue` を候補に追加
- rule 名は `loop/pattern4_continue`
- unit test 追加

### Step 4: single_planner を Pattern4 planner-first に

File:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Changes:
- RuleKind::Pattern4 を追加
- planner_opt が `Pattern4Continue` のとき採用
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
- `./tools/smokes/v2/run.sh --profile integration --filter "phase286_pattern4_frag_poc"`

## Commit

- `git add -A && git commit -m "phase29aj(p4): planner-first pattern4 continue subset"`

## Next (P5 candidate)

- TBD
