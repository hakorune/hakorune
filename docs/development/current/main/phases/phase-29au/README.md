---
Status: Complete
Scope: ExitKind::Unwind reservation (docs-first)
Related:
- docs/development/current/main/design/exitkind-unwind-reservation-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29au: ExitKind::Unwind reservation (docs-first)

Goal: reserve `Unwind` as a first-class ExitKind in the CorePlan/FlowBox model,
so cleanup/observability contracts stay stable when exceptions are introduced.

This phase is docs-first and must not change release behavior.

## Plan

- P0: add SSOT doc + pointers (docs-only). ✅
- P1: closeout (docs-only): mark complete and pick next docs-first phase. ✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29au/P0-UNWIND-RESERVATION-SSOT-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29au/P1-CLOSEOUT-INSTRUCTIONS.md`
