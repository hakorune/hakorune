Status: LANDED
Owner: Codex
Phase: 256x

# Phase 256x

## Summary

- widen the current `SimplifyCFG` jump-threading cut
- allow `Branch -> empty Jump trampoline -> final` threading when the final block PHIs can be rewritten from the trampoline predecessor to the branching block
- keep loop/self-edge, duplicate-predecessor, and non-trivial PHI cases out of scope

## Landed Contract

- threading still requires:
  - empty trampoline block
  - trampoline terminator `Jump { edge_args: None }`
  - no PHIs on the trampoline itself
- final-block PHIs are allowed only when:
  - the trampoline has exactly one predecessor
  - every final-block PHI has exactly one incoming edge from the trampoline
  - no final-block PHI already has an incoming edge from the branching block
- on success, the final-block PHI predecessor is rewritten from the trampoline block id to the branching block id

## Proof

- `cargo fmt --check`
- `cargo test -q --lib mir::passes::simplify_cfg::tests -- --nocapture`
- `cargo test -q --lib mir::passes::semantic_simplification::tests -- --nocapture`
- `cargo check -q --lib`
- `bash tools/checks/dev_gate.sh quick`

## Next

- continue the `semantic simplification bundle`
- next natural cuts are broader jump-threading or the next `memory-effect layer` slice
