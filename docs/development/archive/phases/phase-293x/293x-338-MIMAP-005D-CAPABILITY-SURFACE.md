---
Status: Landed
Date: 2026-05-14
Row: MIMAP-005D
Scope: capability surface for mimalloc-shaped Hakorune allocator blueprint.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-005D Capability Surface

## Summary

Defined the mimalloc blueprint capability surface and fail-fast boundaries.

## Outputs

- Added `docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md`.
- Fixed `uses osvm`, `uses atomic`, and `uses rawbuf` as canonical surfaces.
- Marked `uses tls` and `uses random` as provisional future decisions.
- Kept provider activation, hooks, and global allocator replacement inactive.

## Next

`MIMAP-006` should select the first executable near-transcription slice and avoid
hard substrate gaps.
