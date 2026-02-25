---
Status: Ready
Scope: Reserve ExitKind::Unwind (docs-first; no behavior change)
Related:
- docs/development/current/main/phases/phase-29au/README.md
- docs/development/current/main/design/exitkind-unwind-reservation-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29au P0: Unwind reservation SSOT (docs-first)

## Objective

Define the SSOT-level contract for `ExitKind::Unwind` so:
- future exception/unwind work does not require reworking cleanup rules,
- observability tags can be stable across Normal/Return/Break/Continue/Unwind,
- and current behavior remains unchanged.

## Tasks

1. Ensure SSOT exists:
   - `docs/development/current/main/design/exitkind-unwind-reservation-ssot.md`
2. Update pointers:
   - `docs/development/current/main/10-Now.md`
   - `docs/development/current/main/30-Backlog.md`
   - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`

## Acceptance

- docs-only (no tests required), or optionally:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

