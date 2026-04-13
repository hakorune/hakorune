Status: ACTIVE
Owner: Codex
Phase: 274x

# Phase 274x

## Summary

- continue `IPO / build-time optimization` with the first `ThinLTO` cut
- consume only the landed build-policy + callable/edge contract seams
- keep `PGO` work out of this phase

## Current Cut

- `phase272x` landed the shared build-policy owner seam
- `phase273x` landed `IpoCallableContract` / `IpoCallEdgeContract`
- this phase should wire the first narrow `ThinLTO` policy cut on top of those seams
- profile-generate / profile-use behavior remains disabled

## Next

- `PGO` scaffold
