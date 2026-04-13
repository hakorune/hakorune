Status: LANDED
Owner: Codex
Phase: 273x

# Phase 273x

## Summary

- landed one shared callable/edge contract owner seam for IPO
- kept the cut narrow: static IPO facts only
- kept `ThinLTO` wiring and `PGO` work out of this phase

## Current Cut

- the shared IPO build-policy owner seam is already landed
- this phase landed
  - `IpoCallableContract`
  - `IpoCallEdgeContract`
  ownership before any `ThinLTO` choice is wired
- profile-generate / profile-use behavior remains disabled

## Next

- `ThinLTO` first cut
- `PGO` scaffold after that
