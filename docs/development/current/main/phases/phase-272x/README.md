Status: ACTIVE
Owner: Codex
Phase: 272x

# Phase 272x

## Summary

- start `IPO / build-time optimization` with a shared build-policy owner seam
- keep the cut narrow: build options ownership only
- do not enable `ThinLTO` or `PGO` in this phase

## Current Cut

- centralize LLVM/Python build-policy ownership before widening
- define where:
  - build-time LTO policy
  - profile-generate / profile-use policy
  - object compilation vs future link-time policy
  belong in one seam

## Next

- `ThinLTO` first cut
- `PGO` scaffold after that
