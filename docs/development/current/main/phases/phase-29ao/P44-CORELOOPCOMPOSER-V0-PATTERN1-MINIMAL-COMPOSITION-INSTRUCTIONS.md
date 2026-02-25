---
Status: Ready
Scope: CoreLoopComposer v0 を “最小の実合成” で起動し、Facts→CorePlan(skeleton) の SSOT を composer 側へ寄せる（仕様不変）
Related:
- docs/development/current/main/phases/phase-29ao/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29ao/P42-STAGE3-CORELOOPCOMPOSER-V0-DESIGN-INSTRUCTIONS.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29ao P44: CoreLoopComposer v0 — minimal composition (Pattern1 skeleton only)

## 目的

P43 で作った `CoreLoopComposer v0` は未接続・常に `Ok(None)` の足場になっている。
P44 では **最小の実合成**として Pattern1(SimpleWhile subset) の “Loop skeleton” のみを v0 で組み立てられるようにし、
Facts→CorePlan(skeleton) の責務を **normalizer ではなく composer 側**に寄せる。

重要: 仕様/ログ/エラー文字列は不変（内部の配線のみを整理）。

## 非目的

- Pattern6/7/2/3/5 の v0 合成（P45+）
- exitmap/cleanup/value-join の本格合成（v0 は skeleton のみ）
- 新しい env var 追加

## 実装

### Step 1: `try_compose_core_loop_v0` を Pattern1 skeleton だけ `Some(CorePlan::Loop)` にする

対象:
- `src/mir/builder/control_flow/plan/composer/coreloop_v0.rs`

採用条件（最小・誤マッチ防止）:
- `facts.skeleton_kind == Loop`
- `facts.value_join_needed == false`
- `facts.facts.pattern1_simplewhile.is_some()`（既存 subset の SSOT を利用）
- `facts.exit_kinds_present.is_empty()`（exitmap 合成は v1+、v0 は空のまま）
- `facts.cleanup_kinds_present.is_empty()`

合成:
- 既存の helper を再利用して “core loop skeleton” を組み立てる:
  - `src/mir/builder/control_flow/plan/normalizer/pattern1_coreloop_builder.rs`（`build_pattern1_coreloop`）

注意:
- v0 は builder mutation/emit をしない。`CorePlan` 構築のみ。

### Step 2: `PlanNormalizer::normalize_loop_skeleton_from_facts` を v0 呼び出しに縮退

対象:
- `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`

方針:
- “facts→core loop skeleton” は composer の責務に寄せる。
- 互換のため `normalize_loop_skeleton_from_facts` は残し、内部で `try_compose_core_loop_v0(...)` を呼ぶ。

### Step 3: Pattern1 の shadow/release adopt 経路を v0 に統一（内部配線のみ）

対象（例）:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

方針:
- Pattern1 の Facts→CorePlan(skeleton) 合成は `coreloop_v0` を使う（重複実装を増やさない）。
- 既存の gate/smoke/タグ仕様は維持（新ログ禁止）。

### Step 4: ユニットテストで境界固定

追加/更新:
- `coreloop_v0.rs`:
  - Pattern1 facts あり + exitmap/cleanup empty → `Some(CorePlan::Loop(..))`
  - exitmap present → `Ok(None)`（v0 は扱わない）

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p44): coreloop composer v0 composes pattern1 skeleton"`
