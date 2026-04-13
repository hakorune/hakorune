Status: ACTIVE
Owner: Codex
Phase: 273x

# Phase 273x

## Summary

- continue `IPO / build-time optimization` with a shared callable/edge contract owner seam
- keep the cut narrow: static IPO facts only
- keep `ThinLTO` wiring and `PGO` work out of this phase

## Current Cut

- the shared IPO build-policy owner seam is already landed
- this phase should fix
  - `IpoCallableContract`
  - `IpoCallEdgeContract`
  ownership before any `ThinLTO` choice is wired
- profile-generate / profile-use behavior remains disabled

## Next

- `ThinLTO` first cut
- `PGO` scaffold after that
