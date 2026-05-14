---
Status: Landed
Date: 2026-05-14
Row: MIMAP-007
Scope: size-class / bin map executable pilot for mimalloc-shaped Hakorune work.
Related:
  - docs/development/current/main/design/mimalloc-size-class-bin-pilot-ssot.md
  - docs/development/current/main/design/mimalloc-first-executable-slice-ssot.md
  - lang/src/hako_alloc/memory/size_class_box.hako
  - apps/mimalloc-size-class-usize-policy-proof/main.hako
---

# MIMAP-007 Size-Class / Bin Pilot

## Summary

Adopted the existing `SizeClassBox` and `mimalloc-size-class-usize-policy-proof`
as the first executable near-transcription slice for the mimalloc blueprint lane.

## Outputs

- Added `docs/development/current/main/design/mimalloc-size-class-bin-pilot-ssot.md`.
- Bound existing executable artifacts to MIMAP-007 instead of duplicating policy code.
- Kept the pilot no-OSVM, no-atomic, no-TLS, no-rawbuf, and no-provider.

## Main Decision

MIMAP-007 closes by recognizing the already-landed size-class policy as the
first executable slice. Future work can wrap scalar results in `SizeClassEntry`,
but that is not required for this row.

## Next

`MIMAP-008` should start the page/free-list model pilot using explicit lifecycle
state and the existing size-class policy.
