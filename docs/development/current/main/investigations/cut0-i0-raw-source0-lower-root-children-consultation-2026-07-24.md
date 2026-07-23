# RAW-SOURCE0 LOWER ROOT0 — CHILDREN0 consultation

Status: **Closed — Candidate CHILD-prime-r1 selected; S0 is next**
Date: 2026-07-24

## Context

`OWNER0-PHYSICAL0` is now closed. It opens one eligible-only Script/App
physical owner with one token, session, empty shell, collector, ledger, and
root tracker. It does not lower a child.

The next boundary is source-ordered child lowering. The two tempting existing
owners are rejected:

```text
RawDraftInvocationV1
  -> opens a second session/shell/collector/ledger
  -> embeds ModuleLoweringInvocationStateV1::MainPending
  -> re-discovers the first child from AST

ModuleLoweringInvocationV1::with_shell_collector
  -> also constructs MainPending state
```

Neither can consume the PHYSICAL0 owner without duplicating authority or
reintroducing the Main-only lifecycle.

## Candidate CHILD-prime-r1

```text
RawRootInvocationV1::{Script,App}
  -> RawChildrenPendingInvocationV1
       exact plan-derived child locator schedule
       same token/session/physical owner

  -> Builder sibling RawRootChildTerminalV1
       short-lived route-neutral collector loan
       child capture/lower/restore
       branded admission and ledger completion

  -> RawChildrenCompleteInvocationV1::{Script,App}
       exact child cardinality/order
       successful prefix receipts retained
```

Every transition is consuming. A child error aborts the outer invocation;
there is no fresh sibling, retry, fallback, or partial completion.

Only the following existing lower-level terminals may be reused:

```text
RawInvocationChildPortV1::complete_static_box_method_branded
LegacyFunctionPendingSessionV1::complete_before_restore
RawExpansionDraftRequestV1::legacy_discovered
branded collector admission/receipt terminal
```

The ledger abort history remains coarse (`Primary`, `Cleanup`, `Admission`,
`Panic`). The rejected owner retains the exact typed child cause, including a
primary/cleanup pair; no new DuringCleanup ledger variant is introduced.

## Questions for decision lock

### Q1 — child order authority

Should the child schedule be the deterministic lexical order produced by
`sorted_method_entries`, with the source-order claim narrowed accordingly?
The recommendation is **yes**: source-derived deterministic order, not
HashMap iteration order.

### Q2 — collector loan owner

Should `RawRootPhysicalStateV1` own the sole short-lived
`ModuleLoweringPortV1`/`RawInvocationChildPortV1` loan terminal? The
recommendation is **yes**; no compiler-side collector or shell tuple should
be exposed.

### Q3 — reservation timing

Should exact locator/declaration/request validation finish before the ledger
reservation, and should reservation happen immediately before child capture?
The recommendation is **yes**: validate first, reserve second, capture third.

### Q4 — failure mapping

Should the rejected owner preserve the exact primary/cleanup/admission typed
cause while the ledger records only its existing coarse abort reason?
The recommendation is **yes**; do not add a new ledger error variant.

### Q5 — completion products

Should Script produce a typed zero-child completion witness while App produces
an exact all-child completion product, both retaining successful prefix
receipts and forbidding later sibling descent after failure?
The recommendation is **yes**.

### Q6 — root tracker separation

Should pre-root static-helper completion use a separate
`RawPreRootChildrenCompletionV1` cardinality witness, leaving the existing
`RootBodyCompletionTrackerV1.completed_children` exclusively for BODY0 root
descent? The recommendation is **yes**: CHILDREN0 may retain the tracker
brand, but must not count helper receipts in the root-body witness.

## Decision closeout

Candidate `CHILD-prime-r1` is selected for Q1 through Q6.

The child order is the single PLAN0-derived deterministic lexical method-name
order. `RawRootPhysicalStateV1` owns the sole short-lived collector loan. Exact
locator/declaration/request validation precedes one-child reservation and
capture. Rejected owners retain the exact typed cause while ledger history
keeps its existing coarse abort vocabulary. Script and App have distinct
zero-child/all-helper completion products, and helper evidence is separate
from the BODY0-only root tracker.

The next executable row is
`RAW-SOURCE0-LOWER0-ROOT0-CHILDREN0-S0`, recorded in the linked execution task
card. Production root capture/commit and CUT0 activation remain forbidden.

## Non-claims while stopped

```text
root body lowering
callable Main descent
Main/condition batch
drain/finalization/postprocess/external commit
production ingress/CUT0 activation
```

The consultation is closed. No implementation beyond the S0 execution task is
authorized.
