# CALLABLE-LOOP-PHYSICAL-CANARY-P0

Status: `Target/failure-phase slice landed; full callable canary still open; caller-zero only`
Date: `2026-08-07`
Parent: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`
North star: `docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md`

## Decision

The bounded ConstI64 S0 and ReadBinding I0 leaves are landed. The current
operation boundary now also has an exact per-row target receipt and a
phase-separated `emit_all` path. The remaining implementation is the first
full callable physicalization canary:

```text
PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
  -> explicit Prelude/entry receipt
  -> complete callable Loop operation program
  -> distinct callable Tail
  -> CanonicalSsaFunctionSessionV2::finish_for_draft_seal
  -> DraftSeal prepare/commit
```

This row is a caller-zero integration proof. It does not select a production
caller, change the selector, publish a module, or retire a legacy route.

Before full physical emission, the row must close four mechanical API gaps
without adding a semantic owner: a consuming Prepared-product handoff, an
exact Prelude materialization receipt, one common five-family operation
dispatcher, and an exact Tail-to-ValueId handoff. The bounded preparation
slice is landed: the consuming Prepared handoff, complete WriteBinding
projection, typed Const/Binary/Compare leaf bridges, private row-level
five-family dispatch, full Recipe-order Builder-free prepare, an opaque typed
value ledger, exact logical-to-physical target receipts, and distinct
pre-effect/post-claim failure types compile and have focused evidence. The
exact Prelude adapter is also landed as caller-zero evidence: resolver-issued
parameter/argument bindings are read through canonical identity, the external
Prelude result is emitted through the resolver-backed static-call helper, and
the Loop input initializer is materialized from its exact source site and
published separately. The Prelude result binding and the Loop input binding
are intentionally distinct; neither is inferred from the other. Tail,
Completion, and DraftSeal adapters remain part of this row;
they must not infer names, re-resolve source, clone Completion, or create a
second CFG/SSA/PHI owner. If an adapter cannot be expressed against the
existing canonical owners, stop and record a design correction before adding
physical code.

## Sole claim

One complete resolver-backed callable fixture can consume one prepared callable
physicalization product exactly once and reach the existing function terminal
and DraftSeal owners on a fresh unpublished session. The common physicalizer
owns only portable Loop operations; the outer callable adapter owns Prelude,
Tail, return ABI, Completion, and DraftSeal handoff.

## Required operation coverage

The real callable fixture must be consumed as a complete program. The canary
must cover its complete operation matrix, not extract a single row from the
full demand:

```text
ReadBinding
ConstI64
CompareI64
BinaryI64
WriteBinding
```

The operation schedule comes from `LoopRecipeV1` structure. Evidence-vector
order, operation names, and profile labels are not execution-order authority.
Every operation is preflighted before Builder effects and emitted exactly once
through the common private leaf emitters and canonical CFG/BindingSSA/PhiTxn
services. Unsupported shapes return typed `NoSafeSlice` before physical
effects; they never fall back to the legacy scheduler.
All issued logical-to-physical target receipts are also validated against the
fresh function's block table and termination state in one read-only batch
before the first operation instruction is emitted. A later target failure is
therefore not allowed to become a partial schedule.

## Required ownership boundaries

```text
PreparedCallableLoopPhysicalizationV1
  owns the one-time compatibility relation

VerifiedLoopOperationPhysicalDemandV1
  owns complete operation/effect coverage and common continuation

common Loop physicalizer
  owns portable operation physicalization only

outer callable lowerer
  owns Prelude, callable Tail, exact return ABI, and Completion claim

CanonicalSsaFunctionSessionV2
  owns CFG/SSA/PHI/completion finish through one terminal

CanonicalFunctionLoweringSessionV1
  owns unpublished-function discard and caller restoration

DraftSeal / ModuleDraftCollector
  remain the sole function/module publication owners
