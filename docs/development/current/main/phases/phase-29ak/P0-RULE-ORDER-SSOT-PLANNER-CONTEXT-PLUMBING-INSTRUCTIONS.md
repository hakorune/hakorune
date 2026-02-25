# Phase 29ak P0: PlanRuleOrder SSOT + PlannerContext plumbing

Date: 2025-12-29  
Status: Ready for execution  
Scope: 構造整備（仕様不変）＋ docs/Now/Backlog/CURRENT_TASK 更新  
Goal: single_planner の順序/名前/ガード SSOT を 1 箇所へ寄せ、planner 側へ ctx を通す土台を作る

## Objective

- ルール順序/表示名の SSOT を新設し、single_planner の手書きテーブルを撤去
- PlannerContext を導入して planner へ ctx を渡せるようにする（P0 では未使用）
- 既定挙動・ログ・エラー文字列は不変

## Non-goals

- CandidateSet へ順序＝優先を移す
- Pattern1 guard / Pattern8 static box filter を planner 側へ移す
- extractor fallback の削除

## Implementation Steps

### Step 1: ルール順序 SSOT を新設

New:
- `src/mir/builder/control_flow/plan/single_planner/rule_order.rs`

Contents:
- `PlanRuleId` と `PLAN_RULE_ORDER`
- `rule_name()` は従来の名前を完全一致で返す

### Step 2: single_planner を SSOT 参照に差し替え

Update:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Notes:
- Pattern1 guard / Pattern8 static box filter / Pattern2 promotion tag を維持
- ループの順序/ログ文言は変えない

### Step 3: PlannerContext の足場（未使用）

New:
- `src/mir/builder/control_flow/plan/planner/context.rs`

Update:
- `src/mir/builder/control_flow/plan/planner/mod.rs`
- `src/mir/builder/control_flow/plan/planner/outcome.rs`
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

Rule:
- P0 では ctx を意思決定に使わない（挙動不変）

### Step 4: docs / CURRENT_TASK 更新

Add:
- `docs/development/current/main/phases/phase-29ak/README.md`

Update:
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29ak(p0): ssot rule order + planner context plumbing"`
