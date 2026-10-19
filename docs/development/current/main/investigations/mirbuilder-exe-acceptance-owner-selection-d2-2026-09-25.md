# APP-allocator-stress

Status: selected; Loop/If/Bool read pass; Main terminal source-result connection remains open.
Date: 2026-10-19
Scope: unchanged allocator-stress, physical EXE, exact output/exit and retained handle safety.
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml),
[retained-handle obligation, row 7](mirbuilder-exe-acceptance-suite-red-disposition-d21-2026-09-27.md),
[parked untyped-slot D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md).

## Selection and measured stop

- User authorized skipping parked untyped-field. Allocator reuses passing mimalloc's heap;
  binary-trees/JSON also need unresolved untyped slots, JSON additional Text/Loop connections.
- Latest fixed suite: committed computed-argument revision `3965b758ba`, 6PASS/5FAIL,
  actual EXE5 + empty marker1, no SKIP/timeout; mimalloc green. Outer1900s/build/run300s.
  Later Bool/If/argument WIP is outside that measurement; no product acceptance claim.
- Matching BranchNormal quick CLI passes prior source refusal at main.hako:29
  `rejects = rejects + 1` (Body11/IfThen0). Bool equality at line28 also passes.
- Matching canonical smoke passes Loop, If Phi and release/Get Bool comparison, then stops
  at Main `return stress.run()`: `ordinary-new/local-commit/artifact-source-unavailable`.
  No EXE artifact is produced. Original line15 computed Object arguments also pass.
  Next: connect the original scalar-result loan to the existing terminal-call protocol;
  do not infer an I64 result from MIR or broaden the declaration-only route.
- Existing smoke enters canonical physical lifecycle EXE directly, retaining exact
  rejects=4/output/exit, lifecycle/C API/artifact checks and no compat replay.

## Connected owners

- Same original named-Array source selects holder/Get/Loop-body append before Text issuance.
  Same Home observation, checked Loop action, Recipe slot and child group reach the sole
  raw/Loop append writer. No duplicated root effects or source rewrite.
- Append transfer/Fault cancellation and holder teardown remain checked. Original source
  Phi tuples feed existing liveness/repair; backedges require exact stable resource state
  and agreement between cached/actual CFG, without claiming termination.
- Computed arguments retain complete finite signed source SUM and original argument-purpose
  frames. I64 uses ScalarCompleted/NormalInteger; Text keeps OwnedTextDeferred/OwnedString.
  Declared usize/legacy transport remain unchanged. Wire tag1 requires the same finalized
  descriptor and complete signed callee domain; bare frames cannot issue transport.
- Add/Mul use the existing numeric envelope/writer/final matcher. Delegated argument frames
  retain exact Call/ordinal, original method/plan/loan identity and evaluation-time Homes;
  their parent Call owns execution/final checks, not separate Loop child prefixes.
- Bool equality uses exact Bool/Bool Eq/Ne envelopes, existing Compare writer/final matcher;
  no truthiness or Bool/Integer mixing. Existing positive/negative matrices cover this class.
- IfForm shares snapshots across both arms and restores merge values/provenance. BranchNormal
  retains complete original ThenNormal/ElseNormal inputs and original immutable flow coordinate.
  Existing IfForm condition/exits and indexed Phi supply physical evidence; the same
  ExpressionReferenceRead checks every input and finished correspondence. No new SSA writer.
- Loop port coverage requires original If coverage and every incoming producer. Worker
  review confirms coverage union alone grants no physical/source-edge correspondence.

## Current WIP and verification

- BranchNormal root build PASS; existing focused61 PASS, test list9263 unchanged.
  Existing initializer matrix changes expected refusal to proved If-join, without new cases.
- Known `callable_source_loop_exit_read_binds_after_phi_dst` remains red at Text source
  Loop unavailable, identical to preserved pre-change binary. It remains an open boundary.
- Builder dependency/CallTarget guards and diff check PASS. Previous If-state CLI's mimalloc
  smoke PASS, exact output/exit and no replay; current argument WIP has not repeated it.
- WIP normalizes the existing argument source view to borrowed site/arguments/Homes.
  Candidate arguments come from the SAME Taken Object packet and original Array recipe,
  with no synthetic local/Home. Existing receiver consumer is lent to nested argument Calls;
  candidate receiver itself stays consumed exactly once.
- Scalar and Object candidate purposes share the existing argument cleanup graph writer.
  Call Fault: argument Ends then prior Homes, with no nonexistent candidate cancellation.
  Normal argument-End Fault: remaining Ends, SAME packet candidate cancellation, then Homes.
  Normal success retains candidate for Append; Append Fault retains cancellation then Homes.
  The independent final argument validator derives these expectations from the original packet.
- Append Call validation now invokes that same full source/frame/producer/Normal-Fault check
  and requires argument completion before transfer. Worker review confirms phase separation.
- Object-argument root build PASS; existing focused88 PASS, 5 baseline reds reproduced
  with the preserved pre-change binary (3 Array source, 2 synthetic final-call visitors).
  They remain open, not accepted as success. Test list9263 unchanged.
  Matching CLI build PASS; unchanged smoke advanced to expression-call/context-drift.
- WIP normalizes Append prior Homes to acquisition order, matching Call/expression frames.
  Remove two reversed comparisons and the unwind-side reverse adapter; the existing unwind
  owner still emits newest-first. Initializer-prefix order is a separate unchanged contract.
  Worker confirmed all consumers; root74 PASS/6 baseline reds, test list9263 unchanged.
  Matching CLI passes context-drift and reaches ordinary If's not-indexed-phi.
- WIP registers actual materialized/sorted final Phi at the same successful writer.
  Unborrowed final patch replaces its indexed observation; any original identity/append loan
  freezes it. Provisional still completes once. Both use the existing index, not MIR inference.
  Worker approved. One existing test adds the independent unborrowed-final/borrowed-final
  mutation boundary; provisional-only evidence cannot protect it. No new test function/file.
  Root build and existing focused65 PASS, test list9263 unchanged. Matching CLI also passes
  Phi indexing and advances beyond the previous stop before any artifact.
- WIP connects the same Array Get Fault inventory through existing array_read_records_v1
  inside the original validated Call walk. Producer lending is unchanged; no new accessor.
  Existing Read owner retains source/Fault/final checks. Root/CLI builds PASS; existing
  focused55 PASS, test list9263 unchanged. Matching smoke reaches the Main stop above.
- No new file/test function/hook/checker/registry or runtime/C ABI change.
  Existing manual Array writer test only adapts the receiver callback signature.

## Completion obligations

- `bash tools/smokes/v2/profiles/integration/apps/allocator_stress_exe.sh` must pass all
  artifact/run/output obligations, including requested_bytes=454, rejects=4 and exit0.
- Retire replaced exclusive edges only after caller/validation closure. Shared owners stay.
  Untyped D0 stays ParkedSealed until explicit acceptance; full goal remains incomplete.
- Prior selection and completed implementation details remain in this path's Git history.
