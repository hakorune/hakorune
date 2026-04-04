---
Status: Active
Date: 2026-04-04
Scope: choose the next source lane after the rust-vm retirement corridor ended in residual explicit keep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-63x/63x-90-rust-vm-final-retirement-decision-ssot.md
  - docs/development/current/main/phases/phase-63x/README.md
---

# 64x-90 Next Source Lane Selection SSOT

## Intent

- pick the next source lane after the `60x -> 63x` corridor
- keep the rust-vm decision stable:
  - mainline retirement: achieved
  - full source retirement: deferred
  - residual explicit keep: frozen
- keep `vm-hako` out of the retirement corridor as reference/conformance

## Starting Read

- current direct/core mainline is stable
- rust-vm remains residual explicit keep only
- the next lane should not reopen broad rust-vm ownership

## Candidate Directions

- `keep-surface hygiene`
  - continue narrowing explicit keep wording and proof/compat helper surfaces
- `direct/core mainline hardening`
  - improve current mainline checks, contracts, and source ownership around direct/core paths
- `non-vm future lane selection`
  - choose a lane outside rust-vm retirement, such as product/runtime or archive follow-up

## Decision Rule

- prefer the lane with the highest leverage on current source clarity
- do not reopen rust-vm broad-owner work without new caller-zero or replacement evidence
- keep `vm-hako` as live reference/conformance, not archive/delete scope

## Big Tasks

1. `64xA1` successor lane inventory lock
2. `64xA2` candidate lane ranking
3. `64xB1` successor lane decision
4. `64xD1` proof / closeout
