---
Status: Active
Scope: code（仕様不変、未接続のSSOT足場）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P0: SkeletonFacts SSOT（Loop/If/BranchN/StraightLine）

Date: 2025-12-29  
Status: Ready for execution  
Scope: Facts に Skeleton（骨格）を追加する（未接続、仕様不変）

## Objective

- “pattern名で入口分岐” ではなく、**Skeleton → Feature → CorePlan 合成**へ寄せるための Facts SSOT を作る
- planner が “再解析で穴埋め” しないで済むように、骨格の観測/導出を Facts に集約する

## Non-goals

- ルーティング順序・観測・エラー文字列の変更
- 既存の planner/legacy extractor の削除
- 新 env var / 恒常ログ追加
- Freeze を増やして gate を壊す（P0 は Ok(None) へ倒す）

## Implementation

### Step 1: SkeletonFacts 型を追加（SSOT）

Add:
- `src/mir/builder/control_flow/plan/facts/skeleton_facts.rs`

Suggested vocabulary:
- `SkeletonFacts { kind: SkeletonKind, ... }`
- `SkeletonKind::{Loop, If2, BranchN, StraightLine}`

Notes:
- “BranchN” は match/switch 相当の将来枠（P0 は未使用でもOK）
- いまは **観測の器**が目的。derive は最小でよい

### Step 2: LoopFacts に “optional skeleton” を接続（未使用）

Update:
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Rules:
- 既存の pattern facts 抽出は維持
- skeleton は `Ok(Some(_))` のときだけ `LoopFacts` に埋め、既定挙動は変えない

### Step 3: unit tests（最低限）

Add tests:
- “単純な loop(cond) { ... }” で `SkeletonKind::Loop` が取れる
- “直列だけ” は `Ok(None)` または `SkeletonKind::StraightLine`（どちらかに統一してSSOT化）

P0 の方針（推奨）:
- 既存の導線を壊さないため、`StraightLine` は **Ok(None)** に倒す（plan対象外）

## Verification (required)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p0): add skeleton facts ssot (no wiring)"`

