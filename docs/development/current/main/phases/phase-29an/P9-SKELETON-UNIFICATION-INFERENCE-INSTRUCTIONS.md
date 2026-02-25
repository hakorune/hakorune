---
Status: Active
Scope: code（仕様不変、Skeleton一意化のSSOT足場）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P9: Skeleton 一意化（0/1/2+ → None/Some/Freeze）を Facts 側で SSOT 化（未接続）

Date: 2025-12-29  
Status: Ready for execution  
Scope: skeleton inference の境界をコードで固定する（未接続、仕様不変）

## Objective

- Skeleton の “一意化” を **Facts 側**で SSOT として固定する
  - 0 個 → `Ok(None)`（StraightLine: plan対象外）
  - 1 個 → `Ok(Some(SkeletonFacts))`
  - 2 個以上 → `Err(Freeze::unstructured(...))`（複合構造/定義域外）
- 将来の Region planning（loop/if/match を入口分岐で増やさない）に備え、Skeleton/Feature 合成の前提を固める

## Non-goals

- 既存の routing / planner-first / legacy fallback の変更
- 既存の `try_build_loop_facts*` の gate 変更
- 新 env var / 恒常ログ追加
- “骨格の完全推論” をやり切る（P9は境界SSOTが目的。subsetでOK）

## Implementation

### Step 1: Region skeleton inference API を追加（Facts層SSOT）

Update:
- `src/mir/builder/control_flow/plan/facts/skeleton_facts.rs`

Add:
- `pub(in crate::mir::builder) fn infer_region_skeleton_facts(stmts: &[ASTNode]) -> Result<Option<SkeletonFacts>, Freeze>`

Subset rules（保守的）:
- stmts を走査し、`try_extract_skeleton_facts_from_stmt(stmt)` が `Some` を返した数を数える
- `skeleton_count == 0` → `Ok(None)`
- `skeleton_count == 1` → その skeleton を返す（他が straight-line でもOK）
- `skeleton_count >= 2` → `Err(Freeze::unstructured("multiple top-level skeleton statements"))`

Notes:
- “2個以上” を `ambiguous` ではなく `unstructured` にするのが自然（複合骨格は単一skeletonではない）

### Step 2: unit tests（0/1/2+ 境界を固定）

Add tests in `skeleton_facts.rs`:
- empty slice → `Ok(None)`
- `[loop]` → `Ok(Some(Loop))`
- `[assign, if]` → `Ok(Some(If2))`
- `[loop, if]` → `Err(Freeze::unstructured(_))`

### Step 3: wiring はしない（仕様不変）

P9 では `infer_region_skeleton_facts()` を呼び出し側へ配線しない。

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p9): add skeleton unification facts api"`

