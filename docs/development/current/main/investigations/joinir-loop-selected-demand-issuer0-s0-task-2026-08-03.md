# JOINIR Loop Selected-Demand Issuer S0

Status: Green, caller-zero implementation slice.

Task: `JOINIR-LOOP-SELECTED-DEMAND-ISSUER0-S0`

## Purpose

Close the neutral handoff between the already-frozen route policy and the
caller-zero recursive Recipe producer. This slice binds ownership; it does
not create a Recipe, choose a family, lower MIR, or touch PHI/SSA.

## Owner and API

Place the issuer in `src/mir/loop_structural_facts/selected_demand.rs`, next to
the existing resolved-source adapter. The consuming boundary is:

```text
VerifiedLoopPolicyWinnerV1
  + VerifiedLoopStructuralFactsV1
  + VerifiedResolvedLoopSourceV1
  -> VerifiedSelectedLoopRecipeDemandV1
```

All three inputs are non-`Clone` and carry private seals. The selected demand
consumes each exactly once. S0 proves facts/source identity matching plus
linear winner consumption; the winner's current raw-cursor seal does not yet
carry a shared execution-frame brand. A full three-way brand is a later design
stop, not a claim of this slice. The selected demand contains only the sealed
handoff capabilities and an opaque migration receipt for diagnostics. It must
not expose a route/family dispatch key.

`VerifiedLoopStructuralFactsV1` is intentionally minimal in S0: an owned,
AST-free identity witness plus a private seal. It is not `CanonicalLoopFacts`,
not a Recipe, and not a new intermediate representation.

The issuer compares the structural owner identity with
`VerifiedResolvedLoopSourceV1::into_parts()`:

```text
(FunctionOriginV1, SemanticOwnerSourceKindV1, SourceStmtSiteV1)
```

The policy cursor and route receipt are provenance only. They must never be
used for semantic selection, reverse lookup, or family dispatch.

## Accepted and rejected cohorts

- Direct Accum: one test-only positive handoff fixture; production callers
  remain zero.
- Nested: no source-bound demand is issued in S0. The current root-only source
  capability cannot represent a recursive forest, so Nested remains a typed
  non-claim until a sealed root plus exact child forest capability exists. Do
  not synthesize child paths from raw indices or AST.
- LoopTrue / LoopCond: remain behind their shared logical JoinSig closure.
- Generic V0/V1: remains behind the independent M4 debt classification.

## Required gates

1. Positive Direct Accum handoff consumes all three capabilities once.
2. Negative fixtures cover no winner, blocked/exhausted policy, and
   facts/source identity mismatch. Nested source-bound remains caller-zero and
   is explicitly not claimed until the forest owner exists.
3. The issuer and policy layer have zero imports of Recipe, AST,
   `CanonicalLoopFacts`, Builder, CorePlan, PlanLowerer, PHI, Binding SSA,
   Retry, scheduler, physicalizer, or Generic debt machinery.
4. The selected demand has exactly one test-only consumer; production
   `route_loop`, scheduler, physicalizer, and `LoopPhiMaterializerV1` callers
   remain zero.
5. Existing policy, M3 parity, logical-demand, pointer, and release-build
   gates remain green; touched Rust files stay below 800 lines.

## Stop conditions

Return to the design stop immediately if the issuer needs AST reconstruction,
route/family dispatch, a second policy evaluator, source-path manufacture, a
Recipe builder, physical IDs, PHI/SSA state, Retry, Generic debt conversion,
or a production caller. Those belong to later owners and milestones.

Do not promote this S0 to a production caller until the Direct Accum facts
producer supplies an AST-free typed shape and the missing execution-frame
brand decision is closed.
