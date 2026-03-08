# Phase 29aj P9: Isolate phase286 historical label 9 legacy integration (docs/smokes)

Date: 2025-12-29  
Status: Ready for execution  
Scope: smokes/docs の整理のみ（挙動コードは触らない）  
Goal: JoinIR 回帰ゲートを phase29ae pack に固定し、historical label 9 の archived replay lane を legacy/skip に隔離する

## Objective

- archived replay case の plugins disabled 経路 mismatch を “legacy” として SSOT 化
- JoinIR 回帰は `phase29ae_regression_pack_vm.sh` のみに固定
- 任意の integration filter 実行で誤爆しないように legacy pack を用意する

## Non-goals

- archived replay case の挙動修正
- 新 env var / 新ログ追加
- JoinIR 本体の挙動変更

## Implementation Steps

### Step 1: legacy pack を追加

New:
- legacy pack script（historical replay basename; exact name stays in retirement SSOT）

Behavior:
- historical replay case を `test_skip` で固定
- skip 理由を 1 行で明記（plugins disabled path mismatch）

### Step 2: docs に SSOT を追記

Update:
- `docs/development/current/main/phases/phase-29aj/README.md`
- `docs/development/current/main/phases/phase-29ae/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## Verification

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- retirement SSOT の historical replay pack command（SKIP, historical replay lane）

## Commit

- `git add -A && git commit -m "docs(phase29aj): isolate phase286 pattern9 legacy smoke"`
