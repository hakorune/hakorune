Status: ACTIVE
Owner: Codex
Phase: 270x

# Phase 270x

## Summary

- continue `closure split` with an env scalarization owner seam
- keep the cut narrow: classify empty/single/multi env layouts only
- preserve current ctor lowering behavior

## Current Cut

- `closure_split_contract` now marks:
  - empty envs as `scalar_none`
  - single-capture envs as `scalar_single`
  - multi-capture envs as `aggregate_multi`
- current lowering still calls:
  - `nyash.closure.new` for empty env
  - `nyash.closure.new_with_captures` for non-empty env

## Next

- closure thin-entry specialization
- IPO / build-time optimization
