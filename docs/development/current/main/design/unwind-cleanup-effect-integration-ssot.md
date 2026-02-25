---
Status: SSOT
Scope: ExitKind::Unwind integration (design-only; no behavior change)
Related:
- docs/development/current/main/design/exitkind-unwind-reservation-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
---

# Unwind / Cleanup / Effect Integration (SSOT)

目的: ExitKind::Unwind を導入しても cleanup/effect/FlowBox の契約が揺れないよう、設計上の不変条件を固定する。

## 0. 非目的

- 例外機構/stack unwinding の実装
- release 既定の意味論/ログの変更

## 1. ExitKind

- ExitKind は `Return | Break | Continue | Unwind` を同列に扱う（出口語彙の拡張で吸収する）。

## 2. cleanup の扱い

- cleanup は “出口の種類” ではなく “出口を横断する wrapper” として定義する。
- cleanup は以下すべてに適用されうる:
  - normal（継続）
  - Return/Break/Continue/Unwind

## 3. effect 制約（設計）

- Unwind を跨いだ移動・ホイストが危険な effect は、既存の effect classification に従って禁止する。
- strict/dev では、禁止が破られた場合は Freeze/Fail-Fast にする（silent fallback 禁止）。

## 4. observability

- strict/dev のみ FlowBox schema タグ/Freeze taxonomy で可視化する。
- release 既定のログ出力は不変（タグは raw output の gate smoke のみで検証）。
