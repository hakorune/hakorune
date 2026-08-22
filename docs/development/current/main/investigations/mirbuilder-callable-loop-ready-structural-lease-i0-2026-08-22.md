---
Status: fast; bounded caller-zero BoxShape implementation
Task: MIR-CALLABLE-LOOP-READY-STRUCTURAL-LEASE-I0
Date: 2026-08-22
Priority: implement one route-neutral source-bound lease with no production edge
Parent: MIR-CALLABLE-LOOP-READY-SOURCE-BOUND-STRUCTURAL-LEASE-D0
PreviousCard: mirbuilder-callable-loop-ready-source-bound-structural-lease-d0-2026-08-22.md
NextCard: none-until-Decision
---

# Callable Loop Ready structural lease I0

## Contract

This is the only implementation slice opened by the accepted B-prime
Decision. It adds a caller-zero, effect-zero transport lease. It does not
connect the raw Ready production path.

```text
CallableGenericLoopSourceFactsReceiptV1
  -> CallableLoopStructuralLeaseIssuerV1
  -> CallableLoopRouteNeutralStructuralSeedV1
  -> PreparedCallableLoopStructuralHandoffV1
  -> one HRTB CallableLoopReadyStructuralViewV1<'view>
  -> owned callback result
```

The source-facts issuer remains the sole source authority. The structural
module issues only a private seed/lease aggregate from existing proof. It must
not rerun Facts, Recipe, route selection, or `choose_route_kind`.

## Allowed edits

1. Add a route-neutral seed to `control_flow/joinir/structural_port.rs`.
   It may retain only private owner/site/root-lineage relation seals and an
   opaque structural-owner seal. Keep the existing `LoopRouteContext` port
   helper unchanged for its diagnostic-only test.
2. Add one private lease issuer and move-only handoff aggregate. The aggregate
   owns the existing source-facts receipt and seed together; it never drops or
   replaces the retained pre-effect receipt before callback return.
3. Add one HRTB callback view borrowing only existing `PlanBuildOutcome`, exact
   selection/selected proof, pre-effect receipt, owner/site summaries, and the
   opaque route-neutral port. Do not expose AST, source context, route kind,
   Builder, `ValueId`, registry, or physical receipts.
4. Add focused tests in a separate test file. The callback must not be able to
   return the borrowed view or source/AST references; compile-fail evidence may
   be a static guard if the repository has no compile-fail harness.
5. Add a reusable guard proving the issuer has no production caller and the
   lease path has no old route/planner/physical caller.

## Required negative behavior

Reject before any effect when:

```text
parent/condition/body source is unlocated
condition/body has foreign root lineage
condition site is not exact LoopCondition child
body site is not exact LoopBodyRoot child
receipt owner, schedule owner, and pre_effect owner disagree
pre_effect loop site differs from source parent site
```

The rejection is typed and terminal. It must not call
`lower_loop_or_freeze_v1`, `LoopRouteContext::new`, `route_loop`, registry,
`PlanLowerer`, or any Builder method.

## Acceptance and guards

```text
CallableLoopStructuralLeaseIssuerV1 definitions = 1
production lease callers = 0
source-facts -> lease path = focused-test only
Facts extraction = 1
Recipe/selection extraction = 1
LoopRouteContext::new/with_fn_body in lease path = 0
choose_route_kind in lease path = 0
route_loop in lease path = 0
lower_loop_or_freeze_v1 in lease path = 0
PlanLowerer/registry/Builder/ValueId in lease path = 0
pre_effect discard = 0
AST/source borrow escape = 0
all touched Rust source files < 760 lines
```

Positive evidence must show the exact owner, site, pre-effect receipt, and
selected proof are visible together only within the callback. Negative tests
must show zero effect and typed rejection for each relation failure.

## Closeout

On green evidence, update this card and the module README, commit and push the
caller-zero I0, then return to `design_stop` for the separate production
Ready-ingress/normalizer Decision. Do not delete or alter the current raw
Ready -> old route edge in this slice.

This I0 does not claim ordinary Loop lowering, physical publication, fallback
retirement, or production activation.
