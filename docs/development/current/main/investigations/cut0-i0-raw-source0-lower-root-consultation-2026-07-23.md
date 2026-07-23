# CUT0-I0 RAW-SOURCE0-LOWER0 ROOT Consultation

Status: **Design stop — Root/App lowering must be separated from the landed
child-draft seam**  
Date: 2026-07-23  
Scope: Raw Script/App root traversal, declaration facts, callable Main, and
atomic root completion only. No public ingress, executor, JSON behavior
change, physical drain, finalizer, postprocess, external commit, or CUT0
activation is allowed here.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-execution-task-2026-07-23.md`
- `src/mir/builder/raw_draft_invocation.rs`
- `src/mir/builder/module_lifecycle.rs`
- `src/mir/builder/raw_physical_finalization.rs`

## Why this is the next design stop

`RAW-SOURCE0-LOWER0-S0` is now landed as a disconnected child-only proof.
It proves one source locator -> reservation -> function-local lowering ->
branded collector receipt -> ledger completion chain. It deliberately does
not claim a Raw module root.

The current root products are not yet a safe continuation:

```text
ModuleLoweringInvocationStateV1
  = MainPending -> MainCaptured -> Complete
  = Raw/Main-only state, not Script/App-neutral

RawRootCompletionInputV1
  = assumes Main + condition_fn receipts
  = rejects Selected callable Main today

MirBuilder::finalize_module
  = synthesizes condition_fn and publishes current_module

RawPhysicalCompleteInvocation::prepare_finalization
  = hardcodes [condition_fn, main]
  = cannot represent already-collected static/helper/callable rows
```

The missing boundary is a route-owned Raw root owner that keeps the source
projection, declaration facts, callable-Main disposition, root body witness,
collector, and ledger in one unpublished state until the complete root batch
is ready.

## Worker inventory

The read-only worker audit found four load-bearing blockers:

1. Script needs an explicit synthetic `main/0` draft and `condition_fn/1`
   draft. It must not obtain either from `finalize_module`.
2. App must preserve declaration/index facts, static children, and the sealed
   `Omitted`/`Selected` callable-Main disposition. A callable-Main locator
   being present is not evidence that it is selected.
3. Main + condition root reservations need a paired reserve/abort/complete
   terminal. Two independent aborts can leave partial ledger history.
4. Root lowering currently reaches current-module-dependent static-data,
   closure, materializer, and legacy call-resolution seams. Those need a
   short shell-backed access port or an explicit non-claim before root
   activation.

Additional invariants:

```text
source Main method arity != physical wrapper main/0 arity
CompletedRootBody brand == invocation token brand
root body completes exactly once
Selected callable-Main failure aborts before inline root lowering
no later sibling descent, retry, fallback, drain, or commit
```

## Questions to lock

### Q1 — root owner/state

How should Root lowering avoid reusing the Main-only state?

```text
1. Add a route-owned RawRootKindV1 { Script, App } state/product around the
   existing child owner. It owns root progress and exposes typed root-batch
   terminals; MainPending/MainCaptured remains legacy/raw-completion-only.

2. Reuse MainPending/MainCaptured for Script and App by making Main optional.

3. Re-enter MirBuilder::lower_root and adapt its published module afterward.
```

Recommendation: **1**. Making Main optional would turn one state into several
incompatible protocols; option 3 creates a second publication authority.

### Q2 — Script root and atomic batch

When should Script root drafts be reserved and committed?

```text
1. Lower the source-driven inline body first, then reserve and preflight one
   required root batch containing physical main/0 and condition_fn/1, and
   commit the two collector/ledger events atomically.

2. Reserve Main and condition independently before body lowering.

3. Let finalize_module synthesize condition_fn after main publication.
```

Recommendation: **1**. The physical wrapper is always `main/0`; source
`Main.main` parameter arity must never be copied into the wrapper row.
`condition_fn/1` must be an unpublished draft produced by the root owner.

### Q3 — App source inventory

How should App declarations and metadata enter the root owner?

```text
1. Extend the owned source projection with declaration/index locators and
   source-derived declaration/static-plan/closure facts, then install those
   facts through one shell-backed port before root body lowering.

2. Re-run VerifiedRawRootExpansionV1 or rescan current_module during root
   lowering to recover missing declarations.

3. Keep the partial PLAN0 projection and silently omit unrepresented boxes,
   free functions, instance methods, static data, or closure metadata.
```

Recommendation: **1**, but split the implementation into a dedicated
declaration/access row if the facts cannot fit safely in ROOT0. Missing facts
must reject before Builder effects rather than be reconstructed from
`current_module`.

### Q4 — callable Main disposition

How should the optional compatibility child be sequenced?

```text
1. Read only the sealed continuation disposition. NotSelected performs no
   reservation. Selected reserves/lowers/collects the exact callable receipt;
   any failure aborts the whole root owner before inline root lowering.

