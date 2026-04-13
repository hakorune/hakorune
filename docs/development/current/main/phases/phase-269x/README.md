Status: LANDED
Owner: Codex
Phase: 269x

# Phase 269x

## Summary

- start `closure split` with a shared capture classification owner seam
- keep the cut narrow: closure creation classification only
- defer closure env scalarization and closure thin-entry specialization

## Current Cut

- closure creation now reads a shared contract that classifies:
  - `empty_env`
  - `capture_env_only`
  - `me_only_env`
  - `capture_env_with_me`
- current lowering behavior stays the same:
  - empty env uses `nyash.closure.new`
  - non-empty env uses `nyash.closure.new_with_captures`
- both modern and legacy closure lowering routes read the same owner seam

## Closeout

- closure env scalarization is the next cut
- closure thin-entry specialization stays after env scalarization
