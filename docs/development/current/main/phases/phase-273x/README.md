Status: ACTIVE
Owner: Codex
Phase: 273x

# Phase 273x

## Summary

- continue `IPO / build-time optimization` with the first `ThinLTO` cut
- keep the cut narrow: policy-to-build wiring only
- keep `PGO` work out of this phase

## Current Cut

- the shared IPO build-policy owner seam is already landed
- this phase should wire the first visible `ThinLTO` choice through that seam
- profile-generate / profile-use behavior remains disabled

## Next

- `PGO` scaffold
- broader IPO closeout
