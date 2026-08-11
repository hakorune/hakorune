# Resolved callable semantic batch

This directory owns the single parser-backed resolver batch used by callable
parameter-contract and Dynamic source/lifecycle projections.

```text
VerifiedFinalCallableProgramSourceV1
  Program + complete callable anchors + parameter-source subset
  -> issue_resolved_callable_semantic_batch_v1
  -> VerifiedResolvedCallableSemanticBatchV1
       final parser source
       every final callable exactly once
       exact ordered forest/projection rows
```

The issuer traverses the complete final-anchor declaration loan, calls
`resolve_selected_callable_forests` once, verifies every root/profile/source
projection, and only then publishes the non-Clone batch. Callers may borrow an
exact `ResolvedFunctionLoweringInputV1` inside a scoped callback; they cannot
move out syntax, the parameter catalog, a forest, or a projection.

Parameter syntax is an exact partial projection onto private batch slots. The
loan preserves optional declared-type spelling as borrowed source syntax; it
does not classify parameter ABI/Home demand or create a physical ValueId.
Missing parameter-source evidence does not remove a top-level, selected-gate,
or generated callable from the batch and does not synthesize `Ordinary`.
Child issuers that require parameter contracts must fail closed for a selected row
without it. The older retained parser parameter product remains a disconnected
parser test substrate and is not a semantic-batch input.

Neutral child issuers may also borrow one complete declaration-semantic view.
That view carries optional source-order parameter syntax and references to the
same batch-owned resolved functions; it cannot escape its callback or own a
second forest. Parameter contract is the first partial child projection; its
source spelling classification belongs only to the downstream contract issuer.

This module owns no parameter contract, receiver/result Home ABI, Dynamic
lifecycle, Recipe key, Builder state, MIR value, physical ABI, retry, or
fallback. Those are child projections or later co-seals. A second resolver
call, name-based pairing, numeric owner repair, and arbitrary verified
constructors are forbidden.
