Status: LANDED
Owner: Codex
Phase: 274x

# Phase 274x

## Summary

- landed the first narrow `ThinLTO` cut
- consumed only the landed build-policy + callable/edge contract seams
- kept `PGO` work out of this phase

## Current Cut

- `phase272x` landed the shared build-policy owner seam
- `phase273x` landed `IpoCallableContract` / `IpoCallEdgeContract`
- this phase landed the first narrow `ThinLTO` policy cut on top of those seams
- `ThinLTO` stays default-off and emits a companion `.thinlto.bc` only when thin mode is explicitly requested and direct-thin candidates are present
- profile-generate / profile-use behavior remains disabled

## Next

- `PGO` scaffold
