# Phase 29ak P4: Remove Pattern1 guard from single_planner

Date: 2025-12-29  
Status: Ready for execution  
Scope: single_planner の特例削除（仕様不変）＋ docs 更新  
Goal: Pattern1 guard を planner/facts 側 SSOT に一本化する

## Objective

- single_planner の Pattern1 guard を削除
- fallback 側で ctx による抑制を追加し、契約を二重化
- 観測差分なし（ログ文字列は変えない）

## Non-goals

- rule順序SSOTの CandidateSet 移管
- extractor fallback の削除
- 新 env var / 新ログ追加

## Implementation Steps

### Step 1: Pattern1 guard を削除

Update:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

### Step 2: fallback 側で Pattern1 抑制

Update:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Notes:
- `ctx.pattern_kind != Pattern1SimpleWhile` のとき Pattern1 fallback を `Ok(None)` にする

### Step 3: docs / CURRENT_TASK 更新

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

- `git add -A && git commit -m "phase29ak(p4): remove pattern1 guard from single_planner"`
