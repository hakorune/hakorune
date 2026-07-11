# Phase 189x: merge-entry overwritten local field-set pruning

Status: Landed

Purpose
- widen effect-sensitive DCE one step past the `phase188x` linear-edge cut by allowing predecessor-local `FieldSet` pruning when a reachable merge block overwrites the same definitely non-escaping local root/field before any reachable read or escape use
- keep this slice narrow by requiring the predecessor block to have exactly one reachable successor and by keeping loop backedges, generic `Store`/`Load`, `Debug`, and terminators outside the cut

Scope
- DCE-only widening inside [dce.rs](/home/tomoaki/git/hakorune-selfhost/src/mir/passes/dce.rs)
- focused unit contracts for branch-join overwritten local field writes
- docs/pointer sync for the landed slice

Non-goals
- no generic `Store` pruning
- no generic `Load` pruning
- no `Debug` stripping
- no terminator cleanup
- no loop-backedge overwrite folding
- no mixed-root merge reasoning

Acceptance
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`

Result
- predecessor-local `FieldSet` writes now disappear even when the overwriting write lives in a reachable merge block, as long as the predecessor has a single reachable successor into that merge and the merge writes the same local root/field before any reachable read or escape use
- loop backedges, mixed-root merges, `Store`, `Load`, `Debug`, and terminators stay outside this cut
