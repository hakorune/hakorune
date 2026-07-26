# Nested Instance Result Emission: route reconciliation D1

```text
Decision: NESTED-INSTANCE-RESULT-EMISSION-RECONCILIATION-D1-prime-r1
Closes: NESTED-INSTANCE-RESULT-EMISSION-RECONCILIATION-D1
Status: accepted design
Opened: 2026-07-27
Predecessor: NESTED-INSTANCE-RESULT-EMISSION-CORRESPONDENCE0-P0
```

```text
Selected:
  A-double-prime

Near-term family:
  pre-loop standard unified Method route only

Loop-refresh:
  parked as a disconnected proof family

First next design row:
  RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-D0
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

## Accepted closeout

The four independent audits converge on A-double-prime.  The old two-site
series is not resumed.  The work is split into three owner series:

```text
raw source-site carry
  -> source-location transport only

unified physical Call receipt
  -> physical success/final destination only

pre-loop nested result receipt
  -> exact source contract + exact carried site + successful physical receipt
```

The first two are BoxShape work.  The third is one BoxCount semantic slice.
They must not be combined in one refactor series.

### Route correction

```text
accepted pre-loop route =
  MeStandardUnified

rejected prediction =
  MeLoweredGlobal

current pre-loop source-site authority =
  none in the production raw descent
```

The raw route may not derive a site from a call ordinal, source spelling,
callee name, AST rescan, or emitted MIR.  The location must enter before raw
syntax is destructured and advance with the same structural child descent.

### Loop-refresh disposition

```text
located GenericLoop composer =
  durable disconnected proof

production caller =
  0

current selected-loop Integer publisher =
  emit_selected_exact_i64
  retained unchanged
```

The loop publisher is not a consumer of the pre-loop receipt and is not
retired in this series. A behavior-neutral split of that disconnected
publisher would have no production payoff, so it remains parked with the
route. The following is a parked candidate order, not a mandatory schedule:

```text
GENERIC-LOOP-NESTED-RESULT-ACTIVATION0-D0
  -> choose one exact production ingress

GENERIC-LOOP-LOCATED-CLAIM-ROUTE0-S0
  -> one production claim/session caller

GENERIC-LOOP-FINAL-DST-CORRESPONDENCE0-P0
  -> requested dst / finalized dst / emitted Call.dst

GENERIC-LOOP-RESULT-PUBLISHER-D0
  -> retain or replace the direct Integer publisher

GENERIC-LOOP-RESULT-PUBLISHER0-I0
  -> only after the publisher decision
```

No loop task is on the pre-loop critical path. After the pre-loop type row,
rerun the real Stage-B guard. Open this loop decision only if loop-refresh is
still the next blocker; if the guard reaches the ownership syntax boundary,
resume `OWN-GRAM-REJECT0-HAKO0` instead.

## Executable umbrella order

### Umbrella A — raw source-site carry

```text
RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-prime-r1
```

This D0 is now closed. The source owner is the same declaration-catalog
allocation retained by the nested contract. Source navigation delegates to
the existing `SourcePathV1` / site / child-role machinery through a thin
catalog-backed raw view; a second navigation engine is forbidden.

```text
source owner =
  same declaration catalog allocation
  + exact caller
  + exact declaration

navigation =
  existing SourcePath/site/ExprChildRole/BodyChildRole authority

candidate ingress =
  one explicit crate-private request
  + one isolated unpublished draft
```

```text
RAW-SOURCE-CURSOR0-S0
  -> Builder-free common navigation kernel
  -> thin catalog-backed raw source view
  -> body/statement/expression cursor products

RAW-EXPRESSION-DISPATCH-CURSOR0-I0
  -> one 3-5 commit behavior-neutral refactor series
  -> input-view-generic sole matcher
  -> legacy AST facade unchanged

RAW-LOCATED-INSTANCE-METHOD-INPUT0-S0
  -> exact site-aware MethodCall input

RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-I0
  -> one explicit candidate raw route

RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-P0
  -> real-route location/behavior/failure parity

RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-G0
  -> source reconstruction/map/fallback zero
```

Commit budget after D0: 3-5 buildable commits.  This umbrella changes no
accepted source shape, Call instruction, result type, or process behavior.

### Umbrella B — neutral physical Call receipt

```text
UNIFIED-CALL-PHYSICAL-RECEIPT0-S0
  -> one private common physical Call terminal
  -> CompletedUnifiedCallEmissionV1(final destination)

UNIFIED-CALL-PHYSICAL-RECEIPT0-P0
  -> success/failure timing and ordinary API parity

UNIFIED-CALL-PHYSICAL-RECEIPT0-G0
  -> one Call writer; source policy zero
