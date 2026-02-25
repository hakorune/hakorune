---
Status: Active
Scope: code（仕様不変、Feature合成の最小ステップ）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P11: ExitMapFacts “presence” を ExitUsageFacts から埋める（保守的、仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: ExitMapFacts を “存在集合” として最小で埋める（対応付け/CFGはやらない）

## Objective

- `LoopFeatureFacts.exit_map` を `None` 固定から前進させ、**presence だけ**を埋める
  - break/continue/return が観測されたら、その kind を `kinds_present` に追加
- 依然として “どの出口がどこへ” は扱わない（それは CorePlan/Frag の領域）

## Non-goals

- 既存ルーティング/観測/エラー文字列の変更
- `Ok(None)` gate の変更
- ExitMap の対応付け（join/edge/exit block 特定）
- cleanup/value_join の実装

## Implementation

Update:
- `src/mir/builder/control_flow/plan/facts/feature_facts.rs`

### Step 1: ExitMapFacts を構築して `exit_map: Some(...)` にする

In `try_extract_loop_feature_facts()`:
- `exit_usage` を先に抽出（既存）
- `kinds_present` を `BTreeSet` で作る
  - `has_return` → insert Return
  - `has_break` → insert Break
  - `has_continue` → insert Continue
- `exit_map = if kinds_present.is_empty() { None } else { Some(ExitMapFacts { kinds_present }) }`
- `LoopFeatureFacts { exit_usage, exit_map, value_join: None, cleanup: None }`

### Step 2: unit tests を更新/追加

Update tests in `feature_facts.rs`:
- break/continue/return が立つケースで `exit_map` が `Some` になり、`kinds_present` が3種を含む
- nested loop だけのケースは `exit_map == None`（外側に影響しない）を維持

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m \"phase29an(p11): populate exitmap presence from exit usage\"`

