---
Status: Active
Scope: code（仕様不変、planner内部の段階移行）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P2: Planner を Skeleton→Feature の “段取り” に寄せる（仕様不変）

Date: 2025-12-29  
Status: Ready for execution  
Scope: planner の内部構造だけを Skeleton→Feature の段取りへ寄せる（出力/観測は不変）

## Objective

- `build_plan_from_facts_ctx()` を「complete pattern の羅列」から、**Skeleton inference → Feature inference → CandidateSet finalize** の段取りへ寄せる
- ただし P2 は “段取りの導入” が目的で、**挙動は不変**（候補の集合/順序/ログ/エラー文字列を変えない）

## Non-goals

- 新しい Freeze の追加（gate を壊さない）
- 既存 candidate の増減や優先順序変更
- single_planner 側のルーティング変更

## Implementation（コード）

対象（中心）:
- `src/mir/builder/control_flow/plan/planner/build.rs`

### Step 1: Planner 内に “段取り” 用ヘルパーを追加

`build.rs` 内でOK（新ファイルは任意）。

提案:
- `fn infer_skeleton_kind(facts: &CanonicalLoopFacts) -> Option<SkeletonKind>`
  - `facts.facts.skeleton.as_ref().map(|s| s.kind)`
  - `None` の場合は “未知” 扱い（P2では gate しない）
- `fn infer_exit_usage(facts: &CanonicalLoopFacts) -> Option<ExitUsageFacts>`
  - `facts.facts.features.as_ref().map(|f| f.exit_usage.clone())`
  - P2ではまだ候補の増減に使わない（観測・将来の feature 合成の足場）

注意:
- skeleton/feature に矛盾があっても `debug_assert!` まで（Freeze は増やさない）

### Step 2: “候補 push” を分類関数に分割（順序は維持）

`build_plan_from_facts_ctx()` の本体を薄くして、push の順序を関数分割で固定する。

例（順序は今のまま）:
- `push_scan_with_init(&mut candidates, facts)`
- `push_split_scan(&mut candidates, facts)`
- `push_pattern2_break(...)`
- `push_pattern3_ifphi(...)`
- `push_pattern4_continue(...)`
- `push_pattern5_infinite_early_exit(...)`
- `push_pattern8_bool_predicate_scan(...)`（allow_pattern8 gate維持）
- `push_pattern9_accum_const_loop(...)`
- `push_pattern1_simplewhile(...)`（allow_pattern1 gate維持）

受け入れ:
- ルール文字列（`rule: "loop/..."`）は完全一致のまま
- `allow_pattern1` / `allow_pattern8` の gate は今のまま

### Step 3: unit tests（構造固定）

追加/更新:
- 既存テストが壊れないこと（テストの LoopFacts 構築はそのまま）
- 新規で 1 本だけ追加（任意）:
  - skeleton/features を持つ LoopFacts を与えても、候補が変わらない（構造固定）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p2): stage planner via skeleton/feature inference (no behavior change)"`

