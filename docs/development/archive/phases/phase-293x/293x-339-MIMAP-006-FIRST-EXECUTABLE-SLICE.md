---
Status: Landed
Date: 2026-05-14
Row: MIMAP-006
Scope: first executable near-transcription slice selection for mimalloc-shaped Hakorune work.
Related:
  - docs/development/current/main/design/mimalloc-first-executable-slice-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-006 First Executable Slice

## Summary

Selected the size-class/bin lookup pilot as the first executable
mimalloc-shaped slice.

## Outputs

- Added `docs/development/current/main/design/mimalloc-first-executable-slice-ssot.md`.
- Rejected OSVM, atomic, TLS, rawbuf, and provider-dependent slices as first work.
- Defined the acceptance shape for `MIMAP-007`.

## Main Decision

Start execution with a small table-driven `SizeClassEntry` lookup. The table may
be a pilot subset and must not claim full mimalloc parity.

## Next

`MIMAP-007` should implement the size-class/bin pilot and proof surface.
