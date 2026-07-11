---
Status: Landed
Date: 2026-05-14
Row: MIMAP-005A
Scope: brand/type vocabulary for mimalloc-shaped Hakorune blueprint.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-005A Brand/Type Vocabulary

## Summary

Defined the canonical Hakorune scalar vocabulary for mimalloc-shaped allocator
blueprint work.

## Outputs

- Added `docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md`.
- Separated unit aliases from identity brands.
- Chose `HeapId`, `SegmentId`, `PageId`, `BlockId`, `ArenaId`, `SizeClassId`,
  `Generation`, and `ThreadId` as initial brands.
- Kept raw pointer identity out of the vocabulary until `rawbuf` / `Span` rows.

## Main Decision

Use brands for identity and generation boundaries. Use type aliases for ordinary
units such as `Bytes`, `Count`, `Index`, `Offset`, and alignment/count names.

## Next

`MIMAP-005B` should define identity-free record vocabulary using these names.
