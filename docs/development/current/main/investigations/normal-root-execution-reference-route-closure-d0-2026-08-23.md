# Normal root execution reference-route closure D0

Status: design stop — route split selected; canonical source-plan consumer open
Date: 2026-08-23
Decision: NORMAL-ROOT-EXECUTION-REFERENCE-ROUTE-CLOSURE-D0
Owner: parser callable-source product -> route-specific affine handoff

## Six-line brief

Decision:
  Keep App/ProgramRuntime source meaning separate from transport. Preserve it
  to profile split; normal/default and canonical source-plan consume it, while
  only the explicit Raw VM leaf may issue a discard receipt.
Source authority + canonical issuer:
  Exact initial callable Program + normal Program authority + complete
  parameter catalog; `ParserNormalRootExecutionIssuerV1::issue_once`, caller 1.
Non-authority:
  Frontdoor profile, discard receipt, Script-A rows, raw/source-plan AST
  inventory, `into_ast`, narrow root state, compatibility, Builder, and MIR.
Fail-fast boundary:
  Normal/default rejects before exact transform, canonical source-plan rejects
  before source inventory, and Raw VM rejects/discards before AST extraction.
Smallest next slice:
  Freeze canonical source-plan parity and its named total-root consumer, then
  Raw-only discard typestate and the corrected atomic P0/S0 series.
Non-claims:
  No Rust/fixture work, Builder lifecycle consumer, default-route cutover,
  fallback, compatibility fabrication, Recipe, MIR, or publication in this D0.

Census boundary: `ParsedProgramWithCallableParameterSourceV1::new` -> every
terminal move, destructure, retained owner, source-plan classification, or
source-backed AST extraction of that product; includes normal/default,
canonical-core reference, Raw VM reference, retained owners, and test-only
helpers; excludes downstream Builder consumers after exact final source.

## Why P0 paused before code

The parent D0 exhaustively counted old Builder root consumers, but its declared
boundary did not include parser-product exits. The first correction then called
the shared reference frontdoor a Script-A discard route. That was also too
broad: the frontdoor fans out after parsing to two different consumers.

~~~text
shared parsed source
  ├─ FileNoImportVmReference
  │    -> Raw VM AST execution
  └─ FileCanonicalCoreVmReference
       -> NormalSourcePlanClassifierV1
       -> Script | Main0 | CallableModule
~~~

The canonical branch accepts Main and callable modules in production tests.
Discarding the total relation before this branch and then rescanning the AST
would create a second App/ProgramRuntime authority. No P0 Rust work began.

## Decision

Adopt route-specific affine handoffs:

~~~text
ParserNormalRootExecutionSourceDispositionV1
  ├─ MovedToNormalDefault
  │    -> exact transform preservation
  ├─ MovedToCanonicalSourcePlan
  │    -> named role-bound source-plan consumer
  ├─ Retained
  │    -> scoped parser borrow only
  └─ Ready(App | ProgramRuntime) + sealed Raw profile
       -> DiscardedForRawVmReference
       -> one AST extraction
~~~

Do not add `Discarded` to the source disposition. Do not use one shared
reference discard state.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| parser total-root issuer | App/ProgramRuntime and exact role relation | route selection, source-plan family, discard |
| exact normal transform | preservation of the same relation | reclassification |
| canonical reference consumer | projection to Script/Main0/CallableModule using role-bound syntax | App/ProgramRuntime reissue |
| sealed Raw reference profile | permission to close an unselected Ready relation | source meaning or compatibility fabrication |
| retained parser owner | required relation beside exact source | independent role getter or reissue |
| compatibility owner | AST-only legacy behavior | total-root product or discard receipt |

`NormalSourcePlanClassifierV1::seal()` and
`NormalSourceSurfaceInventoryV1::collect()` are not total-root authorities.
The source-backed canonical reference must not enter them with an unconsumed
total relation.

## Semantic and transport states

Semantic source states remain:

~~~text
Ready(App)
Ready(ProgramRuntime)
SourceAuthorityUnavailable
Incomplete
IntegrityInvalid
~~~

Route transport states are separate:

~~~text
Available
MovedToNormalDefault
MovedToCanonicalSourcePlan
Retained
DiscardedForRawVmReference
Consumed
RejectedBeforeEffect
~~~

Compatibility has no semantic product and is not an `Available` state.

## Finite transition table

