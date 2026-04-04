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

## Inventory Freeze

- `thin-now`
  - `docs/development/current/main/phases/README.md`
    - 117 lines, still carrying a long landed ledger in the current-facing index
- `keep-now`
  - `CURRENT_TASK.md`
    - 62 lines, already pointer-thin enough
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
    - 53 lines, restart contract only
  - `docs/development/current/main/10-Now.md`
    - 39 lines, current pointer only
  - `docs/development/current/main/15-Workstream-Map.md`
    - 57 lines, one-screen map still acceptable

## Target Ranking

1. `phases/README.md` first-cut thinning
   - replace long landed ledger with a short active/recent-landed window plus phase-local references
2. `15-Workstream-Map.md` only if first cut still leaves duplicate current narrative
3. leave `CURRENT_TASK.md`, `05-Restart-Quick-Resume.md`, and `10-Now.md` unchanged unless the first cut proves they widened again

## First Cut Result

- `docs/development/current/main/phases/README.md`
  - `117 -> 65` lines
  - now carries:
    - current active lane
    - recent landed window
    - a short corridor/history pointer

## Acceptance

1. target mirror/index surfaces are source-backed
2. cuts preserve current navigation value
3. proof shows current pointers remain usable after thinning
