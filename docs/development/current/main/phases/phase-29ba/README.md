---
Status: Complete
Scope: FlowBox fallback observability consolidation (strict/dev only)
Related:
- docs/development/current/main/design/flowbox-adopt-tag-migration-ssot.md
- docs/development/current/main/design/flowbox-fallback-observability-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ba: FlowBox fallback observability consolidation

Goal: strict/dev の “採用/フォールバック” 観測を FlowBox schema に一本化し、補助タグを撤去する。

release 既定挙動・恒常ログは不変。

## Plan

- P0: SSOT（fallback 観測）を書く（docs-only）✅
- P1: code 側の fallback タグを FlowBox freeze に寄せる（strict/dev only）✅
- P2: closeout（docs-only）✅

## Summary

- strict/dev の fallback 観測は `flowbox/freeze` の code 語彙に一本化
- `[plan/fallback:*]` はコードから撤去（release 既定挙動・恒常ログは不変）

## Instructions

- P0: `docs/development/current/main/phases/phase-29ba/P0-FLOWBOX-FALLBACK-OBSERVABILITY-SSOT-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29ba/P1-CONVERGE-FALLBACK-TO-FLOWBOX-FREEZE-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29ba/P2-CLOSEOUT-INSTRUCTIONS.md`

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