```

The receipt is source-neutral and non-Clone. It is issued only by the actual
generic physical `MirInstruction::Call` branch, after instruction emission
and the existing post-success commit. An outer `emit_unified_call() -> Ok` is
not sufficient because special rewrites, BoxCall, and compatibility routes are
not physical generic Call receipts.

The value-result product is:

```text
CompletedUnifiedValueCallEmissionV1
  - exact finalized ValueId destination only
```

Call-without-destination, special rewrite, BoxCall, legacy emission, and failed
`emit_instruction` produce no receipt. Existing
`PreparedUnifiedCallPostSuccessV1` behavior remains in the same order.
Ordinary callers consume and discard the receipt; the later bounded adapter
may retain it.

Commit budget: 2-3 buildable commits.  `unified_emitter.rs` is already near
the source-file limit, so preparation/receipt/tests belong in small sibling
files.

### Umbrella C — pre-loop nested receipt

Preconditions:

```text
RAW-INSTANCE-METHOD-SOURCE-SITE-CARRY0-G0 = green
UNIFIED-CALL-PHYSICAL-RECEIPT0-G0          = green
```

Then:

```text
CALLABLE-RESULT-NESTED-PRELOOP-REP0-S0
  -> exact contract/site/caller association
  -> physical Call consumer zero

CALLABLE-RESULT-NESTED-PRELOOP-REP0-I0
  -> one actual MeStandardUnified adapter
  -> successful Call creates one EmittedNestedInstanceCallV1

CALLABLE-RESULT-NESTED-PRELOOP-REP0-P0
  -> selected success, foreign/unselected reject, failed Call, fresh reuse

CALLABLE-RESULT-NESTED-PRELOOP-REP0-G0
  -> exactly one adapter and receipt producer
```

This receipt stores the final destination only.  The bridge itself has zero
`MirType` and `type_ctx` writes.  Existing unrelated annotation policy is not
redefined as globally absent.

The next semantic stop is separate:

```text
CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-D0
```

Only that later decision may select a receipt consumer that publishes Integer.
It must fix the pre-existing fact conflict law before implementation:

```text
type_ctx[dst] == None:
  publish Integer once

type_ctx[dst] == Integer:
  decide exact same-authority idempotence or typed duplicate rejection
  blind overwrite = forbidden

type_ctx[dst] == Unknown:
  decide typed conflict or explicit Unknown replacement

type_ctx[dst] == any other type:
  typed conflict

Call failure or missing receipt:
  publication = 0
```

## Failure law

```text
source cursor seal failure:
  Builder effects = 0

cursor/descent failure:
  exact source owner retained
  reclassification/retry/fallback = 0

Call preparation/emission failure:
  completed physical receipt = 0
  nested receipt = 0
  new type write = 0

successful Call:
  completed physical receipt = 1
  nested receipt = 1 only for the selected pre-loop association
  new type write = 0 until the later type I0 decision
```

Rejected owners expose inspection, bounded report, and `discard(self)` only.
They do not expose `into_owner`, resume, retry, or an alternate route.

## Structural gate

```text
pre-loop accepted physical route                     = MeStandardUnified
MeLoweredGlobal forced activation                    = 0

raw exact structural source-site carry owner         = 1 after carry G0
raw AST/source re-walk                               = 0
source-site/call-ordinal reconstruction              = 0
Builder source-site registry                         = 0
persistent source-site -> ValueId map                = 0

physical Call writer                                 = existing 1
neutral completed-Call receipt producer              = 1 after receipt G0
receipt-producing route                              = actual generic Call only
special/BoxCall/legacy/no-destination receipt         = 0
generic emitter source lookup/classification         = 0

pre-loop nested association adapter                  = exact 1 after REP0 I0
loop-refresh nested association adapter              = 0
new bridge MirType/type_ctx write                     = 0

located GenericLoop production caller                = 0
existing emit_selected_exact_i64 publisher delta     = 0
LocatedLegacyLoweringSession production activation   = 0

fallback / retry / profile reselection               = 0
all modified/new source/check files                  < 800 lines
```

## Required closeout

```text
Decision:
  NESTED-INSTANCE-RESULT-EMISSION-RECONCILIATION-D1-prime-r1

Status:
  accepted

Choice:
  A-double-prime

Near-term:
  pre-loop MeStandardUnified only

Owner sequence:
  raw source-site carry
  -> neutral physical Call receipt
  -> pre-loop nested result receipt

Loop-refresh:
  parked behind GENERIC-LOOP-NESTED-RESULT-ACTIVATION0-D0
  existing Integer publisher unchanged

Type publication:
  zero in all three near-term umbrellas
  separate CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-D0

First next design row:
  RAW-SOURCE-CURSOR0-S0
```
