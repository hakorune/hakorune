---
Status: Complete
Scope: CorePlan purity Stage-2 fallback inventory (docs-first)
Related:
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
---

# Phase 29ax: CorePlan purity Stage-2

Goal: strict/dev の fallback を棚卸しして境界を SSOT 化し、次の修正対象を明確にする。

## Plan

- P1: 残 fallback 棚卸し＆境界固定（gate実行 → 分類 → fixture/smoke で固定）✅
- P2: gate の不安定要因除去（環境汚染の直列化/無効化の徹底）✅
- P3: closeout（docs-only）✅

## Summary

- P1: strict/dev purity gate の残 fallback を棚卸しして、境界を SSOT 化
- P2: `phase29as_purity_gate_vm.sh` で `*_JOINIR_DEBUG/DEV` を都度解除し、環境汚染を除去
- P3: closeout（docs-only）

## Inventory (P1)

- phase29as_purity_gate_vm: is_integer strict/dev の tag missing が出た
  - 原因: smoke 実行間で `*_JOINIR_DEBUG/DEV` が残留し、strict 実行の観測条件が揺れた
  - 対応: `phase29as_purity_gate_vm.sh` で毎回 `unset` して直列化（P2）
  - 状態: gate / regression pack ともに PASS に復旧

## Gate / Commands (SSOT)

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Instructions

- P1: `docs/development/current/main/phases/phase-29ax/P1-INVENTORY-REMAINING-FALLBACKS-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29ax/P2-STABILIZE-PURITY-GATE-ENV-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29ax/P3-CLOSEOUT-INSTRUCTIONS.md`
