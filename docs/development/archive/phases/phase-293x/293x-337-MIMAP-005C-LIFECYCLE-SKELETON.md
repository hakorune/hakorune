---
Status: Landed
Date: 2026-05-14
Row: MIMAP-005C
Scope: enum/transition lifecycle skeleton for mimalloc-shaped Hakorune blueprint.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-lifecycle-skeleton-ssot.md
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-005C Lifecycle Skeleton

## Summary

Defined the non-executable Hakorune enum/transition skeleton for page, segment,
and block lifecycle states.

## Outputs

- Added `docs/development/current/main/design/mimalloc-hakorune-lifecycle-skeleton-ssot.md`.
- Fixed `enum` state vocabulary and `transition` declarations for page and segment models.
- Added contract seeds for local release, retire, and OSVM-gated reactivation.

## Next

`MIMAP-005D` should define the capability surface for OSVM, atomic, TLS, rawbuf,
random, and provider boundaries.
