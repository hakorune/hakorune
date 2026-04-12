# 189x-90: merge-entry overwritten local field-set pruning SSOT

Goal
- extend the landed local field DCE ladder with one more narrow effect-sensitive slice after `phase188x`

Contract
- an earlier `FieldSet { base, field, .. }` is removable when:
  - `base` resolves to a definitely non-escaping local root
  - the writing block is reachable
  - that block has exactly one reachable successor
  - the reachable successor path leads into a merge block that overwrites the same `(root, field)` before any reachable `FieldGet` or escape use of that root
  - the successor is not a loop backedge target for the predecessor
- this slice is root-sensitive, not block-shape-sensitive: multiple predecessors on the successor are allowed

Out of scope
- mixed-root merge proofs
- generic memory SSA for `Store` / `Load`
- `Debug` and terminator cleanup
- loop-carried overwrite pruning

Checks
- `cargo fmt --check`
- `cargo test -q --lib mir::passes::dce::tests -- --nocapture`
- `bash tools/checks/dev_gate.sh quick`
- `git diff --check`
