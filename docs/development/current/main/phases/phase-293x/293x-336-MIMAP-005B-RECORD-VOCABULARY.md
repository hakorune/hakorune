---
Status: Landed
Date: 2026-05-14
Row: MIMAP-005B
Scope: record vocabulary for mimalloc-shaped Hakorune blueprint.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-record-vocabulary-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-005B Record Vocabulary

## Summary

Defined identity-free record vocabulary for allocator references, size-class
entries, page/segment snapshots, lifecycle stats, and gap summaries.

## Outputs

- Added `docs/development/current/main/design/mimalloc-hakorune-record-vocabulary-ssot.md`.
- Kept records as value aggregates only.
- Kept raw pointer residence and record behavior out of scope.

## Next

`MIMAP-005C` should define enum/transition lifecycle skeletons that consume this
vocabulary.
