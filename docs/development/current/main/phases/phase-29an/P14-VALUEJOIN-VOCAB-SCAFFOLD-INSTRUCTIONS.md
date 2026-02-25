---
Status: Active
Scope: code（仕様不変、ValueJoin語彙の足場）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P14: ValueJoinFacts の語彙足場（型 + canonical projection + verifier、未接続）

Date: 2025-12-29  
Status: Ready for execution  
Scope: join値（post-phi）を Feature として合成するための語彙を先に用意（未接続、仕様不変）

## Objective

- `ValueJoinFacts` を “join 値が必要か（= PHI相当が存在するか）” の **最小語彙**として追加する
- `CanonicalLoopFacts` に projection を追加し、planner/合成が深掘りしない入口を作る
- verifier に “未接続のままでも矛盾しない” 最小不変条件を追加する（fail-fastは debug-only）

## Non-goals

- join入力の対応付け（pred→value mapping）を実装しない（SSOTは `post-phi-final-form-ssot.md` にある）
- CFG/ExitMap との統合（CorePlan合成）は次フェーズ
- 既存ルーティング/候補集合/順序/ログ/エラー文字列の変更
- 新しい Freeze 発火（gate を壊さない）

## Implementation

### Step 1: Facts語彙に ValueJoinFacts を拡張（未接続）

Update:
- `src/mir/builder/control_flow/plan/facts/feature_facts.rs`

Change:
- 既存の `struct ValueJoinFacts;` を “最小情報” を持つ形へ変更
  - 例: `pub struct ValueJoinFacts { pub needed: bool }`（P14では常に `false` を入れる想定）
  - または `enum ValueJoinFacts { Needed }`（P14では `Option<ValueJoinFacts>` を None のまま）

推奨:
- P14では **Optionのまま None**（解析しない）。projectionだけ用意する。

### Step 2: Canonical projection を追加（入口整備）

Update:
- `src/mir/builder/control_flow/plan/normalize/canonicalize.rs`

Add:
- `pub value_join_needed: bool`

Populate:
- `value_join_needed = facts.features.value_join.is_some()`（P14は false のまま）

### Step 3: verifier に最小ルールを追加（debug-only）

Update:
- `src/mir/builder/control_flow/plan/verifier.rs`

Add rule（debug-only / cfg(debug_assertions) でもOK）:
- `value_join_needed == true` の場合は `exit_kinds_present` が empty でない（joinがあるなら制御構造もあるはず、など）
  - ここは弱いルールで良い。目的は “矛盾したfeature” の早期検知。

### Step 4: unit tests（projection固定）

Add tests in `canonicalize.rs`:
- `value_join_needed` が既定で false

Add tests in `verifier.rs`（debug-onlyでOK）:
- 矛盾ケースが panic する（もし debug_assert を入れるなら）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m \"phase29an(p14): add valuejoin facts scaffold and projection\"`

