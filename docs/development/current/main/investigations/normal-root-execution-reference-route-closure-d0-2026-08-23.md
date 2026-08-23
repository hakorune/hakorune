# Normal root execution reference-route closure D0

Status: design stop — route/parity selected; exact source-plan consumer open
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
  Freeze the exact paired source-plan surface and atomic cutover contract;
  reserve one behavior-neutral frontdoor split before semantic edits.
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

The canonical branch has production-shaped Main and callable-module tests, and
the classifier owns those result variants. Discarding the total relation
before this branch and then rescanning the AST would create a second
App/ProgramRuntime authority. No P0 Rust work began.

The current source-plan positives are not all green baseline evidence. Script
and empty Program reach classification, but valid Main0 and top-level-helper
CallableModule rows stop at the shared pre-profile discard with
`AppReadyRequiresNormalRootConsumer`. That red is the blocker this D0 owns;
it must not be normalized as baseline debt or bypassed by an AST-only test.

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

Census boundary (R2 source-plan parity):
`PreparedNormalFileSourcePlanRequestV1::classify` ->
`ClassifiedNormalFileSourcePlanV1 | RejectedNormalFileSourcePlanningV1`;
includes parser identity validation, every surface/Main classifier arm, and
Script/Main0/CallableModule terminals; excludes parsing before the request and
compiler dispatch after classification.

Required parity:

| Total role + source-plan surface | Existing result |
| --- | --- |
| ProgramRuntime, no callable entry, supported script statements | `ScalarRoot::Script` |
| ProgramRuntime, top-level callable but no Main | `MissingSourceEntry` |
| ProgramRuntime, non-static Box Main | `MainMustBeStatic` |
| ProgramRuntime, unsupported top-level surface/non-Main Box | existing typed unsupported reject |
| App, exact main/0 and no additional callable | `ScalarRoot::Main0` |
| App, exact main/0 plus top-level or Main helper | `CallableModule` |
| App, main arity nonzero | `MainArityMismatch` |
| App mixed with script statements | `MixedSourceFamilies` |
| App with an ordinary/non-Main Box sibling | existing typed unsupported reject |
| duplicate/foreign/incomplete total relation | mapped typed pre-effect reject |

The consumer uses preissued root/callable roles for Main ownership and only
examines already-paired statement syntax for source-plan-specific supported
surface. It may inspect a role-bound declaration name or method shape to apply
source-plan policy, but never uses name, ordinal, span, or pointer to reissue
App/ProgramRuntime or to pair foreign rows. It never invokes the old
source-backed whole-Program inventory.

`App` / `ProgramRuntime` alone is therefore not the classifier input. The
canonical consumer must issue one owned, AST-free source-plan surface while a
single HRTB loan keeps every statement/callable role paired:

~~~text
ParserNormalRootSourcePlanConsumerV1::consume_once
  (total relation + same-invocation source loan)
    -> ParserBackedNormalSourcePlanSurfaceV1
       + consumed parser source owner
    -> Script | Main0 | CallableModule | typed reject
~~~

No public role getter, repeated `&self` loan, parallel role/syntax arrays, or
generic parts tuple is permitted. `NormalSourceSurfaceInventoryV1::collect()`
remains available only to AST-only fixture/explicit compatibility ownership;
its parser-backed production caller becomes zero.

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
- The test helper that invokes source-plan classification from the narrow Raw
  profile is test-only evidence. It does not authorize a Raw production edge
  into canonical source-plan; canonical production has one sealed canonical
  profile caller.
- Source-backed and compatibility products remain different outer enum arms;
  compatibility receives no `Option`, empty/default state, or discard receipt.
- Generic source-backed `into_ast()` retires once both reference leaves use
  their route-specific handoff.

## Bounded task sequence

1. `R1-ROUTE-CENSUS` — closed
   - normal/default, canonical source-plan, Raw VM, retained/test, and
     compatibility exits are named through their terminals.
2. `R2-SOURCE-PLAN-PARITY` — closed as a design matrix
   - Script/empty currently reach classification; valid Main0 and
     CallableModule rows are classified current-change red at the shared
     discard;
   - freeze every positive/reject against total role plus paired source syntax.
3. `R3-SOURCE-PLAN-CONSUMER`
   - freeze the exact owned surface rows, one named HRTB consumer, typed result
     mapping, old parser-backed inventory retirement, and no-reissue guard.
4. `R4-RAW-DISCARD` — route selected; concrete typestate open
   - freeze Raw-profile capability, Ready-only discard, source-failure reject,
     and one AST extraction.
5. `T0-FRONTDOOR-SPLIT`
   - before semantic growth, move the inline tests/route responsibilities out
     of the 748-line frontdoor without changing type, caller, or behavior;
   - every production file remains below the 760-line design trigger.
6. `C0-ATOMIC-ROOT-CUTOVER`
   - land issuer, exact-transform preservation, normal/default consumer,
     canonical source-plan consumer, Raw-only discard, retained/test closure,
     compatibility absence, and all selected old-edge retirements atomically;
   - standalone or caller-zero P0/S0 commits are forbidden.

## Done

- route census has an explicit start/end and all branch families;
- source meaning and route lifecycle are separate;
- canonical source-plan has one named role-bound consumer and does not rescan
  total root role;
- Raw VM alone has a non-Clone Ready-only discard receipt;
- typed source failures reject before every source-backed route effect;
- retained/test helpers cannot drop the required field;
- compatibility has structural absence rather than an empty state;
- T0 has an exact behavior-neutral move manifest;
- C0 has exact files, callers, retirement edges, positive/negative tests, and
  reusable guard checks; no intermediate commit exposes a second authority.

## Stop / NoSafeSlice

- canonical source-plan still classifies Main/Script from raw AST alone;
- reference is treated as one Script-A or one discard route;
- Main0, main/N, Main helper, CallableModule, or ProgramRuntime parity is open;
- source-backed generic `into_ast()` remains a bypass;
- compatibility needs an empty/default/optional total product;
- typed source failure reaches Raw AST or source-plan inventory;
- narrow and total roots require separately observable partial transitions;
- retained/test helper destructures or drops the field;
- source-plan consumer uses name, ordinal, span, or pointer to reissue total
  role or pair independently transported rows;
- Raw discard receipt can enter canonical source-plan;
- parser P0 or consumer S0 must exist caller-zero in a landed intermediate;
- fallback/retry or Builder work is required.
