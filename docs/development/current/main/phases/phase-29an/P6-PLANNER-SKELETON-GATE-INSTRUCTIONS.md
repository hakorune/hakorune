---
Status: Active
Scope: code（仕様不変、Plannerの骨格前提を明文化）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P6: Planner に Skeleton gate を追加（Loop 以外は Ok(None)、仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: Planner の前提（LoopFacts は LoopSkeleton）をコードで明文化して、将来の Region plan へ繋ぐ

## Objective

- `build_plan_from_facts_ctx()` の入口で `skeleton.kind` を確認し、Loop 以外は **`Ok(None)`** へ倒す（fallback を維持）
- 現状は `LoopFacts` が loop 起点なので実質的に到達しないが、**SSOT としての境界**をコードに固定する

## Non-goals

- 候補の集合/順序/ログ/エラー文字列の変更
- 新しい Freeze を追加して gate を壊す
- Skeleton の “一意化” を実装する（P7 以降）

## Implementation

Update:
- `src/mir/builder/control_flow/plan/planner/build.rs`

### Step 1: skeleton gate を追加

`build_plan_from_facts_ctx()` 冒頭で:

- `use crate::mir::builder::control_flow::plan::facts::skeleton_facts::SkeletonKind;` を追加
- `match facts.facts.skeleton.kind { SkeletonKind::Loop => {}, _ => return Ok(None) }`

注意:
- `Ok(None)` に倒す理由: 既定挙動を変えず legacy fallback を維持するため
- 将来 “対象っぽい” を Freeze にしたくなった場合は P7（Skeleton一意化）で taxonomy に従って実装する

### Step 2: unit test を 1 本追加（SSOT固定）

`build.rs` の `#[cfg(test)]` 内で、手動で `LoopFacts` を構築して `skeleton.kind=If2` を入れたケースを作り、
`build_plan_from_facts_ctx(...)=Ok(None)` を確認する。

注意:
- ここは “仕様不変” のためのガードテスト。実行導線では発火しない想定。

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p6): gate planner by skeleton kind"`

