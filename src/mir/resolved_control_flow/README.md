# Resolved Control Flow

This directory is the future control-only, pre-Builder authority for canonical
resolved-source lowering.

```text
exact located syntax + VerifiedResolvedFunctionV1
  -> family-specific control product
  -> inseparable structural source coverage
  -> canonical materialization
```

## Authority boundary

This layer may own:

- exact owner/source closure;
- structural source coverage;
- ScopeId/RegionId topology;
- typed control ports and exact targets;
- cleanup obligations.

It must not own binding effects, `may_rebind` sets, carrier rows, PHI-source
rows, names, `ValueId`, `BasicBlockId`, or mutable lowering state.

The existing `resolved_region_flow` module remains the unchanged production
A+ statement-If owner until the later atomic function-wide Binding SSA
cutover. Both modules must not become production authorities for one function.

## B0-L4-S2′ generic located source coverage

`source_coverage.rs` owns one reusable ordered vocabulary for exact Body,
Statement, and Expression claims. `VerifiedLocatedSourceCoverageV1` combines
that preorder with one compiler-sealed nonempty `ConsumedSourceRangeV1`.

The verified type, fields, and constructor remain private to this module
subtree. It is not `Clone`, has no `into_parts`, and has no production
consumer. A future family product must co-seal coverage rather than pass it to
Lower as an independent argument.

S2′ proves owner closure, exact checked range transport, nonempty ordered
claims, and duplicate rejection. Exact Loop/If subtree completeness remains a
later family-product obligation. Planner, Builder, Lower, and runtime
activation remain disconnected.
