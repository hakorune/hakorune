---
Status: Active
Scope: code（仕様不変、Feature合成の足場）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P10: ExitMap FeatureFacts の足場（型だけ先に追加、未接続）

Date: 2025-12-29  
Status: Ready for execution  
Scope: FeatureFacts を “ExitMap/cleanup 合成” に繋ぐ語彙の足場を作る（未接続、仕様不変）

## Objective

- Skeleton+Feature 合成の中核である `ExitMap` を、Facts 側の語彙として先に定義する
- P10 では **型と変換導線の足場だけ**（解析は次の P11 以降）

## Non-goals

- 既存ルーティング/観測/エラー文字列の変更
- `Ok(None)` gate の変更
- exit の対応付け（どの break/continue がどこへ）を推論する
- cleanup/value_join を実装する（P12+）

## Implementation

### Step 1: ExitKind/ExitMapFacts を feature_facts に追加（SSOT語彙）

Update:
- `src/mir/builder/control_flow/plan/facts/feature_facts.rs`

Add types（最小）:
- `enum ExitKindFacts { Return, Break, Continue }`
- `struct ExitMapFacts { kinds_present: std::collections::BTreeSet<ExitKindFacts> }`

Add field:
- `LoopFeatureFacts { exit_map: Option<ExitMapFacts>, ... }`

Populate（P10では未接続）:
- `exit_map: None` を既定

注意:
- 既存の `ExitUsageFacts` はそのまま残す（ExitMapFacts を使うのは P11+）
- `ExitKindFacts` は将来 `Unwind` を追加できる形にする（P12以降）

### Step 2: unit tests（型・既定値の固定）

Update/add tests in `feature_facts.rs`:
- `try_extract_loop_feature_facts()` が `exit_map: None` を返すこと

### Step 3: LoopFacts/CanonicalLoopFacts の projection は触らない（仕様不変）

P10では `canonicalize_loop_facts` の projection は増やさない。
（projection拡張は P11 でまとめてやる）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m \"phase29an(p10): add exitmap feature facts scaffold\"`

