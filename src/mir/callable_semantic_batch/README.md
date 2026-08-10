# Resolved callable semantic batch

This directory owns the single parser-backed resolver batch used by callable
parameter demand and Dynamic source/lifecycle projections.

```text
RetainedParserCallableSemanticSourceV1
  -> issue_resolved_callable_semantic_batch_v1
  -> VerifiedResolvedCallableSemanticBatchV1
       retained parser source
       exact ordered forest/projection rows
```

The issuer traverses the complete declaration loan, calls
`resolve_selected_callable_forests` once, verifies every root/profile/source
projection, and only then publishes the non-Clone batch. Callers may borrow an
exact `ResolvedFunctionLoweringInputV1` inside a scoped callback; they cannot
move out syntax, the parameter catalog, a forest, or a projection.

Neutral child issuers may also borrow one complete declaration-semantic view.
That view carries source-order parameter syntax and references to the same
batch-owned resolved functions; it cannot escape its callback or own a second
forest. Parameter demand is the first such child projection.

This module owns no parameter demand, receiver/result Home ABI, Dynamic
lifecycle, Recipe key, Builder state, MIR value, physical ABI, retry, or
fallback. Those are child projections or later co-seals. A second resolver
call, name-based pairing, numeric owner repair, and arbitrary verified
constructors are forbidden.
