# MIRBUILDER-SEARCH-LOOP-ROOT-EXIT-CONSUMPTION-D0

Status: selected design stop
Date: 2026-10-11
Scope: physical consumption of the original search Loop's ready early return
Related: `mirbuilder-search-loop-exit-relation-s0-2026-10-11.md`

## Observed frontier

The unchanged `size_class_box.hako` plus trivial Main now has the selected
early `I64Scalar` and tail `Call` Normal exit projections. Physical input
stops at
`[freeze:contract][ordinary-new/local-commit/root-exit-unconsumed]`
(`/tmp/hako-search-loop-exit-s0-physical-20261011.log`). This validation
requires a `RootHomeExitProgress` row for every ready explicit exit.

The normal return statement route invokes `prepare_root_home_exit` through
`raw_expression_dispatch/statement_surface.rs` and
`raw_ordinary_new_claim.rs`. The selected search Loop's early return may
bypass that route, but the observed error does not name an owner or exit.
Treat that as a hypothesis until an exact census identifies the missing row.

## Measured exit census

At `dd451b7e0d`, a temporary test-only probe compiled the unchanged
`lang/src/hako_alloc/memory/size_class_box.hako` with `Main.main() { return 0 }`.
The `quick` test binary ran one selected physical-input probe; build 5m 00s,
test 0.02s. Its log is `/tmp/hako-root-exit-census-20261011.log`.
`SizeClassBox.size_to_bin/1` is owner slot 5. Its early exit
`Body(2)/LoopBody(0)/IfThen(0)` and tail exit `Body(3)` both have
`RootHomeExitProgress = absent`; validation reports `root-exit-unconsumed`
at the early exit first. Other measured exits have `Emitted` rows. The
temporary probe and diagnostic print were removed; the tree returned clean.

## Decision: one root cleanup graph with an explicit Static Loop handoff

- Source authority is the existing Home Normal/Fault exit projections plus
  the preissued Static I64 V2 product and ExactI64 result claim. The ledger's
  `prepare_root_home_exit` is the sole cleanup-order issuer.
- Physical cleanup and root-exit recording remain with
  `builder/ordinary_new_admission/selected.rs`. The selected Static Loop
  path is a caller of that owner, not a second cleanup emitter. Its early
  scalar and tail Call must each consume their exact site, value, and
  physical coordinate; tail Invoke/NormalResult/Fault ingress must be
  handed to that owner exactly once.
- A progress-only `Emitted` row is forbidden. DraftSeal currently appends
  Return to unterminated early/tail blocks, while the root emitter creates
  its own clean Return block. Directly running both would duplicate Return.
  The handoff must make one Return producer explicit and corroborate the
  emitted Return against Completion before publication.
- Preserve existing all-exit order, Fault/Normal split, local-call bindings,
  final MIR verification, and exact site/owner/coordinate rejection. No
  generic Loop extension or empty-order bypass is selected.

The read-only worker verified the source paths: early and tail call
`canonical.completion.claim_explicit_return` in `static_loop_body_exit.rs`
and `static_loop_tail.rs`; neither prepares or records root Home progress.
`static_loop_entry_session.rs` validates both after DraftSeal. The worker
also identified the Return ownership conflict above.

## Remaining design dependency before an implementation slice

The physical handoff shape is not yet fixed. Map how the existing
`FunctionFaultFrameV1` in the Static Loop can be lent to the root emitter
without a second frame, how tail's existing Invoke is moved under the sole
root Call ingress, and how DraftSeal will corroborate rather than duplicate
the chosen Return. Choose whether a behavior-preserving emitter extraction
must land before the selected-path extension; keep the two meanings in
separate slices. Then name the exact production caller replacement and
focused positive/negative acceptance. Stay in `design_stop` until this
mapping is fixed; do not issue a new semantic receipt or fixture first.

Non-claims: no generic loop return, alternate cleanup emitter, whole
SizeClassBox or mimalloc-lite EXE completion.