```

The physicalizer must not inspect AST, resolve names, infer Tail from a
BindingRef, create a second SSA/CFG/PHI owner, or use profile-specific route
labels. Loop continuation and callable Tail remain distinct contracts.

## Execution stages

1. Close the bounded API gaps above with private/test-only adapters and
   focused contract tests; no MIR effect is allowed in this stage.
2. Prepare and seal the complete callable product with zero Builder effect.
3. Open one fresh unpublished function session and move Completion exactly
   once into the V2 session.
4. Materialize the explicit Prelude and issue `ReadyLoopEntryV1`, including
   the exact zero-input case when applicable. The receipt contains the Loop
   input binding/value; the Prelude result local remains a separate canonical
   declaration.
5. Allocate and receipt the logical-to-physical Loop blocks through the
   canonical CFG session.
6. Bind the complete operation schedule to those blocks and emit all supported
   operations through the common physicalizer.
7. Open the Loop continuation, materialize the callable Tail, and claim the
   exact completion operand through existing owners.
8. Close the function only through `finish_for_draft_seal`, then pass the
   ready draft through the existing DraftSeal prepare/commit path.
9. Exercise both success and post-emission failure. Every failure discards
   the complete unpublished session and restores the caller exactly once.
10. Reopen a fresh session and repeat the same semantic fixture; compare
   operation/placement/shape receipts, not incidental ValueId or block IDs.

The four adapters are mechanically bounded as follows:

```text
Prepared product:
  one consuming into_parts/into_canary_parts handoff; Completion is moved
  exactly once into CanonicalSsaFunctionSessionV2

Prelude:
  exact prepared callable capability + existing direct-call/profile owners
  -> physical result + separate Loop-input initializer + ReadyLoopEntryV1;
  no name lookup or binding conflation

Operation:
  one full Recipe-order prepare plus a bounded row dispatcher over
  Read/Const/Compare/Binary/Write; unsupported forms reject before canonical
  claims; leaf emitters remain private. `emit_all` now issues one exact target
  receipt per row, validates all target blocks before the first leaf effect,
  and separates target/physical failures, but a complete
  callable schedule has not yet been emitted through Prelude/Tail/DraftSeal.

Tail:
  exact prepared binding + canonical identity read
  -> ValueId -> one completion claim; Tail never becomes Loop After
```

## Acceptance evidence

Required focused evidence:

```text
complete Callable demand/preflight has no extraction API
all five operation families are covered by one complete schedule
Prelude/entry owner and block receipts are exact
Read/Const/Compare/Binary/Write are emitted once each
Loop continuation and callable Tail remain disjoint
Completion is moved and consumed exactly once
finish_for_draft_seal is the only ReadyFunctionDraftSeal issuer
DraftSeal receives a complete unpublished function
pre-effect rejection leaves Builder/session state untouched
post-emission failure discards the whole session
caller context is restored once
fresh-session reuse produces equivalent semantic receipts
production caller count remains zero
legacy/fallback/retry edge count remains unchanged at zero activation
```

The implementation commit must update the exact reference entries and owning
README in the same commit:

```text
docs/reference/mir/loop-recipe-contract.md
docs/reference/mir/generic-loop-stage-matrix.md
src/mir/builder/resolved_lowering/README.md
docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
```

It must also update `CURRENT_STATE.toml`, `10-Now.md`, and the active
workstream receipt. No implementation is complete while those pointers or
reference claims are stale.

## Gates

At minimum, run the focused tests for the complete callable canary, `cargo
check -q`, rustfmt check on touched Rust files, `git diff --check`,
`bash tools/checks/current_state_pointer_guard.sh`,
`bash tools/checks/mirbuilder_inplace_replacement_guard.sh`, and the touched
file `<800`-line check. A failed fast gate stops the row; do not add a new
fixture or fallback to make the gate green.

## Explicit non-claims

```text
Generic G0 physical parity = 0
production selector/switch = 0
module publication beyond the canary draft = 0
retry/fallback/reselection = 0
legacy scheduler/route deletion = 0
M8/M9 all-route coverage = 0
backend performance or C-speed parity = 0
```

The next authorized row after this canary is
`LOOP-CALLER-ZERO-PARITY-G0`. Production selection is a separate design stop;
named caller replacement and selected old-edge retirement happen only after
that gate and the existing in-place replacement law are satisfied.
