# JOINIR-LOOP-RECURSIVE-RECIPE-PRODUCER0-M7-S0-NESTED-ALWAYS-CALLER0

Status: accepted caller-zero witness slice; implementation may proceed.
Date: 2026-08-03

## Purpose

Close the smallest recursive Recipe producer seam without pretending that the
legacy `NestedLoopMinimal` route is already portable.  This row exercises the
existing nested-`Always` semantic golden through the common test-only
producer facade, then proves that D0/D1 source ownership can be attached to the
verified Recipe and re-verified as a portable artifact.

The fixture is an M6 logical nested-resume witness: one outer `Predicate` loop
and one inner `Always` loop with the existing carrier/exit obligations.  It is
not a parity fixture for the real `NestedLoopMinimal` source route.  The real
legacy route has an inner `Predicate` (`j < 3`) and remains blocked by the
existing `UnsupportedNestedPredicate` JoinSig boundary.  A separate design
row is required before that source route may claim a portable producer.

## Authority and ownership

```text
owned nested Recipe golden + opaque receipt
  -> existing test-only Recipe producer facade
  -> RecipeVerifierV1
  -> LoopJoinSigElaboratorV1
  -> D0 resolved source forest
  -> D1 source-binding projection
  -> portable artifact verification
```

The receipt is diagnostic provenance only.  It must not select a route, alter
Recipe/JoinSig semantics, or imply `NestedLoopMinimal` source parity.

PHI/SSA is untouched.  The later physical chain remains
`CanonicalCfgSessionV1 -> BindingSsaBuilderV1 -> PhiTxn` through
`MirBindingSsaAdapterV1`.  `LoopPhiMaterializerV1` is not called here and is
not a production PHI owner.

## Allowed slice

- Reuse the existing `VerifiedLoopRecipeProducerFacadeV1` test-only facade.
- Consume the existing `accum_nested_v1.json` semantic golden.
- Resolve a two-loop source fixture once, issue D0's sealed source forest, and
  pass it through D1's non-Clone source-binding projection.
- Attach `LoopRouteId::NestedLoopMinimal` only as an explicitly opaque
  diagnostic receipt in the test artifact.
- Verify source-path coverage, parent links (`[None, Some(0)]`), Recipe shape,
  JoinSig determinism, and artifact verification.

## Forbidden in this row

- No `NestedLoopMinimalFacts`, `CanonicalLoopFacts`, AST-bearing producer, or
  source re-observation after the resolver forest.
- No real route policy, `route_loop`, registry change, Retry/decline change,
  Generic classification, or legacy composer/lowerer change.
- No Builder, CorePlan, PlanLowerer, physical ID, candidate publish,
  PHI/SSA writer, or physicalizer caller.
- No renaming of the synthetic `Always` golden into a claim that real nested
  `Predicate` source semantics are supported.

## Acceptance gates

1. The common facade remains test-only and has zero production callers.
2. The source forest contains exactly two members with parent indices
   `[None, Some(0)]`; D1 retains the canonical root and child paths.
3. The facade yields two deterministic JoinSig loop rows, including inherited
   outer carrier visibility and the inner `Enter`/`Break` closure.
4. The D1-bound artifact verifies, and normalized Recipe/JoinSig values are
   unchanged when the opaque receipt changes.
5. A malformed/unreachable item still returns a typed verifier/JoinSig error;
   no retry projection is introduced.
6. Focused tests, the compile-candidate scope guard, current-state pointer
   guard, `cargo check --lib`, and all touched files under 800 lines are green.

## Next design stop

After this witness is green, stop before a real Nested producer.  The next
question is shared nested-`Predicate` JoinSig/closure semantics.  Only after
that is accepted should an AST-free resolved source projector and a genuine
`NestedLoopMinimal` producer be designed.
