# Phase 29ak P3: Remove Pattern8 static box filter from single_planner

Date: 2025-12-29  
Status: Ready for execution  
Scope: single_planner の特例削除（挙動不変）＋ docs 更新  
Goal: Pattern8 static box filter を planner/facts 側 SSOT に一本化する

## Objective

- single_planner の Pattern8 static box reject 分岐を削除
- debug ログは SSOT ではない（差分対象外）ことを明記

## Non-goals

- Pattern1 guard の削除
- rule順序SSOTの CandidateSet 移管
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: single_planner の特例削除

Update:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Remove:
- Pattern8 static box filter ブロックと debug ログ

### Step 2: docs / CURRENT_TASK 更新

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

- `git add -A && git commit -m "phase29ak(p3): remove pattern8 static box filter from single_planner"`
