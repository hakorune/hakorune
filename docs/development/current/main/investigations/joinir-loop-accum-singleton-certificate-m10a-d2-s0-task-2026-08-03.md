---
Status: Accepted task — source-side exclusivity certificate before policy admission
Date: 2026-08-03
Decision: choose the typed singleton-observation product; do not promote the test-only winner
Related:
  - joinir-loop-accum-production-issuer-m10a-d2-design-stop-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
---

# DirectAccum singleton certificate: M10a/D2/S0

## Objective

Issue one builder-free, source-owned proof that a resolved DirectAccum loop is
the sole accepted candidate for this pilot. The proof must precede policy
schedule construction and must not reuse legacy `CanonicalLoopFacts`.

The product is conceptually:

```text
VerifiedResolvedLoopSourceV1
  + VerifiedLoopStructuralFactsV1::DirectAccum
  + VerifiedDirectAccumDisjointnessV1
  + matching owner/frame/site
  -> VerifiedDirectAccumSingletonObservationV1
```

The product carries no AST clone, `MirBuilder`, `ValueId`, `BasicBlockId`, raw
cursor, route-name dispatch, or PHI/SSA state. It is consumed once by the
future policy admission issuer.

## Disjointness contract

`VerifiedDirectAccumDisjointnessV1` must prove, before Builder effects, that
the exact DirectAccum source is not an accepted SimpleWhile, Generic, nested,
or other route candidate for this pilot. Shape alone, loop count alone, and a
copied legacy schedule are insufficient.

The certificate may use a source-side structural classifier, but that
classifier must be an analysis-only view over the resolved source. It must not
reconstruct names/paths, rescan AST outside the already-owned source view, or
call the legacy registry/composer/planner.

If any excluded route cannot be proven to decline pre-effect, the certificate
must reject with a typed overlap disposition. It must never fabricate an
18-row decline matrix.

## Implementation order

1. Define the non-Clone observation/disjointness types in the neutral
   structural-facts boundary.
2. Add the source-side issuer that co-seals owner, frame, source identity,
   DirectAccum shape, and exclusivity proof.
3. Add focused tests for exact singleton acceptance, each overlap mutation,
   foreign frame/owner rejection, and one-shot consumption.
4. Add a parity-only test against the existing legacy schedule as an oracle;
   the legacy path remains test evidence, never a production input.
5. Only after S0 is green, add the policy-side admission wrapper that builds
   the canonical 19-row schedule internally and calls the existing evaluator.

## Gates

- `VerifiedDirectAccumSingletonObservationV1` is Builder-free and non-Clone.
- No production caller reaches `issue_policy_winner_for_test_with_frame`.
- No test fixture or source name is used as a route selector.
- DirectAccum fixture proves one candidate; SimpleWhile/Generic/overlap
  mutations reject or remain explicitly unadmitted.
- Frame, owner, and source-site mismatch fail before any physical effect.
- Existing PHI/SSA owner remains untouched: the product contains no physical
  identity and does not create a second session/ledger.
- Every touched Rust file remains below 800 lines.

## Non-claims

This task does not add `CanonicalFirstFamilyPlanV1::DirectAccum`, change
`CanonicalTrivialSsaLowererV1`, wire `route_loop`, remove Retry, classify
Generic V0/V1, or retire the legacy Accum/PHI edge. Those become eligible only
after this certificate and the subsequent policy admission are green.

## Consultation decision

The worker audit compared three designs. The chosen one is this typed
source-side certificate. A full 19-row resolved observation product is a
future generalization; a resolver-only loop-count certificate is rejected
because loop count does not prove SimpleWhile/Generic disjointness. If the
source classifier cannot issue the exclusivity proof without importing legacy
facts, this task remains `NoSafeSlice` and the full observation product must be
designed instead.

## Caller-zero implementation progress

The source-side slice is now caller-zero green. The resolved DirectAccum
projector issues `VerifiedDirectAccumDisjointnessV1`, and the structural-facts
boundary co-seals it with the exact source/frame facts as
`VerifiedDirectAccumSingletonObservationV1`. The observation is non-Clone and
contains no physical identity or route cursor. Focused structural-facts and
DirectAccum projection tests, the DirectAccum family tests, binary check, and
current-state pointer guard are green.

This does not yet authorize policy schedule construction. The next slice must
consume this observation in the policy SSOT, prove the canonical row matrix,
and keep Generic/overlap rows typed rather than silently converting them to
decline.
