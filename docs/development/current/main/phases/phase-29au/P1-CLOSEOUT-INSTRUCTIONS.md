---
Status: Ready
Scope: Phase 29au closeout (docs-only)
Related:
- docs/development/current/main/phases/phase-29au/README.md
- docs/development/current/main/design/exitkind-unwind-reservation-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29au P1: Closeout (docs-only)

## Objective

Close Phase 29au after confirming the Unwind reservation SSOT and pointer
alignment are in place, then move focus to the next docs-first phase.

## Checklist

1. Mark `docs/development/current/main/phases/phase-29au/README.md` as `Status: Complete`.
2. Ensure the SSOT doc is linked from the roadmap:
   - `docs/development/current/main/design/exitkind-unwind-reservation-ssot.md`
3. Move pointers:
   - `docs/development/current/main/10-Now.md`
   - `docs/development/current/main/30-Backlog.md`
   - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## Recommended next phase (docs-first)

- Observability tags: FlowBox schema (strict/dev only)
  - Prefer `box_kind` + `feature_set` + `freeze_code` tags over pattern names.
  - Keep release output unchanged.

## Verification

- docs-only (no tests required), or optionally:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

