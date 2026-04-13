Status: ACTIVE
Owner: Codex
Phase: 271x

# Phase 271x

## Summary

- continue `closure split` with a thin-entry specialization owner seam
- keep the cut narrow: thin-entry eligibility only
- preserve current ctor lowering behavior

## Current Cut

- `closure_split_contract` now marks:
  - empty envs as thin-entry candidates
  - single-capture envs as thin-entry candidates
  - aggregate envs as public-entry-only
- current lowering still keeps:
  - `nyash.closure.new` for empty env
  - `nyash.closure.new_with_captures` for non-empty env

## Next

- closure split closeout judgment
- IPO / build-time optimization
