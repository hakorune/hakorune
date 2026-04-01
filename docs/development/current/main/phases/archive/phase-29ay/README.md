---
Status: Complete
Scope: ExitKind::Unwind integration design (docs-first; no behavior change)
Related:
- docs/development/current/main/phases/phase-29au/README.md
- docs/development/current/main/design/exitkind-unwind-reservation-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
---

# Phase 29ay: Unwind integration design (docs-first)

Goal: ExitKind::Unwind（予約済み）を、cleanup/effect/FlowBox の契約に **矛盾なく統合**する “設計SSOT” を 1 枚で固定する。
実装は行わず、release 既定挙動は不変。

## Plan

- P0: SSOT（設計）を書く（docs-only）✅
- P1: code-side “no-op integration” を最小で入れる（strict/dev verify のみ）✅
- P2: closeout（docs-only）✅

## P1 Summary

- strict/dev only の Unwind 契約チェックを最小で追加（release 既定の挙動/ログは不変）
  - `src/mir/builder/control_flow/edgecfg/api/verify.rs`
  - `src/mir/builder/control_flow/edgecfg/api/compose/cleanup.rs`
- FlowBox schema の feature 語彙に `unwind` を追加（未使用でも矛盾なく拡張可能に固定）
  - `src/mir/builder/control_flow/plan/observability/flowbox_tags.rs`
  - `src/mir/builder/control_flow/plan/facts/feature_facts.rs`
  - `src/mir/builder/control_flow/plan/normalize/canonicalize.rs`

## Instructions

- P0: `docs/development/current/main/phases/phase-29ay/P0-UNWIND-INTEGRATION-SSOT-INSTRUCTIONS.md`
 - P1: `docs/development/current/main/phases/phase-29ay/P1-UNWIND-NOOP-CODE-INTEGRATION-INSTRUCTIONS.md`
 - P2: `docs/development/current/main/phases/phase-29ay/P2-CLOSEOUT-INSTRUCTIONS.md`
## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Closeout notes

- Strict/dev only: `ExitKind::Unwind` は “予約語彙” として扱い、未実装の wiring は fail-fast で早期検知する（release 既定は不変）。
- Strict 判定は `joinir_dev::strict_enabled()` を SSOT とする（`NYASH_JOINIR_STRICT` と `HAKO_JOINIR_STRICT` の両方を受理）。