2. Treat a present source locator as Selected.

3. Try the child, discard its error, and continue with inline root lowering.
```

Recommendation: **1**. Presence of a locator is not selection authority, and
Selected failure must preserve the original typed error.

### Q5 — root failure and handoff

What is the failure product and atomic handoff?

```text
1. Return a rejected RawRootInvocation owner retaining token, continuation,
   session/shell, collector prefix, ledger state, root tracker, and any
   successful child receipts. Add paired root reserve/abort/complete
   terminals. Successful root completion produces one route-owned complete
   product; no retry or fresh sibling continuation exists.

2. Return a bare error after dropping the physical prefix.

3. Keep lowering later siblings and report the first root error at finalizer.
```

Recommendation: **1**. The pair terminal must preflight Main and condition
together, require condition replacement `Inserted`, and mutate neither
collector nor ledger on rejection.

## Required non-claims while stopped

```text
Raw root production consumer = 0
public executor / public wrapper wiring = 0
AST-JSON and Program(JSON v0) behavior = unchanged
MirBuilder::lower_root/finalize_module retirement = 0
current_module as expected-inventory authority = 0
physical drain/finalizer/postprocess/external commit = 0
retry/fallback/catch_unwind = 0
```

## Required decision output

The next consultation response must select Q1-Q5 and name the smallest
executable root slice. It must explicitly define:

```text
one Script/App root owner/state
one source-derived declaration/access boundary
one paired root ledger/collector terminal
one callable-Main Selected/NotSelected law
one rejected-owner handoff with no retry
```

Until that decision is locked, do not implement Root/App lowering or wire any
production consumer. S0 remains the landed child-only evidence; Root0 is the
next design row.

## ROOT0-D0 closeout (2026-07-23)

Worker-audited **Candidate RAW-ROOT-prime-r1** is selected. Q1-Q5 all select
option 1, with the first executable row intentionally limited to source-plan
construction.

### Q1 — route-owned root protocol

Raw root lowering receives a dedicated Script/App protocol. The existing
`MainPending -> MainCaptured -> Complete` state remains legacy/disconnected
evidence and is not widened into a Script/App authority. A future root owner
will retain token, source/continuation, candidate session, physical
shell/collector, ledger, child receipts, root tracker, and route-specific
environment as one consuming chain.

### Q2 — required root pair

The inline body closes before any required root reservation. Physical `main/0`
and `condition_fn/1` are then prepared as one required batch. Collector and
ledger checks are mutation-free until a single prepared pair commit. Condition
is fixed to `CanonicalRejectDuplicate + Inserted`. Source `Main.main/N` arity
is never copied to physical wrapper `main/0`.

### Q3 — source-derived environment

The owned source projection is extended into one complete Root environment plan
covering declaration/index facts, callable catalog, static-data plans, closure
sites, access requirements, and root runtime inputs. Missing or unsupported
facts reject before Builder effects; `current_module` is not an inventory
authority. Process-global method-slot mutation is either moved behind a later
invocation-local slot row or explicitly rejected for unsupported source shapes.

### Q4 — callable Main

Only the sealed `RawSourceContinuationV1` disposition selects callable Main.
`NotSelected` performs no reservation or descent. `Selected` lowers and
collects the exact compatibility child before inline root lowering; any
failure aborts the whole root owner and preserves its typed cause. Locator
presence is not selection authority.

### Q5 — failure and handoff

Every fallible root stage returns a discard-only rejected owner retaining the
exact unpublished chain, including successful child receipt evidence. Paired
root reservation terminals are required; independent Main/condition aborts are
forbidden. Successful completion produces one route-owned
`CompletedRawSourceInvocationV1` with source, continuation, session, shell,
collector, sealed ledger, root witness, environment evidence, and child
receipts. Retry, fallback, sibling continuation, drain, finalizer, postprocess,
external commit, and public ingress remain outside this decision.

### First executable row

`RAW-SOURCE0-LOWER0-ROOT0-PLAN0` constructs and verifies the complete
source-derived Script/App root plan only:

```text
RawRootKindV1::{Script, App}
physical main/0 and condition_fn/1 identity
ordered source work schedule
declaration/index plan
callable catalog
static-data and closure locators
access requirements
sealed runtime-input snapshot
```

PLAN0 opens no Builder session, shell, collector, or ledger; performs no
reservation or lowering; and has zero production consumers. The next rows are
OWNER0, DECLACCESS0, SLOT0, CHILDREN0, CALLMAIN0, BODY0, PAIR0, COMPLETE0,
then P0/G0.
