Status: LANDED
Owner: Codex
Phase: 257x

# Phase 257x

## Summary

- widen the current `SimplifyCFG` jump-threading cut
- allow `Branch -> empty Jump trampoline -> final` threading even when the threaded arm carries edge-args
- keep the cut narrow by allowing edge-arg dropping only when the final target is PHI-free

## Landed Contract

- threading still requires:
  - empty trampoline block
  - trampoline terminator `Jump { edge_args: None }`
  - no PHIs on the trampoline itself
- threaded-arm edge-args may be dropped only when:
  - the final target has no PHIs
  - the trampoline/final path does not need those edge-args after threading
- if the final target still has PHIs, threaded-arm edge-args keep the transform disabled for this cut

## Proof

- `cargo fmt --check`
- `cargo test -q --lib mir::passes::simplify_cfg::tests -- --nocapture`
- `cargo test -q --lib mir::passes::semantic_simplification::tests -- --nocapture`
- `cargo check -q --lib`
- `bash tools/checks/dev_gate.sh quick`

## Next

- continue the `semantic simplification bundle`
- next queued cut is `S2` first SCCP propagation widening beyond direct `Compare`
