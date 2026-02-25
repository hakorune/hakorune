# Phase 29ak P1: Guard Pattern1 facts via PlannerContext

Date: 2025-12-29  
Status: Ready for execution  
Scope: planner の Facts 入口で pattern_kind を参照して早期 Ok(None)（保守的）  
Goal: single_planner の guard 責務を減らし、planner 側へ寄せる第一歩にする（仕様不変）

## Objective

- PlannerContext.pattern_kind を使い、Pattern1 以外のループで Pattern1 facts 抽出を行わない
- 挙動・ログは不変（single_planner 側の guard は維持）

## Non-goals

- Pattern8 static box filter の移動
- CandidateSet の順序SSOT化
- extractor fallback の撤去

## Implementation Steps

### Step 1: loop_facts に guard を追加

Update:
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Add:
- `try_build_loop_facts_with_ctx(ctx, condition, body)` を新設
- ctx.pattern_kind が Pattern1 以外のとき pattern1_simplewhile 抽出を抑制

### Step 2: planner outcome から ctx 版を使う

Update:
- `src/mir/builder/control_flow/plan/planner/outcome.rs`

Notes:
- `build_plan_with_facts_ctx` は `try_build_loop_facts_with_ctx` を使う
- `build_plan_with_facts` は既存挙動のまま

### Step 3: single_planner guard は残す

Update:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Notes:
- planner 側に移した旨を docs に明記（guard は冗長だが安全策として維持）

### Step 4: テストを追加

Add:
- pattern_kind != Pattern1 のとき pattern1 facts が None になること
- pattern_kind == Pattern1 のとき従来通り抽出できること

### Step 5: docs / CURRENT_TASK 更新

Update:
- `docs/development/current/main/phases/phase-29ak/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29ak(p1): guard pattern1 facts via planner context"`
