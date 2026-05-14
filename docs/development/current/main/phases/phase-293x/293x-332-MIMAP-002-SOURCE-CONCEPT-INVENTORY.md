---
Status: Landed
Date: 2026-05-14
Row: MIMAP-002
Scope: mimalloc source concept inventory for Hakorune blueprint planning.
Related:
  - docs/development/current/main/investigations/mimalloc-upstream-pin.md
  - docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-002 Source Concept Inventory

## Summary

Classified upstream mimalloc concepts into near-transcription,
lifecycle-rewrite, substrate-gap, representation-gap, and deferred-unsafe groups.

## Outputs

- Added `docs/development/current/main/investigations/mimalloc-source-concept-inventory.md`.
- Classified segment/page/block/heap/free-list/size-class/os/stats concepts.
- Seeded the MIMAP-003 lifecycle vocabulary for page and segment states.
- Seeded the MIMAP-004 gap ledger for raw memory, atomics, TLS, OSVM, bitmaps,
  and global allocator replacement.

## Main Decision

Do not translate mimalloc line-by-line. The first executable work should choose
one bounded near-transcription slice after lifecycle states are named.

## Next

`MIMAP-003 lifecycle rewrite blueprint` should define page/segment states,
transitions, guard points, and non-goals before any executable allocator row.
