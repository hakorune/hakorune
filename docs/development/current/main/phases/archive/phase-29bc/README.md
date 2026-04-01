---
Status: Complete
Scope: Plan/Composer API consolidation + dead_code cleanup
Related:
- docs/development/current/main/design/coreloop-composer-single-entry-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29bc: Composer API consolidation + cleanup

## Goal

`src/mir/builder/control_flow/plan/composer/` の API を **最小の SSOT 入口**へ寄せる。

- “使われない足場” と “古い dead_code scaffolds” を削除して迷子を防止する
- `CoreLoopComposer single entry` の導線を明確にする（`try_compose_core_loop_from_facts` が入口）
- 既定挙動・ログ・エラー文字列は不変（純リファクタ）

## Non-goals

- subset 拡張（facts/extractors/planner の拡張）
- FlowBox schema / 観測語彙の変更
- 新 env var の追加

## Plan

- P0: SSOT（composer の公開 API）を固定（docs-first）✅
- P1: `composer/mod.rs` の dead_code scaffolds を削除して API を引き締める ✅
- P2: closeout（docs-only）✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29bc/P0-COMPOSER-API-SSOT-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29bc/P1-COMPOSER-CLEANUP-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29bc/P2-CLOSEOUT-INSTRUCTIONS.md`

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
