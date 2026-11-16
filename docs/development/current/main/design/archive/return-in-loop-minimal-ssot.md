---
Status: Historical (was: SSOT)
Scope: loop 内 early return を吸うための最小語彙（stdlib `StringUtils.is_integer` 専用）
Related:
- docs/development/current/main/phases/phase-29ar/README.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
---

# Return-in-loop minimal SSOT

> **Historical(2026-11-17、メンテナ決定):** この文書は、古い設計か作業の記録である。**新しい作業を選ぶ根拠にしない。** MirBuilder(source → MIR)の設計は [MirBuilder の設計図](../mirbuilder-blueprint.md) を読む。食い違う時は、設計図が勝つ。

This document fixes the minimal vocabulary for early return inside a loop.

## Scope

- Target: stdlib `StringUtils.is_integer` minimal shape only.
- No nested loops, no multiple exits, no general return-heavy patterns.

## Vocabulary (minimal)

- `ExitIfReturn { cond, value }`
  - Return-only; no generic ExitIf and no non-return kinds.
  - `value` must match return payload contract (exactly one value).

## Verification (strict/dev)

- `ExitIfReturn` is only allowed in loop bodies (effect-only contexts).
- `cond` must be a boolean expression.
- `value` must be present and compatible with function return.

## Lowering

- Lower to `if cond { return value }` using existing CorePlan/Frag emission.
- Do not bypass block_params or re-parse CorePlan structures.

## Notes

- Expansion to Break/Continue is out of scope for this phase.
