Status: selected design consultation; implementation stopped at policy boundary
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Predecessor: joinir-generic-resolved-carrier-family-overlap-census-d3-s2-p3-task-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SHARED-SOURCE-BRIDGE-DESIGN0-D3-S2-D4`
Decision: design required before any shared family classifier or production selection

# Design consultation stop

P3 is closed as independent evidence. It confirms that raw Generic
`LoopRouteContext` observations and resolved NestedPredicate/DirectAccum/A+
preflight observations do not currently share a FunctionOwnerId, source
forest, `BindingRefV1`, frame, or invocation brand. Exact family disjointness
therefore cannot be implemented by pairing fixture labels, AST names, route
IDs, or coordinates.

This row decides whether a common source bridge is warranted and, if so, which
existing owner should provide it. It is a design task only; no source or
production selection implementation is authorized yet.

# Source authority

The consultation must inspect the complete producer/classifier arms for:

```text
raw Generic: LoopRouteContext -> try_build_loop_facts -> registry schedule
resolved families: VerifiedResolvedSourceUnitV1 -> CanonicalLoweringPreflightV1
```

The design brief must name the semantic unit (whole function or loop
fragment), exact body/window membership, function/source owner identity,
execution frame, and the treatment of transferred or opaque subtrees. It must
also account for every current NestedPredicate, DirectAccum, A+, Generic,
trivial-profile, and canonical-rejection arm rather than inferring policy from
type names or one green fixture.

# Non-authority

The following are reporting evidence only and may not become a bridge key or
classifier input:

```text
fixture labels, source strings, AST names, route IDs, raw schedules
raw frame coordinates without an owner brand
plan digests, stage/debt traces, ValueId/PHI, Recipe keys, selector outcomes
```

The P3 census remains an independent report. Its
`UnresolvedStop(FamilyOverlap)` must not be reinterpreted as a winner or as a
proof that two observations describe the same source unit.

# Candidate bridge shapes

The consultation may compare, but must not implement, these bounded choices:

1. keep the families permanently separate and make disjointness a policy
   non-claim;
2. extend the existing resolver session to issue one owner/frame/source
   receipt consumed by both raw Generic observation and resolved preflight;
3. introduce a neutral source-view bridge owned below both products, with no
   route selection or physical identity.

For every candidate, record the new authority it creates, the old authority it
retires, the exact typed reject boundary, and whether a whole-function source
caller can consume one non-`Clone` receipt without loose pairing. If no
candidate satisfies those conditions, retain NoSafeSlice and do not add a
fourth census.

# Forbidden until design acceptance

```text
shared Generic/NestedPredicate/DirectAccum/A+ classifier
exact disjointness proof or winner precedence as a semantic contract
co-sealed cross-family capability without one branded source receipt
selector, eligibility, Recipe, LoopBindingKeyV1, BindingRef/ValueId/PHI
Builder, MIR, Return, ABI, Home, debt, retry, fallback, runtime, production
caller, or source AST reconstruction
```

# Acceptance

- A compact design brief names source authority, non-authority, semantic unit,
  all classifier arms, fail-fast mismatch reasons, and one counterexample.
- The recommended bridge shape either has one clear owner and retirement edge
  or explicitly rejects a shared bridge as NoSafeSlice.
- The smallest future implementation slice is named, including its
  test-only/production status and no-claim boundary.
- No implementation starts from this row until the design decision is
  accepted. Any later implementation/test landing must update affected
  `docs/reference/**` and current support pages in the same commit.
- Source/check files and the workstream remain below 800/1000 lines.

# Current next action

Worker premise audit is complete; exact disjointness is not. Read this card
with the parent D3 design and P3 census, then stop for the design decision.
