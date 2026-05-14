---
Status: Landed
Date: 2026-05-14
Row: MIMAP-003
Scope: lifecycle rewrite blueprint for mimalloc-shaped Hakorune allocator work.
Related:
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-003 Lifecycle Rewrite Blueprint

## Summary

Defined the mimalloc lifecycle rewrite boundary as explicit Hakorune enum states,
transition tables, guard points, invariants, and capability stop lines.

## Outputs

- Added `docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md`.
- Defined page, block, and segment lifecycle vocabularies.
- Listed transition families for page and segment state movement.
- Identified guard points for allocation, release, retire, reclaim, and reactivate.
- Separated no-capability local models from `osvm`, `atomic`, `tls`, and `rawbuf` rows.

## Main Decision

Upstream mimalloc lifecycle state is not copied from flags and pointer ownership.
Hakorune owns the lifecycle explicitly with enum + transition + guard contracts.

## Next

`MIMAP-004 substrate and representation gap ledger` should turn this blueprint's
capability boundaries into explicit missing-feature and fail-fast rows.
