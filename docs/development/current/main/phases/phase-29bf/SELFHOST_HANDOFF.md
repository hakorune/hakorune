# Selfhost handoff — after CorePlan migration

Status: SSOT  
Scope: selfhost 開発へ戻るための入口（CorePlan 移行後）  

## What’s done

- CorePlan 移行の Done criteria は P0 で検証済み:
  - `docs/development/current/main/phases/phase-29bf/README.md`
  - `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
- JoinIR regression gate は SSOT で固定:
  - `docs/development/current/main/phases/phase-29ae/README.md`

## Gate (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Rules (do / don’t)

- release 既定の意味論/恒常ログ/エラー文字列は変えない
- strict/dev の fail-fast は FlowBox スキーマに収束（追加語彙は SSOT 更新が先）
- CorePlan/Composer を触る条件:
  - gate が赤になった時（最小差分で戻す）
  - 新しい stdlib 形状を “subset 追加” で吸う時（docs-first + fixture/smoke）

## Next steps (selfhost)

- 入口は `docs/development/current/main/10-Now.md` を SSOT にする
- 代表 gate が緑のまま selfhost 作業へ戻る（前提が崩れたら CorePlan 側を最小差分で修正）
- selfhost 側で新しい loop 形が出たら facts subset 追加の docs-first から始める

## Pointers

- CorePlan 移行の道筋 SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- FlowBox fallback 観測 SSOT: `docs/development/current/main/design/flowbox-fallback-observability-ssot.md`
- Return-in-loop 最小語彙 SSOT: `docs/development/current/main/design/return-in-loop-minimal-ssot.md`
