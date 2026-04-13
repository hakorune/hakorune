Status: LANDED
Owner: Codex
Phase: 258x

# Phase 258x

## Summary

- widen the current `SimplifyCFG` constant-propagation cut
- allow constant compare and branch conditions to follow single-input PHIs
- keep the cut narrow by stopping at passthrough PHIs instead of a broader lattice walk

## Landed Contract

- constant compare / branch folding may now follow:
  - copied constants
  - single-input PHIs
- this cut does not attempt:
  - multi-input PHI lattice joins
  - general dataflow propagation
  - loop-carried constant inference

## Proof

- `cargo fmt --check`
- `cargo test -q --lib mir::passes::simplify_cfg::tests -- --nocapture`
- `cargo test -q --lib mir::passes::semantic_simplification::tests -- --nocapture`
- `cargo check -q --lib`
- `bash tools/checks/dev_gate.sh quick`

## Next

- continue the `semantic simplification bundle`
- next queued cut is `S3` jump-threading / SimplifyCFG closeout judgment
