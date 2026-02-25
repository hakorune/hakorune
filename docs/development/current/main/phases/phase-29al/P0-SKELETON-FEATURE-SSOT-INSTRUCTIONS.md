---
Status: Active
Scope: docs-only（仕様不変）
Related:
- docs/development/current/main/phases/phase-29al/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planfrag-ssot-registry.md
- docs/development/current/main/design/planfrag-freeze-taxonomy.md
---

# Phase 29al P0: Skeleton/Feature SSOT（docs-only）

Date: 2025-12-29  
Status: Complete（docs-only）  
Scope: 設計SSOTの追加・導線固定（コード変更なし）

## Objective

- “pattern が重なる” 問題を、実装テクではなく **設計の約束（SSOT）**で根治する
- Facts→Planner→(DomainPlan→CorePlan) を **骨格→特徴→合成**として説明できる状態にする
- “通らない/危険な形” を Freeze taxonomy に落とし、`Ok(None)` との境界を揺らさない

## Deliverables

- `docs/development/current/main/design/coreplan-skeleton-feature-model.md`（SSOT）
- `docs/development/current/main/design/planfrag-ssot-registry.md` に参照追加
- `docs/development/current/main/design/planfrag-freeze-taxonomy.md` に `plan/freeze:unstructured` を追加

## Non-goals

- コード変更
- 新しい env var / 新しい恒常ログ
- 新規パターン追加や互換性変更

