# Phase 29ak P5: Planner candidate ctx gate SSOT

Date: 2025-12-29  
Status: Ready for execution  
Scope: planner の CandidateSet 生成を ctx-aware にして、single_planner を薄くする（仕様不変）  
Goal: ctx gate を planner に一本化し、fallback 側の特例を最小化する

## Objective

- Pattern1/8 の候補抑制を planner 側に集約する
- single_planner の Pattern1 fallback 抑制を撤去
- 挙動は不変（候補を作らないだけで legacy fallback は従来どおり）

## Non-goals

- CandidateSet の順序SSOT化
- extractor fallback の削除
- 新 env var / 新ログ追加

## Risk / Gotchas

- Pattern1 の fallback 抑制を撤去すると、Pattern1 extractor が nested loop を誤マッチし得る（phase1883 の Pattern6NestedLoopMinimal が plan 側に吸われる）。Pattern1 extractor 側で nested loop を `Ok(None)` に倒すこと。

## Implementation Steps

### Step 1: build_plan_from_facts を ctx-aware に

Update:
- `src/mir/builder/control_flow/plan/planner/build.rs`
- `src/mir/builder/control_flow/plan/planner/mod.rs`
- `src/mir/builder/control_flow/plan/planner/outcome.rs`
- `src/mir/builder/control_flow/plan/planner/context.rs`

Notes:
- `build_plan_from_facts_ctx(ctx, facts)` を追加
- 既存 `build_plan_from_facts` は legacy ctx に委譲
- Candidate push 直前で ctx gate を適用

### Step 2: outcome の ctx 版入口を更新

Update:
- `src/mir/builder/control_flow/plan/planner/outcome.rs`

### Step 3: single_planner の Pattern1 fallback 抑制を撤去

Update:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

### Step 4: docs / CURRENT_TASK 更新

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

- `git add -A && git commit -m "phase29ak(p5): ssot ctx gating in planner candidates"`
