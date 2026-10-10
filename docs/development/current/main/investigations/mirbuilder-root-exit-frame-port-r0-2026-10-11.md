# MIRBUILDER-ROOT-EXIT-FRAME-PORT-R0

Status: closed
Date: 2026-10-11
Scope: behavior-preserving root-exit frame access extraction
Related: `mirbuilder-search-loop-root-exit-consumption-d0-2026-10-11.md`

## Contract

`ordinary_new_admission/selected.rs::emit_root_home_exit_payload` currently
borrows/validates the one function fault frame through
`CallableSemanticLoweringState`. Factor only that access into a private
frame port whose existing implementation delegates to the same methods.
The same port may be implemented by `FunctionFaultFrameV1` for the later
selected Static Loop handoff. The existing root callers, cleanup graph,
Return, recorded bindings, fail-fast diagnostics, and physical bytes keep
their current behavior.

Source authority and canonical issuer: existing root Home ledger and
`FunctionFaultFrameV1`. Non-authority: a new frame definition or semantic
receipt. Target caller: the existing root payload's callable-state frame
access. Replaced responsibility: direct frame borrow/validation calls in
that payload, not the physical graph or source classification.

## Acceptance

- Existing focused root Plain and terminal Static Call physical tests remain
  green, including a negative binding/entry refusal.
- The current `size_class_box.hako` first stop stays
  `root-exit-unconsumed` in this behavior-preserving slice; no later source
  or EXE pass is claimed.
- No second frame definition or changed Return/Invoke/Fault graph; the
  current static selected emitter is untouched.
- `cargo check --profile quick -p nyash-rust --lib`, focused tests,
  `rustfmt --check`, `git diff --check`, and pointer guard pass.

Retirement: this private port remains only while the two physical callers
share one root graph owner; if the selected Static cutover chooses another
frame handoff, remove the unused abstraction in that bounded series.

## Closeout

`RootExitFramePortV1` now delegates callable-state and direct
`FunctionFaultFrameV1` access to the same physical frame owner. The root
payload's three frame operations use the port; all graph, Return, Invoke,
and ledger recording code remains unchanged. `selected.rs` is 678 lines.

At the changed checkout, `cargo check --profile quick -p nyash-rust --lib`
passed. Focused existing tests passed 1/1 each:
`source_terminal_call_preserves_both_cleanup_paths_through_finishing`,
`static_direct_return_records_one_root_invoke`,
`root_and_borrowed_child_field_returns_survive_canonical_finishing`, and
`root_instance_call_rejects_emitted_receiver_mutation`. A temporary probe
over unchanged `size_class_box.hako` confirmed the exact previous
`root-exit-unconsumed` first stop (1/1); it was removed. No selected Static
cutover or whole-source pass is claimed. There is no unclassified red.
