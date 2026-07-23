# RAW-SOURCE0 LOWER ROOT0 — CALLMAIN0-S0 execution task

Status: **Ready for implementation; production consumers remain zero**  
Date: 2026-07-24  
Decision: **CALLMAIN-prime-r1**

## Boundary

CALLMAIN0 consumes the already-closed `RawChildrenCompleteInvocationV1` and
closes only the optional callable `Main.main` compatibility child. It does
not lower the inline root body or create the physical `main/0` and
`condition_fn/1` root batch.

```text
RawChildrenCompleteInvocationV1::{Script, App}
  -> finish_callable_main(self)
       sealed disposition is consumed once
       exact App callable-Main locator is consumed only when Selected

  -> RawCallableMainReadyInvocationV1::{Script, App}
       same token/source/session/physical owner
       helper receipts retained
       callable-Main outcome sealed

  -> BODY0 later; this task does not provide a second entry
```

The following remain zero in this row: inline root-body lowering,
`main/0`/`condition_fn/1` batching, drain, finalization, postprocess, external
commit, public ingress, JSON behavior, retry, fallback, `catch_unwind`, and
CUT0 activation.

## Locked decisions

### Q1 — selection authority

Only `RawSourceContinuationV1::callable_main()` selects the route. Its
disposition is consumed exactly once while the post-CALLMAIN continuation is
formed.

```text
Script + NotSelected -> typed Script ready product, physical effects = 0
Script + Selected    -> mutation-free route-drift rejection
App + NotSelected    -> locator moves to non-selected evidence, no descent
App + Selected       -> exact sealed locator becomes callable work
```

Locator presence is source identity, not selection authority. Physical/ledger
disposition is corroborating provenance only; it may reject a mismatch but
may not select or downgrade the route.

### Q2 — physical owner

`RawRootPhysicalStateV1` receives the sole consuming
`complete_callable_main(self, work)` terminal. The terminal creates a
short-lived lowering/collector loan internally and returns named success or
rejection products.

Compiler code must not receive a shell, collector, ledger, tracker, physical
tuple, `ModuleLoweringPortV1`, or `RawInvocationChildPortV1`. The disconnected
`RawDraftInvocationV1`, `ModuleLoweringInvocationStateV1`,
`MainPending/MainCaptured`, and `with_shell_collector` owners are not reused.

### Q3 — selected sequencing

The selected route is strictly:

```text
consume CHILDREN0 completion
-> consume disposition
-> consume exact callable locator
-> validate route/physical provenance and declaration
-> build dedicated CallableMainCompatibility work/request
-> reserve one ledger entry
-> capture/lower/admit/restore
-> complete the exact ledger receipt
-> issue RawCallableMainReadyInvocationV1
```

BODY0 may accept only the ready product. Any selected failure ends the
unpublished invocation before body descent.

### Q4 — semantic role and evidence

The request must use the existing dedicated
`RawExpansionDraftRequestV1::callable_main_compatibility` constructor. The
role is never inferred from the symbol spelling and never authored through
`legacy_discovered`.

The first eligibility slice accepts only source `Main.main/0`. The callable
compatibility request keeps its existing route-specific ledger policy
(`LegacyReplaceWholePair`); CALLMAIN0 proves the semantic role, exact symbol,
arity, brand, and receipt correspondence, and must not relabel it as the
canonical inserted-only policy used by other routes.

The branded callable receipt is retained in a separate outcome lane from the
CHILDREN0 helper receipts. It is not counted by
`RootBodyCompletionTrackerV1`, which remains BODY0-only and untouched.

### Q5 — failure owner

Every fallible transition returns a discard-only
`RejectedRawCallableMainInvocationV1` retaining the complete CHILDREN0
evidence, current physical/ledger state, failed locator, optional issued
receipt, and exact typed cause. The owner exposes inspection and `discard`
only. It has no retry, resume, sibling continuation, `mark_omitted`,
replacement plan, fallback, or body-entry terminal.

Existing coarse ledger abort reasons remain unchanged. Primary, Cleanup,
`DuringCleanup`, Admission, and receipt/ledger errors remain nested in the
rejected owner; `DuringCleanup` maps to the existing coarse Cleanup reason.
No `catch_unwind` or typed panic-retention claim is added.

## Products

```rust
RawCallableMainWorkV1
RawCallableMainReceiptV1
RawAppCallableMainOutcomeV1::{NotSelected, Selected}
RawCallableMainReadyInvocationV1::{Script, App}
RejectedRawCallableMainInvocationV1
```

After success, the reusable callable-Main disposition and locator are no
longer available to BODY0. The ready product carries only the sealed outcome
evidence, while the exact inline-root `main` locator remains in the body-plan
remainder.

## Implementation scope

Add the following small modules (each source/check file must remain below 800
lines):