| Input | Route | Transition |
| --- | --- | --- |
| `Ready(App)` | normal/default | move to exact transform |
| `Ready(ProgramRuntime)` | normal/default | move to exact transform |
| source failure | normal/default | typed reject before transform |
| `Ready(App)` | canonical source-plan | move to named consumer |
| `Ready(ProgramRuntime)` | canonical source-plan | move to named consumer |
| source failure | canonical source-plan | typed reject before inventory/effect |
| `Ready(App)` | sealed Raw VM | Raw-only discard receipt |
| `Ready(ProgramRuntime)` | sealed Raw VM | Raw-only discard receipt |
| source failure | sealed Raw VM | typed reject; do not discard |
| any source state | retained owner | move as required field |
| `DiscardedForRawVmReference` | Raw AST extraction | allowed once |
| `DiscardedForRawVmReference` | normal/source-plan | wrong-route reject |
| `Available` | generic source-backed `into_ast` | forbidden |
| compatibility AST | compatibility route | unchanged; no product/receipt |

## Canonical source-plan consumer contract

The named consumer is source-backed and runs before
`NormalSourceSurfaceInventoryV1::collect()`. It receives the total relation and
one paired source loan. It may project reference-specific source-plan policy,
but may not rediscover App/ProgramRuntime.

Required parity:

| Total role + source-plan surface | Existing result |
| --- | --- |
| ProgramRuntime, no callable entry, supported script statements | `ScalarRoot::Script` |
| ProgramRuntime, top-level callable but no Main | `MissingSourceEntry` |
| ProgramRuntime, unsupported top-level surface/non-Main Box | existing typed unsupported reject |
| App, exact main/0 and no additional callable | `ScalarRoot::Main0` |
| App, exact main/0 plus top-level or Main helper | `CallableModule` |
| App, main arity nonzero | `MainArityMismatch` |
| App mixed with script statements | `MixedSourceFamilies` |
| duplicate/foreign/incomplete total relation | mapped typed pre-effect reject |

The consumer uses preissued root/callable roles for Main ownership and only
examines already-paired statement syntax for source-plan-specific supported
surface. It never searches `Main` by name, zips ordinals, or invokes the old
source-backed raw inventory.

## Raw VM discard contract

Only `FileNoImportVmReferenceV1` may issue:

~~~text
ParserNormalRootExecutionRawVmDiscardV1
~~~

The private receipt consumes `Ready(App | ProgramRuntime)`, carries the opaque
same-product relation until AST extraction, exposes no role, is non-Clone, and
cannot enter canonical source-plan. Source failures reject before the receipt.
The profile is sealed before file I/O, so this is an explicit alternate route,
not fallback.

## Retained/test and compatibility closure

- `into_retained_source()` carries the relation as a required field.
- Retained APIs lend syntax/roles only in scoped callbacks.
- The caller-zero consuming `with_callable_declaration_syntax(self, ...)`
  retires or becomes retained borrowing; it may not destructure the new field.
- Tests that bind `_disposition` do not authorize a production discard API.
- Source-backed and compatibility products remain different outer enum arms;
  compatibility receives no `Option`, empty/default state, or discard receipt.
- Generic source-backed `into_ast()` retires once both reference leaves use
  their route-specific handoff.

## Bounded task sequence

1. `R1-ROUTE-CENSUS` — closed
   - normal/default, canonical source-plan, Raw VM, retained/test, and
     compatibility exits are named through their terminals.
2. `R2-SOURCE-PLAN-PARITY`
   - freeze every existing Script/Main0/CallableModule positive and rejection
     against App/ProgramRuntime plus paired source syntax.
3. `R3-SOURCE-PLAN-CONSUMER`
   - freeze one named consumer, its private HRTB input, typed result mapping,
     old source-backed inventory retirement, and no-reclassification guard.
4. `R4-RAW-DISCARD`
   - freeze Raw-profile capability, Ready-only discard, source-failure reject,
     and one AST extraction.
5. `R5-P0-SERIES`
   - revise parser P0 move chain and reserve immediate route-consumer S0;
     caller-zero total authority may live for at most one commit.

## Done

- route census has an explicit start/end and all branch families;
- source meaning and route lifecycle are separate;
- canonical source-plan has one named role-bound consumer and does not rescan
  total root role;
- Raw VM alone has a non-Clone Ready-only discard receipt;
- typed source failures reject before every source-backed route effect;
- retained/test helpers cannot drop the required field;
- compatibility has structural absence rather than an empty state;
- P0 and its immediate successor have exact files, callers, retirement edges,
  tests, and reusable guard checks.

## Stop / NoSafeSlice

- canonical source-plan still classifies Main/Script from raw AST alone;
- reference is treated as one Script-A or one discard route;
- Main0, main/N, Main helper, CallableModule, or ProgramRuntime parity is open;
- source-backed generic `into_ast()` remains a bypass;
- compatibility needs an empty/default/optional total product;
- typed source failure reaches Raw AST or source-plan inventory;
- narrow and total roots require separately observable partial transitions;
- retained/test helper destructures or drops the field;
- source-plan consumer pairs by name, ordinal, pointer, or a second AST scan;
- Raw discard receipt can enter canonical source-plan;
- fallback/retry or Builder work is required.
