---
Status: Complete
Scope: CorePlan migration finalization (Done SSOT + selfhost handoff)
Related:
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29be/README.md
---

# Phase 29bf: CorePlan migration finalization（Done SSOT + selfhost handoff）

## Goal

CorePlan 移行が SSOT の Done criteria を満たしていることを “手順＋ゲート” で固定し、次の開発（selfhost）へ安全に戻す。

## Non-goals

- subset 拡張（stdlib 形状を増やす）
- 新しい env var の追加
- release 既定の意味論/エラー文字列/恒常ログの変更

## Plan

- P0: Done criteria verification（docs-first）✅
- P1: Handoff doc（selfhost の入口とガード）✅
- P2: closeout（docs-only）✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29bf/P0-DONE-CRITERIA-VERIFICATION-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29bf/P1-SELFHOST-HANDOFF-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29bf/P2-CLOSEOUT-INSTRUCTIONS.md`

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P0 Result (brief)

- Gate green: build/quick/phase29ae pass
- Router legacy loop/table: no matches
- Release adopt list confirmed in phase-29ae README

## Summary

- Done criteria verification and gate checks are fixed as SSOT
- Selfhost handoff doc is finalized with rules and pointers
- CorePlan migration is stable for release path
- Legacy routing is not relied on by gate
- Next: selfhost focus (TBD)
