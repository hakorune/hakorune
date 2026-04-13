Status: LANDED
Owner: Codex
Phase: 260x

# Phase 260x

## Summary

- land the first `memory-effect layer` owner seam and its stats surface
- keep the current private-carrier `Load` / `Store` cleanup outside `semantic simplification bundle`
- make the next queue start at `M1` same-block private-carrier store-to-load forwarding

## Landed Contract

- `memory_effect::apply(module)` is now the top-level owner for the landed memory-sensitive pruning slice
- `OptimizationStats` now has a `memory_effect_optimizations` bucket
- `semantic simplification bundle` keeps pure DCE / CSE / CFG simplification only
- the following memory work is now outside the simplification bundle:
  - dead private-carrier `Load` pruning
  - overwritten private-carrier `Store` pruning
- the next memory cuts stay narrow:
  - same-block private-carrier store-to-load forwarding
  - same-block private-carrier redundant load elimination
  - overwritten-store widening beyond the same-block cut

## Proof

- `cargo fmt --check`
- `cargo test -q --lib mir::passes::memory_effect::tests -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests::memory -- --nocapture`
- `cargo test -q --lib mir::optimizer::tests -- --nocapture`
- `cargo check -q --lib`
- `bash tools/checks/dev_gate.sh quick`

## Next

- continue the `memory-effect layer`
- next queued cut is `M1` same-block private-carrier store-to-load forwarding
