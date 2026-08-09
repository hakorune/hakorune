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

## SSA-E0 function completion

`function_control.rs` seals the accepted function completion forms before
Builder effects:

- one or more exact explicit `Return` sites, including the function-root
  terminal `Return`; or
- implicit Void fallthrough.

The explicit form carries its exact statement site, exact function-region
target, zero unreachable suffix, and an explicit ordered cleanup contract.
The E0 cleanup set is empty by design; nested exits remain rejected by the
existing capability boundary. `cleanup.rs` owns only the ordered crossed-scope
vocabulary and no runtime cleanup or value state.

The multiple-exit form seals exact source membership, common function target,
declared-result compatibility, and uniform value/unit disposition. Canonical
Lower may use that semantic receipt to open a fresh unpublished session, but
DraftSeal completion for multiple physical return paths remains deliberately
closed and fails fast. Implicit completion remains a separate variant and is
finalized only after the canonical function Lower session has finished.

## SSA-S3 carrier-free If control

`if_control.rs` owns the disconnected future statement-`If` contract. One
function product contains one source-preorder row for every exact semantic If
site. Each row co-seals its exact If/IfThen/optional IfElse topology,
fallthrough-only typed ports, one nonempty statement range, and one exclusive
structural coverage partition.

Nested If rows own their own statement/subtree claims. Their parent row owns
the containing body marker but never duplicates a child row's source claims.
The function wrapper verifies the flat partition and exposes only exact-site
lookup; coverage cannot be split from its row or recombined with another
owner.

This product contains no binding effects, `may_rebind` sets, carrier or join
rows, names, MIR identities, or Builder state. The historical A+ RegionFlow
product remains the sole production If authority until the atomic SSA-I1
cutover. The new analyzer has zero production callers.

For a selected Loop profile, `loop_owned_if.rs` verifies that every exact If
region is structurally inside that exact resolver-inventoried Loop before the
outer If ledger may be closed empty. The Loop owner must later consume those
rows; foreign outer If control rejects rather than disappearing.
