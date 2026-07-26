# Nested Instance Result Emission: route reconciliation D1

```text
Decision: NESTED-INSTANCE-RESULT-EMISSION-RECONCILIATION-D1
Status: design stop
Opened: 2026-07-27
Predecessor: NESTED-INSTANCE-RESULT-EMISSION-CORRESPONDENCE0-P0
```

## Why this stop exists

The accepted constrained-C-prime handoff requires two selected source sites to
reach the actual Stage-B unified Call success seam with no type publication
until a later I0.  Read-only correspondence found different facts:

| Source site | Current observed route | Production status | Existing result publication |
| --- | --- | --- | --- |
| `Body(3).Value.Argument(1)` | raw statement descent -> standard unified `Method` Call | production-shaped prefix harness; route is not `MeLoweredGlobal` | none in the focused no-header fixture |
| `Body(4).LoopBody(5).Value.Argument(1)` | located GenericLoop claim -> unified `Global` Call | disconnected; `generic_loop_located_composer.rs` says no production route or claim consumer | `emit_selected_exact_i64` writes `type_ctx[dst] = Integer` after Call success |

The unified emitter has the intended temporal shape:

```text
finalized mir_call.dst
-> successful MirInstruction::Call emission
-> PreparedUnifiedCallPostSuccessV1 commit
```

It remains only a physical Call owner.  It must not gain source-site lookup,
source classification, a persistent source-to-ValueId map, or a second Call
writer.

There is a second independent pre-loop blocker.  The actual raw MethodCall
descent has no `SourceExprSiteV1`: `RawLegacyMethodCallInputV1`, its syntax
view, and `AssociatedMethodCallArgumentsV1` carry receiver/method/arguments
only.  The location-aware method input is owned by the disconnected located
session.  Therefore the planned source-associated `{ caller, site }` input
cannot be issued honestly from the current production raw route without either
forbidden source re-walk/ordinal matching or a new location-carrying raw
descent contract.

## Fixed evidence

```text
LocatedLegacyLoweringSessionV1 = disconnected, not a production activation
located GenericLoop composer   = disconnected, no production claim consumer
pre-loop raw descent           = standard unified Method Call, not MeLoweredGlobal
loop selected-call lowerer     = current post-Call Integer type producer
```

Therefore the previous sequence may not proceed to P0-A while claiming:

```text
two production route adapters = 1 each
pre-loop exact route-owned SourceExprSiteV1 = 1
type_ctx write in P0          = 0
GenericLoop producer          = 0
```

## Decision required

Choose one route truth before any code changes.

### A — split the work by actual ownership (recommended)

Keep the constrained-C-prime bridge only for the pre-loop standard unified
Method route, **only after** a separate source-site-carry decision gives that
real raw descent one exact non-reconstructed location authority.  A narrow
correspondence probe must then prove final `mir_call.dst` to emitted `Call.dst`.
Park loop-refresh behind a separate production-route activation and
result-publication retirement decision.

```text
pre-loop:
  exact raw Method route + explicit source-site carry contract
  -> constrained success receipt proof
  -> no type publication in the bridge

loop-refresh:
  disconnected proof route remains disconnected
  -> first select a real production ingress
  -> decide whether its existing selected-call Integer publisher is retired,
     retained outside the bridge, or replaced by a later I0 owner
```

This keeps a fake production claim out of the handoff and does not move an
existing type writer under a receipt-only row.

### B — make the loop route production first

Authorize a separate route-activation series that gives the located GenericLoop
composer one exact production caller and makes its claim lifecycle explicit.
Only after that series proves call/result semantics may the handoff be revisited.

It must also decide the existing `emit_selected_exact_i64` type publication
before connecting a receipt bridge.  This is larger than the current P0 and
may not be smuggled into it.

### C — park the nested-instance bridge

Keep the source-only Integer contract as a disconnected proof and retire the
P0 handoff proof consumer.  No Call receipt or type publication is added.

## Required answer

1. Is the initial bounded target the pre-loop standard unified Method route
   only, or must it include loop-refresh in the same series?
2. Which new or existing raw-descent owner carries an exact `SourceExprSiteV1`
   for the pre-loop MethodCall without AST re-walk, ordinal reconstruction, or
   activating the disconnected located session?
3. If loop-refresh is retained, which owner has authority for its existing
   post-Call Integer publication, and is that publisher to be retired before
   any receipt bridge is connected?
4. If one route is activated, what is the exact production caller and what
   evidence proves it is not a test-only/disconnected route?
5. Must the later success receipt remain observation-only until I0, with every
   `type_ctx` write outside it?  The recommended answer is yes.

## Non-claims

```text
PreparedNestedInstanceResultEmissionV1 implementation
EmittedNestedInstanceCallV1 implementation
new unified-emitter entry point
MirType::Integer publication move
GenericLoop production activation
LocatedLegacyLoweringSessionV1 activation
raw MethodCall source-site carry implementation
Builder-wide source map
persistent source-site -> ValueId map
fallback, retry, source-policy lookup in unified emitter
```
