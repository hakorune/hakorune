---
Status: Active
Scope: code（仕様不変、Facts SSOT の引き締め）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P4: LoopFacts で Skeleton/Feature を必須にする（SSOT引き締め、仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: Facts の型を “使う側が迷わない” 形に寄せる（挙動不変）

## Objective

- `LoopFacts` が `Ok(Some(_))` になった時点で、**Skeleton と Feature は必ず揃っている** という SSOT をコードの型で固定する
- P5（Skeleton 一意化 / Feature 合成）に向けて、`Option` 剥がしの散在を防ぐ

## Non-goals

- ルーティング順序・観測・エラー文字列の変更
- `Ok(None)` の gate を緩める（features/skeleton だけで Some にしない）
- 新 env var / 恒常ログ追加

## Implementation

### Step 1: LoopFacts のフィールドを必須化

Update:
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`

Change:
- `pub skeleton: Option<SkeletonFacts>` → `pub skeleton: SkeletonFacts`
- `pub features: Option<LoopFeatureFacts>` → `pub features: LoopFeatureFacts`

Rules:
- `has_any == false` のときは従来どおり `Ok(None)`（ここは絶対に変えない）
- `has_any == true` のときは必ず skeleton/features を構築する
  - `try_extract_loop_skeleton_facts(...)` が `None` を返したら `Freeze::bug(...)`（到達してはいけない）

### Step 2: skeleton/features の構築を “has_any の後” に固定

`try_build_loop_facts_inner()` 内で:
- route-specific facts 抽出 → `has_any` 判定 → `skeleton` / `features` 抽出 → `Ok(Some(LoopFacts{...}))`

### Step 3: planner/build.rs の unit test を機械的に更新

Update:
- `src/mir/builder/control_flow/plan/planner/build.rs`

`LoopFacts { ... }` のテスト構築に:
- `skeleton: SkeletonFacts { kind: SkeletonKind::Loop }`
- `features: LoopFeatureFacts::default()`
を追加する（既存テストの意図は変えない）。

### Step 4: “SSOT不変条件” の最小テストを 1 本追加

Add in `src/mir/builder/control_flow/plan/facts/loop_facts.rs`:
- route-specific facts が 1 つでも取れた場合、`LoopFacts.skeleton.kind == Loop` が成立する
- `LoopFacts.features.exit_usage` がデフォルトでも存在する（`Option` ではないことの固定）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p4): require skeleton/features in loop facts"`
