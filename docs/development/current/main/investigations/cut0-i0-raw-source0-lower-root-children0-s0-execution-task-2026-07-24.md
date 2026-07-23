# RAW-SOURCE0 LOWER ROOT0 — CHILDREN0-S0 execution task

Status: **Implementation slice landed; focused success proof green; failure matrix remains in this row**
Date: 2026-07-24
Decision: **CHILD-prime-r1**

## Boundary

`RAW-SOURCE0-LOWER0-ROOT0-CHILDREN0-S0` consumes the already-closed
`RawRootInvocationV1::{Script, App}` physical owner and closes only pre-root
static-helper completion.

```text
RawRootInvocationV1::{Script, App}
  -> RawChildrenPendingInvocationV1::{Script, App}
       exact PLAN0-derived helper schedule
       same token/session/RawRootPhysicalStateV1

  -> complete_all(self)
       validate -> reserve -> capture/lower/admit -> ledger complete

  -> RawChildrenCompleteInvocationV1::{Script, App}
       exact child cardinality/order
       successful prefix receipts
       untouched RootBodyCompletionTrackerV1
```

This row does not lower callable Main, the inline root body, or the required
Main/condition batch. Drain, finalization, postprocess, external commit,
public ingress, JSON behavior, retry, fallback, and CUT0 activation remain
zero.

## Current evidence

The disconnected CHILDREN0 owner, lexical schedule handoff, Builder-side
short-lived child terminal, and Script/App completion products are now wired.
The focused Script-zero-child, App-two-helper lexical-order, locator-drift,
second-child-prefix, and coarse-abort-mapping fixtures pass; production
consumers remain zero. Full physical receipt-brand and ledger-completion
failure fixtures are still required before this row can close.

## Locked decisions

### Q1 — order authority

Use `RawRootChildOrderV1::LexicalMethodName`. The order is produced once by
the PLAN0/source projection's `sorted_method_entries` result. It is a
source-derived deterministic lexical member order, not a claim about textual
declaration order. CHILDREN0 must not re-sort a method `HashMap`.

### Q2 — physical loan owner

`RawRootPhysicalStateV1` owns the only consuming child terminal. It creates a
short-lived route-neutral `ModuleLoweringPortV1`/`RawInvocationChildPortV1`
loan internally and returns a named success or rejection product. Compiler
code receives no shell/collector/ledger tuple and cannot call a raw physical
`into_parts` terminal.

### Q3 — exact transition order

For each moved locator:

```text
validate locator/declaration/work request
-> derive ledger request and admission from one opaque work product
-> reserve one child
-> capture/lower
-> branded collector admission
-> parent restore
-> ledger completion
-> move exact receipt into the successful prefix
```

Only one child reservation is open at a time. Any validation or request
failure occurs before reservation; no fallible unrelated work is inserted
after reservation and before capture.

### Q4 — failure ownership

The rejected owner retains the exact typed source/request/reservation/session/
admission/ledger cause. Existing ledger abort reasons remain coarse:
`Primary`, `Cleanup`, `Admission`, and `Panic`. `DuringCleanup` maps to
`Cleanup` in ledger history while the rejected owner retains both primary and
cleanup details. No new ledger variant or `catch_unwind` policy is added.

### Q5 — completion products

Script produces a typed zero-child completion. App produces exact-all-helper
completion, including the zero-helper case. Successful prefix receipts,
failed locator, and unvisited schedule remain inside a discard-only rejection
on failure. There is no `next_child`, retry, resume, sibling continuation,
fallback, or `into_owner` terminal.

### Q6 — tracker separation

`RawPreRootChildrenCompletionV1` owns helper cardinality/order evidence.
`RootBodyCompletionTrackerV1` remains owned by the route but is untouched:
CHILDREN0 must not call `begin_child`, `close_child`, header-loan, pending-
terminal, or root `complete` operations.

## Products

```rust
RawChildrenPendingInvocationV1::{Script, App}
RawChildrenCompleteInvocationV1::{Script, App}
RawPreRootChildrenCompletionV1
RejectedRawChildrenInvocationV1
RawRootChildReceiptV1
```

The public-in-crate repeated-operation surface is one consuming
`complete_all(self)` terminal. A private `step_one` loop is allowed inside
that terminal.

## Required fixtures

### Success

```text
Script zero child
App zero helper
App one helper
App two helpers inserted in reverse HashMap order -> lexical receipt order
all receipt/session/physical/ledger brands equal the invocation token brand
collector rows and ledger events correspond one-to-one with receipts
Selected/NotSelected callable Main disposition unchanged; descent = 0
RootBodyCompletionTracker.completed_children = 0
```

### Failure and atomicity

```text
locator statement-index / box / method mismatch
symbol or arity mismatch
request rejection before reservation
reservation rejection before capture
natural primary lowering error
cleanup and primary+cleanup cause retention
collector admission collision
receipt-brand rejection
ledger completion mismatch
first child success + second child failure -> prefix=1, third descent=0
```

Every failure proves the live unpublished owner is retained for discard,
later sibling descent is zero, and no retry/fallback/re-entry terminal exists.

## Guard and visibility contract

```text
RawDraftInvocationV1 references in CHILDREN0 = 0
ModuleLoweringInvocationV1::with_shell_collector calls = 0
ModuleLoweringInvocationStateV1 / MainPending / capture_main / complete_root = 0
sorted_method_entries calls in CHILDREN0 = 0
HashMap method iteration in CHILDREN0 = 0
compiler-side RawRootPhysicalStateV1::into_parts = 0
compiler-side shell/collector/ledger tuple = 0
RawRootPhysicalStateV1 raw into_parts terminal = 0
RootBodyCompletionTracker begin/close/complete calls = 0
rejected retry/resume/continue/into_owner = 0
callable Main/root body lowering = 0
production child/root consumers = 0
```

Prefer an existing reusable Raw lane/batch guard assertion. Do not create a
per-cell guard unless an existing assertion is retired or merged in the same
series.

## Proof budget and sunset

```text
ceremony_tier = T2
sunset_id = CHILDREN0-PROOF-SUNSET-01
proof_inventory_before = OWNER0-PHYSICAL0 + CHILDREN0 consultation
new_proofs = child schedule, child terminal, prefix receipt, typed rejection,
             pre-root cardinality witness
retired_or_merged_proofs = none in S0
net_proof_delta = +5
sunset_budget = +5
sunset_row = CHILDREN0-G0 / CUT0-G0
retire_when = production child owner is the sole consumer and legacy child
              proof callers are zero
budget_repayment_evidence = caller census, merged lane guard, parity fixture
```

The positive proof delta is permitted here because this is a T2 owner/failure
boundary. The active closeout must repay it by merging or retiring the
disconnected child scaffolds once the production owner is wired.

## Non-claims

```text
callable Main descent
root body lowering
declaration/access installation
Main/condition root batch
drain/finalization/postprocess/external commit
typed panic retention
public ingress/JSON/CUT0 activation
```

## Evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_root_children --lib -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_children0_guard.py
```

All touched source/check files must remain below 800 lines. The next row is
BODY0 only after this child completion owner and its failure matrix are green.
