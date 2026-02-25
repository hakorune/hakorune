---
Status: Instructions
Scope: Phase 29ay P0（docs-only）
---

# P0: Unwind integration SSOT (docs-only)

目的: ExitKind::Unwind を “予約” で終わらせず、CorePlan/FlowBox/cleanup/effect の契約に統合した設計SSOTを固定する。

## 入力（既存SSOT）

- Unwind reservation: `docs/development/current/main/design/exitkind-unwind-reservation-ssot.md`
- Cleanup contract: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`
- Effect classification: `docs/development/current/main/design/effect-classification-ssot.md`
- FlowBox interface: `docs/development/current/main/design/coreplan-flowbox-interface-ssot.md`

## 出力（新規SSOT）

- `docs/development/current/main/design/unwind-cleanup-effect-integration-ssot.md`

## 設計で固定する内容（最小）

1. **ExitKind 語彙**: Return/Break/Continue/Unwind を “同列の出口” として扱う
2. **cleanup の適用**: cleanup は ExitKind を問わず（normal含む）同一規則で包む
3. **effect 制約**: Unwind を跨いで移動できない effect（Control/Io/RC/Obs）の扱いを明文化
4. **観測**: strict/dev のみ FlowBox schema タグと Freeze taxonomy を使い、release は不変

## 受け入れ

- docs-only（コード変更なし）
- 既存SSOTと矛盾しない（参照導線を追って一周できる）
- “実装しないが破綻しない” を説明できる（未来のUnwind導入で契約が揺れない）
