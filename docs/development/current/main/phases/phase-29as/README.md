---
Status: Complete
Scope: CorePlan purity Stage-1 (fallback visibility in strict/dev)
Related:
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29as: CorePlan purity Stage-1

Goal: make fallback visible in strict/dev for gate-target shapes while keeping
release logs unchanged.

## Gate / Commands (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Plan

- P0: docs-first purity SSOT + pointer alignment. ✅
- P1: strict/dev fallback visibility (tag or Freeze) without release logging. ✅
- P2: purity gate smoke ensures no `[plan/fallback:` tags in raw output. ✅
- P3: closeout (docs-only): mark complete and keep gate SSOT green. ✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29as/P0-COREPLAN-PURITY-STAGE1-SSOT-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29as/P1-FALLBACK-VISIBILITY-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29as/P2-PURITY-GATE-SMOKE-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29as/P3-CLOSEOUT-INSTRUCTIONS.md`

## Next (planned)

After Stage-1 is green and stable:

- BranchN (match/switch) skeleton reservation + minimal plan path (docs-first).
- SSOT note: `match` final form is `BranchN`, not permanent nested `If2`:
  `docs/development/current/main/design/match-branchn-skeleton-ssot.md`.
- ExitKind::Unwind reservation (docs-first; no behavior change).
- Observability tags: prefer FlowBox schema (box_kind/feature_set) over pattern names (strict/dev only).
