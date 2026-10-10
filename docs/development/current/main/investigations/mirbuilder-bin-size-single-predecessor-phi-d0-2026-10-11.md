# MIRBUILDER-BIN-SIZE-SINGLE-PREDECESSOR-PHI-D0

Status: superseded by LoopCond route D0
Date: 2026-10-11
Parent: `mirbuilder-negative-integer-return-home-s0-2026-10-11.md`
Scope: unchanged `SizeClassBox.bin_size/1` recorded physical boundary
Related: `mirbuilder-loop-cond-after-phi-d0-2026-10-11.md`

The signed-immediate Home source proof moves the original SizeClassBox
probe from aggregate `IncompleteOrdinaryNewCoverage` to
`ordinary-new/local-commit/physical-boundary/phi-in-recorded-block`.
Disposable-only diagnosis identifies `BasicBlockId(32)` in
`SizeClassBox.bin_size/1`: a leading PHI, one predecessor, and
`single_predecessor_phi_body=None`
(`/tmp/hako-negative-integer-home-s0-phi-diag-20261011.log`).
`PhysicalBoundary::capture_with_phi_policy` rejects that shape. Existing
physical negative tests pin mid-block and unsupported single-predecessor
PHIs; preserve those refusals.

The exact disposable MIR dump
(`/tmp/hako-negative-integer-home-s0-phi-block-20261011.log`) shows block 29
is the loop header: it has PHIs from 27/31 and an `Lt` branch whose false
edge goes to block 32. Block 32 starts with three one-input PHIs, each
referencing predecessor 29 exactly, then computes `words` and invokes
`SizeClassBox.word_size/0`. Thus block 32 is the variable-accumulation
loop's after continuation, not the selected Static Call loop body.

## Decision

The canonical variable-accumulation loop physicalizer owns the verified
`root_after` returned from `ready_after.emit_and_seal` in
`loop_node_lowerer.rs`; `variable_accum.rs` consumes that continuation and
sets the current block. Issue a private-field role there from the verified
admission's owner+source loop site, sealed after block, and the exact PHI
prefix issued by canonical SSA writeback. The existing
callable state lends the same package `OrdinaryNewClaimLedgerV1`, so
`variable_accum.rs` records the role directly in that ledger once after
successful physicalization and source-site consumption. The ledger is the
sole storage/consumer; no state mirror or new MIR-derived issuer is needed.
Lend only that verified after block and PHI prefix to
the physical boundary's existing one-predecessor PHI policy, separate from
the selected Static Call loop-body role. Do not infer permission from a
raw MIR block ID, one predecessor alone, or a matching function name.

The bounded S0 must keep exact CFG/PHI validation: a leading PHI run,
exactly one actual predecessor, each PHI identical to the sealed canonical
`dst` and `(predecessor, ValueId)` rows, and unchanged finished projection/
edge checks. Missing role,
owner/site/block drift, mid-block PHI, wrong input value or edge, and
unselected loops must refuse. Positive acceptance is the unchanged
`bin_size` source passing this boundary, with `size_to_bin` or another
named later frontier exposed; existing physical PHI negatives remain green.
The read-only audit found that block+predecessor alone would let a wrong
but defined `ValueId` pass. Exact PHI prefix comparison closes that gap.
The subsequent unchanged-source probe found no VariableAccum role was issued
for `bin_size`; its source shape is outside that specialized route. The
proposed issuer is therefore wrong for this caller. Preserve the PHI-prefix
requirement, but select the actual generalized-loop issuer before coding a
replacement slice.
Route instrumentation selected `LoopCondReady`; the replacement Decision and
physical role are in `mirbuilder-loop-cond-after-phi-d0-2026-10-11.md` and
its S0. This card is historical counterexample evidence, not active design.
Do not turn this permission into an unconditional `is_empty()` debt:
unselected loops may have no recorded lifecycle block requiring it.

Keep source S0 open until the physical owner permits its required group
consumption evidence. The separate `size_to_bin` search-loop Home D0 remains
queued after this frontier; no whole-source acceptance is claimed.
