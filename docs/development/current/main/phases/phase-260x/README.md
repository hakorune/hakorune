Status: LANDED
Owner: Codex
Phase: 260x

# Phase 260x

## Summary

- land the first `memory-effect layer` owner seam and its stats surface
- land same-block private-carrier store-to-load forwarding under the new owner
- land same-block private-carrier redundant load elimination under the new owner
- keep the current private-carrier `Load` / `Store` cleanup outside `semantic simplification bundle`
- hand off the next queue to `escape / barrier -> LLVM attrs`

## Landed Contract

- `memory_effect::apply(module)` is now the top-level owner for the landed memory-sensitive pruning slice
- `OptimizationStats` now has a `memory_effect_optimizations` bucket
- `semantic simplification bundle` keeps pure DCE / CSE / CFG simplification only
- the following memory work is now outside the simplification bundle:
  - dead private-carrier `Load` pruning
  - same-block private-carrier store-to-load forwarding
  - same-block private-carrier redundant load elimination
  - overwritten private-carrier `Store` pruning
- the memory-effect layer queue is complete

## Proof

- `cargo fmt --check`
- `cargo test -q --lib mir::passes::memory_effect::tests -- --nocapture`
- `cargo test -q --lib mir::passes::dce::tests::memory -- --nocapture`
- `cargo test -q --lib mir::optimizer::tests -- --nocapture`
- `cargo check -q --lib`
- `bash tools/checks/dev_gate.sh quick`

## Next

- hand off to `escape / barrier -> LLVM attrs`
