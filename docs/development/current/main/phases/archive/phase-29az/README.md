---
Status: Complete
Scope: FlowBox adopt tag migration (strict/dev only; release logs unchanged)
Related:
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md
- docs/development/current/main/design/flowbox-adopt-tag-migration-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29az: FlowBox adopt tag migration (strict/dev only)

Goal: strict/dev の “採用/フォールバック観測” を FlowBox schema (`[flowbox/*]`) に寄せ、旧 `[coreplan/shadow_adopt:*]`
依存を段階的にゼロへ収束する。

release の既定挙動・恒常ログは不変。

## Plan

- P0: SSOT（移行方針）を書く（docs-only）✅
- P1: smokes を FlowBox schema へ寄せる（raw output で検証、generic smoke は従来どおりフィルタ）✅
- P2: 旧 coreplan/shadow_adopt タグの撤去（strict/dev のみ、挙動不変）✅
- P3: closeout（docs-only）✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29az/P0-FLOWBOX-ADOPT-TAG-MIGRATION-SSOT-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29az/P1-SMOKE-MIGRATION-TO-FLOWBOX-SCHEMA-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29az/P2-REMOVE-COREPLAN-SHADOW-ADOPT-TAGS-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29az/P3-CLOSEOUT-INSTRUCTIONS.md`

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Closeout notes

- strict/dev の採用点観測は FlowBox schema（`[flowbox/*]`）を SSOT とする。
- 旧 `[coreplan/shadow_adopt:*]` は emit/参照を撤去（smoke は FlowBox へ移行済み）。