```text
src/mir/compiler/raw_root_callable_main.rs
src/mir/compiler/raw_root_callable_main_p0.rs
src/mir/builder/raw_root_physical/callable_main_terminal.rs
tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_callmain0_guard.py
```

Make only narrow handoff edits in:

```text
src/mir/compiler/raw_root_children.rs
src/mir/compiler/raw_root_plan0.rs
src/mir/compiler/raw_source_binding.rs
src/mir/builder/raw_root_physical.rs
src/mir/builder/raw_root_physical/child_terminal.rs
src/mir/compiler/mod.rs
src/mir/builder.rs
```

Reuse the existing exact locator validator and lower-level child-session
restore path. Do not rescan the AST, rebuild the callable catalog, rerun
`sorted_method_entries`, or introduce a test-only fault adapter.

## Acceptance fixtures

### Success

```text
Script + NotSelected:
  Script ready product; reservation/descent/collector/ledger deltas = 0

App + NotSelected:
  locator retained as non-selected evidence; no CallableMainCompatibility row

App + Selected, zero helpers:
  exact Main.main/0 receipt with CallableMainCompatibility role

App + Selected, helpers already complete:
  helper receipt order unchanged; callable receipt is a separate lane

all success:
  token/session/physical/collector/ledger brands agree
  RootBodyCompletionTracker.completed_children remains 0
  physical wrapper main/0 and condition_fn/1 do not yet exist
```

### Failure and atomicity

```text
Script + forged Selected -> mutation-free selection rejection
continuation/physical disposition mismatch -> rejection before source index
missing or drifted locator -> rejection before reservation
natural Primary/Cleanup/DuringCleanup child error -> exact nested cause retained
collector admission failure -> Admission cause and coarse ledger mapping
ledger completion failure after admission -> issued receipt plus ledger state retained
```

For every rejection:

```text
helper prefix evidence retained
failed locator retained
later sibling descent = 0
BODY0 entry = 0
retry/fallback/re-entry = 0
live Builder mutation = 0
external commit = 0
```

The natural failure fixture should reuse the existing child-session error
shapes. CALLMAIN0 does not add panic injection, `catch_unwind`, or a new
failure authority.

## Guard contract

```text
continuation disposition branch authority = 1
callable locator presence as selection authority = 0
callable_main_compatibility request consumer = 1
legacy_discovered(CallableMainCompatibility) = 0

RawDraftInvocationV1 / ModuleLoweringInvocationStateV1 references = 0
MainPending / MainCaptured / with_shell_collector references = 0
compiler-side shell/collector/ledger tuple = 0
compiler-side RawRootPhysicalStateV1::into_parts = 0
RootBodyCompletionTracker begin/close/complete calls = 0

rejected retry/resume/continue/mark_omitted/fallback = 0
catch_unwind = 0
BODY0/root-batch/drain/finalizer/postprocess/commit consumers = 0
public ingress/JSON behavior changes = 0
production consumers = 0
all modified/new source and check files < 800 lines
```

The existing CHILDREN0 guard is a closed-row guard, not a required current
row gate. It must be run with its historical-row mode or merged into the
reusable Raw-root lane guard before CALLMAIN0 closeout; CALLMAIN0 must not
leave a stale guard that fails solely because the active row advanced.

CALLMAIN0's new guard owns only the selected/not-selected count, continuation
authority, exact locator handoff, dedicated request role, same-brand
correspondence, no-body-descent failure law, tracker immutability, and
production-consumer census. It must not duplicate CHILDREN0 assertions.

## Proof budget and sunset

```text
ceremony_tier = T1
sunset_id = CALLMAIN0-PROOF-SUNSET-01
proof_inventory_before = CHILDREN0 helper owner + lower-level ledger/session evidence
new_proofs = one-shot disposition consumption, callable role receipt,
             selected physical terminal, typed discard-only rejection
retired_or_merged_proofs = none in S0
sunset_budget = +4
sunset_row = CALLMAIN0-G0 / CUT0-G0
retire_when = production Raw owner is the sole consumer and disconnected
              CALLMAIN0/CHILDREN0 proof callers are zero
budget_repayment_evidence = caller census, merged Raw-root guard, parity fixture
```

Do not add a second generic transaction authority or a permanent per-cell
guard. The role-specific product is justified only until the production Raw
owner and its final caller census make the disconnected proof removable.

## Explicit non-claims

```text
inline root-body lowering
Main/condition root batch
declaration/access installation
drain/finalization/postprocess/external commit
public ingress and JSON bridge behavior
typed panic retention
retry/fallback/CUT0 activation
```

## Handoff

The implementation may begin only from this task card. On green focused tests
and the CALLMAIN0 guard, update the pointer to the next BODY0 consultation or
execution row. Do not activate production consumers in CALLMAIN0-S0.
