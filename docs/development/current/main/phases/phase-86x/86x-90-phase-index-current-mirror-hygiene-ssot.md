# Phase 86x SSOT

## Intent

`86x` thins heavy phase index / current mirror surfaces after `85x` selected this lane as the highest-leverage cleanup.

## Facts to Keep Stable

- `84x` landed with Stage1 build/default entry contracts repointed to canonical `entry/*`.
- `85x` selected `86x` over:
  - `87x embedded snapshot / wrapper repoint rerun`
  - `88x archive/deletion rerun`
- root/current mirrors should stay pointer-like, not ledger-like.
- implementation and historical detail belongs in phase-local docs, not root/current mirrors.

## Initial Focus

1. identify which current mirror files still carry redundant landed history
2. identify whether `phases/README.md` can be thinned without losing current navigation value
3. keep the read order stable while shrinking duplicated narrative

## Acceptance

1. target mirror/index surfaces are source-backed
2. cuts preserve current navigation value
3. proof shows current pointers remain usable after thinning
