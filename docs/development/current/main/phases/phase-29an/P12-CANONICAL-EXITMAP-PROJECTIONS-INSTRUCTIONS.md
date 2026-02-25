---
Status: Active
Scope: code（仕様不変、Feature合成の導線整備）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P12: CanonicalLoopFacts に ExitMap projection を追加（仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: ExitMapFacts を normalize 側で “参照しやすい形” に投影する（挙動不変）

## Objective

- `CanonicalLoopFacts` に `exit_kinds_present` の projection を追加し、planner が `facts.facts.features.exit_map` を深掘りしないで済むようにする
- P8（exit_usage invariants）と同様に、将来の Feature 合成で “入口が1箇所” になる土台を整える

## Non-goals

- 既存ルーティング/候補集合/順序/ログ/エラー文字列の変更
- 新しい Freeze 発火（gate を壊さない）
- cleanup/value_join の実装（P13+）

## Implementation

### Step 1: CanonicalLoopFacts を拡張（projection追加）

Update:
- `src/mir/builder/control_flow/plan/normalize/canonicalize.rs`

Add:
- `pub exit_kinds_present: std::collections::BTreeSet<ExitKindFacts>`

Populate:
- `facts.features.exit_map.as_ref().map(|m| m.kinds_present.clone()).unwrap_or_default()`

注意:
- `ExitKindFacts` / `ExitMapFacts` は `src/mir/builder/control_flow/plan/facts/feature_facts.rs` の語彙を使う
- `canonicalize_loop_facts` は pure transform のまま（副作用/ログ禁止）

### Step 2: planner の invariants / gate を projection へ寄せる（挙動不変）

Update:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Change:
- `ExitUsageFacts` を参照している箇所はそのまま維持してよい（P12は projection を増やすだけ）
- ただし `debug_assert_exit_usage_matches_plan()` の引数に `exit_kinds_present` を追加して、
  `exit_usage` と `exit_kinds_present` の整合（presence）を debug-only で確認する
  - `has_break == exit_kinds_present.contains(Break)` 等

### Step 3: unit tests（normalize側で固定）

Add tests in `canonicalize.rs`:
- `exit_kinds_present` が empty のとき empty
- break/continue/return を含む facts で `exit_kinds_present` に3種入る

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p12): project exitmap kinds into canonical facts"`

