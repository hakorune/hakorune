Status: LANDED
Owner: Codex
Phase: 276x

# Phase 276x

## Summary

- landed the first `PGO` generate/use cut
- kept the cut narrow and default-safe
- kept LLVM-side instrumentation/use out of scope

## Current Cut

- `phase272x` landed the shared build-policy owner seam
- `phase273x` landed `IpoCallableContract` / `IpoCallEdgeContract`
- `phase274x` landed the first narrow `ThinLTO` artifact cut
- `phase275x` landed the shared `PGO` scaffold owner seam
- this phase landed the first generate/use cut without widening beyond the current IPO lane
- `generate` now resolves a `.profraw` artifact path
- `use` now accepts an existing indexed profile path
- compile now emits a `.pgo.json` sidecar describing the current generate/use artifact decision

## Next

- optimization lane closeout judgment
