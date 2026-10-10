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

## Decision required

- Measure the failing owner/exit, and whether its root exit row is absent,
  Unavailable, Prepared, Emitting, Emitted or Finalized. Compare the selected
  early and tail exits on the same original source.
- Identify the sole physical Loop/return owner and the existing root-home
  prepare/emit/finish path. Choose where the selected early return can borrow
  the verified source and call the same path without another authority.
- Preserve all-exit cleanup order, Fault/Normal separation, and final MIR
  verification. Reject missing/mismatched site or physical coordinate.
- Fix one bounded execution path only after the above source/physical join
  and focused positive/negative acceptance are decided. Use one read-only
  worker for this design audit.

Non-claims: no generic loop return, alternate cleanup emitter, whole
SizeClassBox or mimalloc-lite EXE completion.
