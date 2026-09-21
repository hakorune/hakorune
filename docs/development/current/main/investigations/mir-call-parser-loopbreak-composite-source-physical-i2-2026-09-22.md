---
Status: design_stop__2026-09-22__CompositePackageI1AcceptedSingletonOwnerBoundary
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PHYSICAL-I2
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-package-i1-2026-09-22.md
Implementation permission: false; I1 accepted; first name the existing physical source-port and selected-target relation, then implement one bounded adapter slice
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
---

# Parser composite LoopBreak physical handoff I2

## Six-line brief

```text
  Decision: extend the existing LoopBreak source owner for composite input,
  while keeping the direct physical input unchanged; selected target/loan
  authority remains the publication owner.
Source authority + canonical issuer: the I1 composite candidate issues the
  projection plus Recipe; VerifiedStaticCallResultPublicationOwnerV1 issues
  each caller/site selected handoff exactly once.
Non-authority: AST rescans, LoopRouteContext reconstruction, names/line numbers,
  generic retry/fallback, VM routes, or a new physical/result authority.
Fail-fast boundary: exact owner/site/Recipe relation, source-port coverage,
  selected target/loan relation, one-shot take, and pre-effect validation.
Smallest next slice: map the accepted composite Recipe to the existing physical
  input and prove positive, missing, foreign, residual, and second-take guards.
Non-claims: no production caller switch, source-to-MIR publication claim,
  old-edge deletion, backend work, or warning cleanup.
```

## Dependency and ordered acceptance

I2 cannot start until I1 records a verified composite package row with no
residual candidate. The physical owner must consume that row once, validate the
source port before allocation, and retain the existing direct LoopBreak route
unchanged. A named source-to-MIR acceptance invocation is required before the
cutover row is opened.

## I1 handoff and I2 task order — 2026-09-22

I1 is now closed with the focused package evidence recorded in its card. I2
starts in design stop because the remaining uncertainty is an authority
relation, not a missing test: identify which existing LoopBreak physical owner
and `CallableLoopSourceExpressionPortV1` instance consume the composite
Recipe, and how its selected target/loan relation is carried without rebuilding
meaning from AST or names.

## Relation census and Decision — 2026-09-22

The direct physical path is singleton-target and remains unchanged. The
composite package currently cannot reach it: the lowering-state take and raw
loop entry accept only `VerifiedCallableLoopBreakSourceCandidateV1`, so a
composite loan would fall through to the generic source-facts route. The
direct `SourceLoopBreakPhysicalInputV1` and
`normal_callable_loop_source_route::into_selected_relation` also carry one
`CallableLoopSourceTargetRelationV1`; target peeking is not a consumed claim.

The selected same-owner design is therefore:

* `VerifiedCallableLoopBreakCompositeSourceCandidateV1` is the input-side
  authority and retains the resolver projection plus `BuiltRecipeTree`.
* The existing `SourceLoopBreakPhysicalInputV1`/`lower_loop_break_source`
  owner is extended for composite input; the direct three-statement contract
  is not relaxed.
* `VerifiedStaticCallResultPublicationOwnerV1::take_for_source` remains the
  only target/loan authority. It supplies the ordered caller/site selected
  handoffs and its duplicate/foreign/residual checks remain authoritative.
* `CallableLoopSourcePartsBlockV1` and the existing Parts/LoopV0 spine remain
  the physical lowering inner owner. `ClaimedCallableResultLoopBatchV1` is
  not connected because it would introduce a second target/result authority
  and its current emission lane is disconnected from LoopBreak.

The composite physical envelope must co-seal the verified Recipe, source
expression port, ordered complete selected handoffs, and cleanup/loan
terminal. It must reject owner/origin/site/forest/Recipe mismatch, missing or
foreign/duplicate target, wrong order, second take, residual claim, and any
failure before `LoopBlocksStandard5::allocate`. AST/name rescans,
`LoopRouteContext` reconstruction, generic fallback, target probes, and VM
routes remain non-authority.

The bounded I2 rows are:

1. **Relation census** — trace the existing direct LoopBreak physical owner
   from package loan to pre-allocation validation; record the exact source port,
   selected target, loan lifetime, and cleanup terminal for the composite row.
2. **Physical adapter** — after the relation is accepted, consume one
   composite Recipe through that owner, with positive, missing, foreign,
   residual, and second-take guards. Keep direct rows unchanged.
3. **Acceptance gate** — run the selected parser source-to-MIR invocation and
   record the named terminal. Only a green source-backed handoff can open I3;
   no caller switch or old-edge deletion is part of I2.

The next implementation permission is limited to this envelope and its
positive, missing-target, foreign-target, residual, and second-take guards.
No production switch, source-to-MIR completion, or old-edge deletion is
implied by the Decision.
