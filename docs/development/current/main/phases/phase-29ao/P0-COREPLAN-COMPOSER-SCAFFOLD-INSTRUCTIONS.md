---
Status: Active
Scope: code（仕様不変、未接続の足場）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P0: CorePlan composer scaffold（CanonicalLoopFacts→CorePlan、未接続）

Date: 2025-12-29  
Status: Ready for execution  
Scope: “CorePlan 合成” の入口を作るだけ（未接続＝仕様不変）

## Objective

- `CanonicalLoopFacts`（projection済み）から `CorePlan` を合成する入口関数を 1 箇所に作る
- 以後の拡張は “合成ロジック” をここに閉じ込め、emit/merge は再解析しない方針を守る

## Non-goals

- 既存ルーティング/観測/エラー文字列の変更
- `Ok(None)` gate を減らす（P0 は常に Ok(None) でも良い）
- Frag/ExitMap の wire を入れる（P1+）
- 新 env var / 恒常ログ追加

## Implementation

### Step 1: composer モジュール追加（leaf）

Add:
- `src/mir/builder/control_flow/plan/composer/mod.rs`

API（案）:

- `pub(in crate::mir::builder) fn try_compose_core_plan_from_canonical_facts(facts: &CanonicalLoopFacts) -> Result<Option<CorePlan>, Freeze>`

P0の実装:
- 必ず `Ok(None)` を返す（未接続の足場）
- `facts.skeleton_kind == Loop` であることを `debug_assert!`（P6と整合）

### Step 2: plan/mod.rs に module 登録（未使用）

Update:
- `src/mir/builder/control_flow/plan/mod.rs`

Add:
- `pub(in crate::mir::builder) mod composer;`

### Step 3: unit test（存在確認）

Add minimal test:
- ダミーの `CanonicalLoopFacts` を作って `Ok(None)` が返る（パニックしない）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m \"phase29ao(p0): add coreplan composer scaffold\"`

