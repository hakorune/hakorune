---
Status: Active
Scope: code（仕様不変、ルーティング変更なし）
Related:
- docs/development/current/main/phases/phase-29am/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29am P0: CorePlan lowerer/verifier “穴埋め” (If/Exit)

Date: 2025-12-29  
Status: Ready for execution  
Scope: `CorePlan::If` / `CorePlan::Exit` の lowerer/verifier を最小実装（仕様不変）

## Objective

- `CorePlan` を “構造SSOT” として使っていくために、lowerer が `CorePlan::If` / `CorePlan::Exit` を処理できる状態にする
- 既存の JoinIR/PlanFrag のルーティング順序・観測・エラー文字列（実行経路）は変えない
- CorePlan 単体テストで仕様を固定し、将来の合成（Skeleton+Feature）に備える

## Non-goals

- Facts/Planner/Normalizer の挙動変更（どのプランを採用するかは変えない）
- 既存プログラムの意味論変更（quick / joinir regression gate は不変）
- 新 env var 追加
- 大規模リファクタ（最小差分）

## Implementation (critical order)

### Step 1: lowerer の未対応分岐を削る（If/Exit）

Target:
- `src/mir/builder/control_flow/plan/lowerer.rs`

Work:
- `CorePlan::If(CoreIfPlan)` を lower できるようにする
- `CorePlan::Exit(CoreExitPlan)` を lower できるようにする

Rules (SSOT):
- effect/再順序は `docs/development/current/main/design/effect-classification-ssot.md` に従う
- cleanup/exit 境界は `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md` に従う
- 既存の emit/merge は再解析しない（CorePlan 入力だけで完結）

Notes:
- まず “standalone” 実装でよい（Loop の内部に入る複合ケースは P1+ に回す）
- 可能なら `PlanVerifier` が落ちる条件（不変条件違反）を先に追加して fail-fast に寄せる

### Step 2: verifier の不変条件を最小追加

Target:
- `src/mir/builder/control_flow/plan/verifier.rs`

Work:
- `CoreIfPlan` の最小不変条件（then/else の空、Exit の位置、など）を追加
- `CoreExitPlan` の最小不変条件（loop 外の Break/Continue 禁止、など）を追加

### Step 3: 単体テストで仕様固定

Target (例):
- `src/mir/builder/control_flow/plan/verifier.rs` の tests
- または `src/mir/builder/control_flow/plan/lowerer.rs` の tests（既存の流儀に合わせる）

Tests (minimum):
- `CorePlan::Exit(Return)` が lower/verify できる
- `CorePlan::If` が lower/verify できる（then/else の片側だけ、両側、など）
- 不変条件違反は fail-fast（Err）になる

## Verification (required)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A`
- `git commit -m "phase29am(p0): implement coreplan if/exit lowering"`

