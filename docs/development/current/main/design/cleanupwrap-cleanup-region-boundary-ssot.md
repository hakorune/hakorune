---
Status: SSOT
Scope: CleanupWrap + cleanup region boundary (CorePlan / Recipe-first)
Related:
- docs/development/current/main/phases/phase-29bq/README.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/recipe-tree-and-parts-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
---

# CleanupWrap + cleanup region boundary (SSOT)

## Goal

`cleanup` の意味論を「どこで実行されるか」ではなく「どの境界を通るか」で固定し、nested exit でも責務が混線しない土台を作る。

本SSOTは BoxShape の土台固定が目的であり、受理拡張は行わない。

## Vocabulary

- `CleanupWrap(inner, cleanup)`:
  cleanup を FlowBox の構造ラッパとして表現する語彙。
- `cleanup region boundary`:
  `inner` の出口を受け取り、cleanup 実行後に同じ `ExitKind` で外側へ渡す境界。
- `ExitKind`:
  `Normal | Return | Break | Continue | Unwind(reserved)`。

## Boundary contract

1. `inner` から外へ出る経路は、`cleanup region boundary` を必ず 1 回通る。
2. boundary は `cleanup` 実行後に、`ExitKind` と payload を保持したまま外側へ渡す。
3. nested cleanup は LIFO で適用される（inner cleanup -> outer cleanup）。
4. cleanup 自身はこのフェーズでは effect-only とし、新しい exit 分岐は導入しない。

## Invariants

- AST rewrite 禁止。
- 受理拡張なし（BoxCount を混ぜない）。
- Verifier が唯一の受理ゲート。
- release 既定挙動は不変。

## Layer responsibilities (SSOT)

- Facts:
  cleanup の有無と境界情報を観測するだけ。出口再判定をしない。
- Verifier:
  「bypass なし」「LIFO」「effect-only cleanup」を機械的に検証する。
- Lower/Parts:
  Verifier 済みの boundary 情報だけを配線する。fallback で補正しない。

## Fail-fast policy (strict/dev)

cleanup 統合で契約違反を検出した場合は原因側で freeze する。

- missing boundary routing
- cleanup bypass routing
- cleanup emits unsupported exit

タグ命名は既存 taxonomy に合わせ、`[freeze:contract]` 系を維持する。

## Phase 29bq operation note

- このSSOTは「CleanupWrap + boundary を先に固定する」ための docs-first closeout。
- `.hako` mirbuilder cleanup lane（M7/M8）は既存 pin 契約を維持し、failure-driven 運用を継続する。

## Decision

- Decision: accepted (2026-02-10)
- Exit/cleanup の責務境界は `CleanupWrap + cleanup region boundary` を真実源にする。
