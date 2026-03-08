---
Status: Active
Scope: code（仕様不変、debug-only のSSOT整合）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P8: exit_usage と plan route の整合を debug-only で固定（historical labels 1/2/4/5, 仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: planner の候補生成に “Feature invariants” を足して再発防止（release 既定は不変）

## Objective

- `ExitUsageFacts`（feature）と plan route の関係を **debug_assert** で固定し、将来の Facts 拡張で “矛盾した plan” を作らないようにする
- まずは **確実に対応が取れるものだけ**を対象にする（historical labels 1/2/4/5）

## Non-goals

- 候補の集合/順序/ログ/エラー文字列の変更
- Freeze の新規発火（gate を壊さない）
- historical labels 6/7/8/9/3 など “return を含む可能性がある/不明” のものへ広げる（P8ではやらない）

## Invariants (SSOT)

対象: `build_plan_from_facts_ctx()` が生成する候補（`PlanCandidate.plan`）。

`exit_usage` と plan の対応:
- historical label 1 (`loop_simple_while`) は `exit_usage.has_break == false && has_continue == false && has_return == false`
- historical label 2 (`loop_break`) は `exit_usage.has_break == true`
- historical label 4 (`loop_continue_only`) は `exit_usage.has_continue == true`
- historical label 5 (`loop_true_early_exit`) は `p.exit_kind` に対応するフラグが true
  - Return → `has_return`
  - Break  → `has_break`
  - Continue → `has_continue`

## Implementation

Update:
- `src/mir/builder/control_flow/plan/planner/build.rs`

### Step 1: debug-only ヘルパー追加

`#[cfg(debug_assertions)]` の private 関数で:

- `fn debug_assert_exit_usage_matches_plan(plan: &DomainPlan, exit_usage: &ExitUsageFacts)`
  - 対象4種だけチェックし、それ以外は何もしない

### Step 2: 候補 push 直前/直後にチェック

候補を `candidates.push(...)` する直前（または直後）に:
- `debug_assert_exit_usage_matches_plan(&candidate.plan, &facts.exit_usage);`

注意:
- `facts.exit_usage` は P7 の projection を使う（`facts.facts.features...` を見ない）

### Step 3: unit tests（debug-only 固定）

`build.rs` の `#[cfg(test)]` 内で:
- OK ケース（panicしない）を 2 本（historical labels 1/2）
- NG ケース（`#[should_panic]`）を 1 本（historical label 1 + `has_break=true` など）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p8): debug-assert exit usage invariants for plans"`
