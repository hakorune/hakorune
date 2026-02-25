# Phase 29ak P2: Gate Pattern8 facts by static box ctx

Date: 2025-12-29  
Status: Ready for execution  
Scope: PlannerContext.in_static_box を使って Pattern8 facts を抑制（挙動・ログ不変）  
Goal: single_planner の特例フィルタ責務を planner 側へ移す

## Objective

- in_static_box == true のとき Pattern8 facts 抽出を行わない
- single_planner 側の static box reject 分岐は安全策として残す
- ログ差分は出さない（planner 側は無言で Ok(None)）

## Non-goals

- Pattern8 static box reject 分岐の削除
- CandidateSet への順序移管
- 新 env var 追加

## Implementation Steps

### Step 1: facts 入口で Pattern8 を抑制

Update:
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Notes:
- `try_build_loop_facts_with_ctx` で `ctx.in_static_box` を参照し、Pattern8 抽出関数を呼ばない

### Step 2: planner outcome 側は ctx 版を継続使用

Update:
- `src/mir/builder/control_flow/plan/planner/outcome.rs`

### Step 3: unit test 追加（facts）

- in_static_box=true で Pattern8 facts が None になることを固定

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

- `git add -A && git commit -m "phase29ak(p2): gate pattern8 facts by static box ctx"`
