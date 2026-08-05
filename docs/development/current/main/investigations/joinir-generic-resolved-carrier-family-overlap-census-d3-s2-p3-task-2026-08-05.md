Status: selected bounded design/test-only census; exact disjointness remains NoSafeSlice
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Predecessor: joinir-generic-resolved-carrier-facts-snapshot-d3-s2-p2-task-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-OVERLAP-CENSUS0-D3-S2-P3`
Decision: selected bounded evidence only; no exact partition, selector, or production caller

# Purpose

Record the current Generic/NestedPredicate/DirectAccum/A+ family boundary
without pretending that the existing products form one branded disjointness
classifier. The P3 premise audit found that Generic observations come from
`LoopRouteContext -> try_build_loop_facts ->` the raw registry, while
NestedPredicate/DirectAccum/A+ are resolved by
`CanonicalLoweringPreflightV1` with ordered probes. They do not share a
FunctionOwnerId, source forest, `BindingRefV1`, frame, or invocation brand.

The existing green overlap test proves ordered precedence over an overlapping
envelope; it does not prove exact disjointness. A+ is also a post-trivial
fallback profile, and loop statements are rejected by its whole-unit body
verifier. Therefore the task must be an independent evidence census, not a
new classifier or a co-sealed family capability.

# Contract

```text
existing resolver/preflight evidence  -> independent resolved-family column
existing raw Generic observation       -> independent raw-fragment column
fixture label / test identity          -> join for reporting only
```

Permitted dispositions are observation-only:

```text
ObservedOverlap
UnresolvedStop(FamilyOverlap)
NoStandaloneRow
NotYetObserved
CanonicalRejected
RawFragmentAbsent
```

Fixture labels may align rows for a human-readable matrix, but they are not a
semantic pairing key. No source string, AST name, route ID, frame coordinate,
or raw `LoopRouteContext` field may be used to co-seal the two columns.

# Evidence scope

The minimum matrix records existing evidence independently for:

1. the nested probe / DirectAccum overlapping envelope;
2. a DirectAccum source admitted by its capability test;
3. an A+ non-loop function after the trivial-profile stop;
4. the Generic raw natural-Both observation.

The matrix must preserve the existing typed rejection and precedence facts,
but it must not reinterpret them as a winner or family selector.

# Forbidden

This row must not add or modify:

```text
Generic/NestedPredicate/DirectAccum/A+ shared classifier
exact disjointness or partition proof
new source bridge or FunctionOwnerId/forest/frame brand
co-sealed cross-family capability
winner, selector, Legacy, eligibility, Recipe, LoopBindingKeyV1
BindingRef/ValueId/PHI, Builder, MIR, Return, ABI, Home, debt
retry, fallback, runtime route, or production caller
```

If exact disjointness is requested, or if a single non-`Clone` product must
consume both family columns, stop and reopen the parent D3 design card. The
raw Generic route and resolved preflight route need a separately designed
source bridge before they can share authority.

# Acceptance

- A machine-readable, independent-column matrix or equivalent cfg(test)-only
  witness records the four bounded evidence classes and the typed observation
  dispositions above.
- The overlapping-envelope test remains explicitly labelled as precedence,
  not disjointness; the A+ fallback and loop-body rejection remain explicit.
- No production import or caller is added; no existing Generic or canonical
  facts type is changed.
- Focused evidence remains green and any mismatch rejects before effects.
- The same implementation/test commit updates affected `docs/reference/**`
  and current support pages. Reference updates are mandatory immediately after
  the implementation/test landing, not a deferred follow-up.
- Source/check files remain below 800 lines and the workstream remains below
  its 1000-line hard boundary.

# Stop / next decision

This task is complete when the independent census is packaged and the parent
design stop can decide whether a future source bridge is worth opening. It
does not authorize Generic selection or lowerer work. Until a shared branded
source authority exists, exact family disjointness and production selection
remain NoSafeSlice.
