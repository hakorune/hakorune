---
Status: Active
Scope: code（仕様不変、normalize SSOT の段階強化）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P7: CanonicalLoopFacts に skeleton/exit_usage の projection を追加（仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: `canonicalize_loop_facts` を “将来の骨格/特徴合成” に耐える形へ寄せる（挙動不変）

## Objective

- `CanonicalLoopFacts` に `skeleton_kind` / `exit_usage` の **投影(projection)** を追加する
  - planner が `facts.facts.*` の深掘りをしなくても “骨格/特徴” の入口が 1 箇所に揃う
- 既存の候補生成・順序・ログ・エラー文字列は不変

## Non-goals

- 新しい Freeze の発火（gate を壊さない）
- 候補の増減や優先順序変更
- facts の `Ok(None)` gate の変更

## Implementation

### Step 1: CanonicalLoopFacts を拡張（projection追加）

Update:
- `src/mir/builder/control_flow/plan/normalize/canonicalize.rs`

Change:
- `struct CanonicalLoopFacts { pub facts: LoopFacts }` に以下を追加
  - `pub skeleton_kind: SkeletonKind`
  - `pub exit_usage: ExitUsageFacts`

Populate:
- `skeleton_kind = facts.skeleton.kind`
- `exit_usage = facts.features.exit_usage.clone()`

注意:
- `LoopFacts` 側で skeleton/features は必須なので `Option` 剥がしは不要
- `canonicalize_loop_facts` は pure transform のまま（副作用/ログ禁止）

### Step 2: planner の skeleton gate を projection へ寄せる（挙動不変）

Update:
- `src/mir/builder/control_flow/plan/planner/build.rs`

Change:
- `facts.facts.skeleton.kind` 参照を `facts.skeleton_kind` に置き換えるだけ（ロジック不変）

### Step 3: unit tests（最低限）

Add tests in `canonicalize.rs`:
- `canonicalize_loop_facts` で `skeleton_kind` が `Loop` になる
- `exit_usage` が Facts 由来で投影されている（例: break/continue/return が立つ）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p7): add canonical projections for skeleton/features"`

