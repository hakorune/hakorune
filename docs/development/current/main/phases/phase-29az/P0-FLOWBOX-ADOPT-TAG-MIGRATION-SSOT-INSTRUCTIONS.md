---
Status: Instructions
Scope: Phase 29az P0（docs-only）
---

# P0: FlowBox adopt tag migration SSOT (docs-only)

目的: strict/dev の採用点観測を FlowBox schema に収束させるための “移行SSOT” を 1 枚で固定する。

## やること（docs-only）

1. `docs/development/current/main/design/flowbox-adopt-tag-migration-ssot.md` を追加
2. 参照導線を `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md` の Related に追記
3. Phase 29az README の Related を更新

## 受け入れ

- strict/dev の観測は FlowBox schema を SSOT として説明できる
- release の既定ログ不変、ノイズフィルタの既存挙動も不変
