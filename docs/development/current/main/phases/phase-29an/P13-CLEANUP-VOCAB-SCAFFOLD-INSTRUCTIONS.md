---
Status: Active
Scope: code（仕様不変、cleanup語彙の足場）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P13: CleanupFacts の語彙足場（型 + canonical projection、未接続）

Date: 2025-12-29  
Status: Ready for execution  
Scope: cleanup を Feature として合成するための語彙を先に用意する（未接続、仕様不変）

## Objective

- `CleanupFacts` を “ExitKind 単位の cleanup がある/ない” の語彙として追加する
- `CanonicalLoopFacts` に projection を追加し、planner/合成が深掘りしない入口を作る

## Non-goals

- cleanup の対応付け（どの値を release する等）を実装しない
- exit のブロック/CFG対応付けを実装しない
- 既存ルーティング/候補集合/順序/ログ/エラー文字列の変更
- 新しい Freeze 発火（gate を壊さない）

## Implementation

### Step 1: Facts語彙に CleanupKind/CleanupFacts を追加（未接続）

Update:
- `src/mir/builder/control_flow/plan/facts/feature_facts.rs`

Add:
- `enum CleanupKindFacts { Return, Break, Continue }`
- `struct CleanupFacts { kinds_present: std::collections::BTreeSet<CleanupKindFacts> }`

Wire (still conservative):
- `LoopFeatureFacts.cleanup: Option<CleanupFacts>` は既定 `None` のまま（P13は解析しない）

### Step 2: Canonical projection を追加（入口整備）

Update:
- `src/mir/builder/control_flow/plan/normalize/canonicalize.rs`

Add:
- `pub cleanup_kinds_present: std::collections::BTreeSet<CleanupKindFacts>`

Populate:
- `facts.features.cleanup.as_ref().map(|c| c.kinds_present.clone()).unwrap_or_default()`

### Step 3: unit tests（型/既定値固定）

Add tests:
- cleanup が None のとき `cleanup_kinds_present` は empty
- `canonicalize_loop_facts` が `cleanup_kinds_present` を必ず生成する（projectionの存在固定）

### Step 4: planner 側の debug-only 整合（任意・最小）

Update (optional):
- `src/mir/builder/control_flow/plan/planner/build.rs`

Add debug_assert:
- “cleanup があるなら、対応する exit_kind が exit_kinds_present に居る” などの弱い整合（未接続のため基本は空）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m \"phase29an(p13): add cleanup facts scaffold and projections\"`

