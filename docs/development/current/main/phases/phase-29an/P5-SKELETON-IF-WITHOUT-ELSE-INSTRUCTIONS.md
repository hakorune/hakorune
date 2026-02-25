---
Status: Active
Scope: code（仕様不変、SkeletonFacts のSSOT正しさ修正）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29an P5: SkeletonFacts — `if` without `else` is still If2（SSOT correctness）

Date: 2025-12-29  
Status: Ready for execution  
Scope: SkeletonFacts の分類だけを正しくする（未接続、仕様不変）

## Objective

- `SkeletonFacts` が `ASTNode::If { else_body: None }` を **If2Skeleton** として分類できるようにする
  - else が無くても CFG 上は “then / fallthrough” の 2 分岐 + join を持てるため、骨格としては If2 に含めるのが自然
- 既存の JoinIR / Plan / Frag の挙動は一切触らない（未接続）

## Non-goals

- ルーティング順序・観測・エラー文字列の変更
- Facts の `Ok(None)` gate の変更
- SkeletonFacts の利用箇所を増やす（P5 は分類の正しさだけ）

## Implementation

Update:
- `src/mir/builder/control_flow/plan/facts/skeleton_facts.rs`

### Step 1: If2 判定を else 有無に依存しない形へ

Before（概念）:
- `ASTNode::If { else_body: Some(_) } => If2`

After:
- `ASTNode::If { .. } => If2`

### Step 2: unit tests 追加/更新

`#[cfg(test)]` に以下を追加:
- `if` + `else_body: None` が `SkeletonKind::If2` になる
- （既存があるなら）`if` + `else_body: Some(_)` も `SkeletonKind::If2` のまま
- `match` が `SkeletonKind::BranchN` になる（最小）

## Verification（required）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29an(p5): classify if-without-else as if2 skeleton"`

