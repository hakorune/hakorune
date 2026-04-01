---
Status: Complete
Scope: CoreLoopComposer unification (v0/v1/v2 → single entry)
Related:
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/coreloop-composer-single-entry-ssot.md
- docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29bb: CoreLoopComposer unification (single entry)

## Goal

CorePlan 合成の入口（CoreLoopComposer）を **feature-driven の単一入口**に収束させ、`coreloop_v0/v1/v2_*` の “版分岐” を
外へ漏らさない。

目的は **挙動不変**での構造整理（self-host 移植しやすい形へ）であり、subset 拡張や新機能追加はしない。

## Non-goals

- 新しい subset 追加（extractors/facts の拡張）
- 新しい env var / debug toggle 追加
- strict/dev のタグ語彙変更（FlowBox schema を維持）

## Plan

- P0: SSOT（単一入口の契約）を書く（docs-only）✅
- P1: `composer` の単一入口を導入（内部で v0/v1/v2 を呼ぶだけ）✅
- P2: `shadow_adopt.rs` の分岐を単一入口に寄せる（重複撤去）✅
- P3: closeout（docs-only）✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29bb/P0-CORELOOP-COMPOSER-SINGLE-ENTRY-SSOT-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29bb/P1-IMPLEMENT-SINGLE-ENTRY-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29bb/P2-SHADOW_ADOPT-USE-SINGLE-ENTRY-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29bb/P3-CLOSEOUT-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29bb/P2-SHADOW_ADOPT-USE-SINGLE-ENTRY-INSTRUCTIONS.md`

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
