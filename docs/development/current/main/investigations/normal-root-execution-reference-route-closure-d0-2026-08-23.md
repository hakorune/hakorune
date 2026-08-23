# Normal root execution reference-route closure D0

Status: design freeze accepted — behavior-neutral T0 next; C0 remains blocked
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
  Move the inline frontdoor tests byte-identically in T0, then return to
  design_stop for the exact atomic C0 file/test manifest.
Non-claims:
  No semantic Rust, fixture behavior change, lifecycle/default cutover,
  fallback, compatibility fabrication, Recipe, MIR, or publication; T0 is
  BoxShape only.

Census boundary: `ParsedProgramWithCallableParameterSourceV1::new` -> every
terminal move, destructure, retained owner, source-plan classification, or
source-backed AST extraction of that product; includes normal/default,
canonical-core reference, Raw VM reference, retained owners, and test-only
helpers; excludes downstream Builder consumers after exact final source.

## Census parking valve

The declared owner/terminal inventory is finite. Every finding enters one of
these states before it may affect this D0:

~~~text
Observed
  -> CutoverBlockerOpen -> CutoverBlockerClosed
  -> ParkedSealed -> Reopened -> CutoverBlockerOpen | ParkedSealed
~~~

A finding is `CutoverBlockerOpen` if it is inside the declared boundary, can
issue/drop/consume/reclassify/bypass App/ProgramRuntime or
Script/Main0/CallableModule, receives the unconsumed parser product, total-root
relation, or Script-A sibling, can re-enter generic source extraction/old
inventory, or is required by C0 build/test/guard/retirement acceptance. Only a
boundary-external finding satisfying none of those predicates may become
`ParkedSealed`; its row records owner, evidence, observable reopen trigger, and
non-authority.

The census closes only when the declared inventory is `Exhausted`,
`CutoverBlockerOpen | Reopened = 0`, and every external finding is
`ParkedSealed`. `ParkedSealed` means classification is closed, not that the
follow-up implementation is complete; a named trigger reopens it.

Finding a later AST getter is therefore not enough to widen the cutover. Raw
cleanup after the one authorized extraction and semantic/physical cleanup
after a sealed terminal are parkable only when the complete predicate above is
demonstrated.

### R1 owner/terminal inventory

The D0 discovery inventory is exactly these 14 rows. “Inside” rows are C0
blockers until their replacement/retirement evidence closes; “outside” rows
cannot receive the unconsumed total relation or Script-A sibling.

| # | Entry owner / symbol | Route and terminal | Boundary | Finding state |
| ---: | --- | --- | --- | --- |
| 1 | `ParsedProgramWithCallableParameterSourceV1::new` | source-backed parser owner with total relation + Script-A sibling | inside | `CutoverBlockerOpen(C0)` |
| 2 | `ParserCallableSourceDispositionV1::into_normal_callable_program` | normal/default -> exact final source/root lifecycle | inside | `CutoverBlockerOpen(C0)` |
| 3 | `PreparedNormalFileSourceV1::prepare_source_plan_request` | canonical source-backed -> Classified/RootReject/PolicyReject | inside | `CutoverBlockerOpen(C0)` |
| 4 | `PreparedNormalFileSourceV1::prepare_raw_vm_handoff` | Raw source-backed -> one Raw invocation | inside | `CutoverBlockerOpen(C0)` |
| 5 | `ParsedProgramWithCallableParameterSourceV1::into_retained_source` | retained semantic owner | inside | `CutoverBlockerOpen(C0)` |
| 6 | direct parser-product test helpers | source disposition/retained test terminals | inside | `CutoverBlockerOpen(C0 tests)` |
| 7 | compatibility `into_normal_callable_program` | normal/default compatibility terminal | outside | `ParkedSealed` |
| 8 | compatibility Raw route | named compatibility extraction -> Raw invocation | outside | `ParkedSealed` |
| 9 | compatibility canonical route | `CompatibilitySourceUnavailable` | outside | `ParkedSealed` |
| 10 | `PreparedNormalSourcePlanInputV1::new` | AST-only fixture -> plan/reject | outside | `ParkedSealed` |
| 11 | `SealedNormalScriptSourceV1::prepare_script_recipe` | sealed Script terminal consumer | outside | `ParkedSealed` |
| 12 | `SealedNormalMainSourceV1::prepare_function_source` | sealed Main0 terminal consumer | outside | `ParkedSealed` |
| 13 | `SealedNormalCallableModuleSourceV1::prepare_callable_source` | sealed CallableModule consumer | outside | `ParkedSealed` |
| 14 | Raw runtime/compiler after authorized extraction | execution/publication terminal | outside | `ParkedSealed` |

Inventory state is `Exhausted(14)` for this D0: open blockers = 6 and parked
rows = 8. C0 may close only at open blockers = 0. A newly observed entry or
terminal returns the inventory to Open and must be classified before work
continues.

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
       + SourcePlanBound parser source owner
    -> Script | Main0 | CallableModule | typed reject
~~~

No public role getter, repeated `&self` loan, parallel role/syntax arrays, or
generic parts tuple is permitted. `NormalSourceSurfaceInventoryV1::collect()`
remains available only to AST-only fixtures; compatibility does not enter this
classifier, and its parser-backed production caller becomes zero.

### R3 authority and source-plan surface freeze

R3 has two named authorities and does not call both merely “the canonical
issuer”:

- `ParserNormalRootExecutionIssuerV1::issue_once` is the total execution-role
  issuer for `App | ProgramRuntime`.
- `ParserNormalRootSourcePlanConsumerV1::consume_once` is the sole production
  source-plan observation issuer for the canonical reference route.

The source authority is the whole
`ParserCallableSourceDispositionV1::SourceBacked(ParsedProgramWithCallableParameterSourceV1)`
with Ready Program authority, total execution relation, and complete callable
relation. The existing `ParserNormalProgramSourceLoanV1` is only a temporary
HRTB observation mechanism inside the move-consuming issuer: its whole-Program
and statement AST borrows cannot escape, and its repeatable `&self` API is not
authority. Opaque parser identity pairs rows only; it cannot issue root role,
source-plan family, surface meaning, or Recipe.

The canonical route performs one affine transition before its source-plan
terminal:

~~~text
Available source-backed parser owner
  -> ParserNormalRootSourcePlanConsumerV1::consume_once
       -> one ParserNormalProgramSourceLoanV1 callback
       -> AST-free complete surface
  -> SourcePlanBoundNormalCallableSourceV1 { owner + surface }
  -> pure policy kernel
  -> Script | Main0 | CallableModule | typed reject
~~~

`SourcePlanBoundNormalCallableSourceV1` is a route-specific co-sealed owner,
not a `(surface, source)` tuple. It cannot issue a second source-plan loan,
expose role getters, or yield generic AST parts. This canonical classification
occurs on the parsed source before the normal/default exact-transform route;
the separate normal/default branch alone moves the total relation through that
transform and lifecycle consumer.

The surface distinguishes an actual empty Program from missing coverage:

~~~text
ParserBackedNormalSourcePlanSurfaceV1
  = CompleteEmpty(exact zero-coverage witness)
  | CompleteRows(NonEmptyCompleteRowsV1)

StatementRow
  = parser body row
  + root statement role tag (Ordinary | AppMain)
  + exactly one observation:
      Executable
      TopLevelCallable(exact callable relation)
      MainBoxSyntax {
        is_static,
        ordered members:
          Function { inventory key, declaration name, arity, is_static,
                     exact callable relation }
          | NonFunction { inventory key, existing kind }
      }
      Unsupported(existing NormalUnsupportedTopLevelKindV1)
~~~

`NonEmptyCompleteRowsV1` has a private constructor that proves row count > 0;
an empty slice cannot represent `CompleteRows`. `AppMain` is only a tag on the
same row: the exact root/child callable relations are moved once into that
row's `MainBoxSyntax` members and are not copied into the tag. The issuer
co-seals `AppMain` with exactly one `RootMain` member and ordered child roles.
Parallel syntax/role arrays, empty-as-default, name/ordinal re-pairing, pointer
identity, and a public parts constructor are forbidden. Names, arity, and
compatibility-order ordinals remain policy/schedule evidence only.

The pure policy kernel receives only the closed AST-free surface and admitted
total role. The parser-backed issuer is its sole production caller. A separate
AST-only surface issuer is fixture-only; compatibility remains on its Raw
owner and cannot issue canonical source-plan authority.
`PreparedNormalSourcePlanInputV1::new` has production caller zero (or is
compiled only under `cfg(test)`); an AST-only classifier production caller is
a hard guard failure.

Total-root failures do not enter the policy kernel or get collapsed into a
generic source-plan error. The reference-route rejection envelope preserves
the moved owner and exact reason:

~~~text
RejectedParserBackedNormalSourcePlanningV1
  = RootExecution {
      owner: source-plan-unbound parser owner,
      script_a: CanonicalScriptSourceRowsDispositionV1,
      exact_reason: ParserNormalRootExecutionSourceRejectV1,
      profile,
      receipt,
    }
  | Policy {
      rejected_plan,
      script_a: CanonicalScriptSourceInputDispositionV1,
      profile,
      receipt,
    }
~~~

The variants are constructor-private and non-Clone. Root failure keeps raw
Script-A ownership for its named canonical discard terminal; policy failure
keeps the already co-sealed canonical sibling. Neither may retry another
profile.

| Input state | Route-level rejection / policy result |
| --- | --- |
| `Ready(App | ProgramRuntime)` | enter the pure policy kernel once |
| `SourceAuthorityUnavailable(reason)` | typed root-execution reject retaining `reason` |
| `Incomplete(MainMethodMissing)` | typed root reject with exact diagnostic projection `MainMethodMissing` |
| other `Incomplete(reason)` | typed root-execution reject retaining `reason` |
| `IntegrityInvalid(DuplicateMain)` | typed root reject with exact diagnostic projection `DuplicateMain` |
| other `IntegrityInvalid(reason)` | typed root-execution reject retaining `reason` |
| Ready surface policy failure | unchanged `NormalSourcePlanErrorV1` |

Thus existing policy results stay stable without mapping foreign, missing, or
contradictory parser relations to an inexact default. Source-backed-unreachable
malformed AST errors remain fixture/compatibility-only.

R3 parity is closed by these rows:

| Role + complete observation | Terminal |
| --- | --- |
| ProgramRuntime + `CompleteEmpty`/Executable only | Script |
| ProgramRuntime + top-level callable but no Main | `MissingSourceEntry` |
| ProgramRuntime + non-static `MainBoxSyntax` | `MainMustBeStatic` |
| App + exact main/0 only | Main0 |
| App + top-level callable or Main helper | CallableModule |
| App + executable sibling | `MixedSourceFamilies` |
| App + main/N | `MainArityMismatch` |
| unsupported row | existing typed unsupported reject |

The pure kernel preserves the existing finite precedence:

~~~text
DuplicateMain
  -> first Unsupported row in Program order
  -> MixedSourceFamilies
  -> MissingSourceEntry / Script split
  -> Main static/method/arity/helper validation
  -> Main0 | CallableModule
~~~

Therefore ProgramRuntime + executable + top-level callable is Mixed;
ProgramRuntime + executable + non-static Main is also Mixed before
`MainMustBeStatic`; and Unsupported wins over Mixed. No map iteration order,
wildcard, or “first successful classifier” may alter this order.

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

### R4 exact route typestate

The route is bound only after one read, one parse, and source-profile closure.
Feature-disabled Usage is an earlier terminal and does not enter this table.

| State | Input | Next state / terminal | Source effect |
| --- | --- | --- | --- |
| Requested | empty path | `Rejected(Profile)` | none |
| Requested | valid Raw/Canonical profile | `PreparedRead(profile)` | none |
| PreparedRead | read failure | `Rejected(Read)` | none |
| PreparedRead | UTF-8 success | `Loaded(read=1, parse=0)` | one read |
| Loaded | parse failure | `Rejected(Parse)` | no AST product |
| Loaded | source-backed parse | `ParsedSourceBacked(root, script_a)` | one parse |
| Loaded | compatibility parse | `ParsedCompatibility(ast, sibling)` | one parse |
| Parsed | Using/Import present | `Rejected(SourceProfile)` | no route handoff |
| Parsed | no-import complete | `RouteBound(profile, product)` | none |
| Raw + SourceBacked | root Ready | sibling close -> root discard -> `RawSourceBackedExtracted` | one AST extraction |
| Raw + SourceBacked | root failure | typed Raw source reject | zero extraction |
| Raw + Compatibility | exact compatibility owner | `RawCompatibilityExtracted` | one compatibility extraction |
| Canonical + SourceBacked | root Ready | R3 -> Classified or policy reject | old inventory zero |
| Canonical + SourceBacked | root failure | typed canonical root reject | classifier zero |
| Canonical + Compatibility | compatibility owner | `CompatibilitySourceUnavailable` | inventory zero |
| wrong-route API | any owner | typed reject retaining owner | fallback/retry zero |

The no-import check observes the intact source-backed product through a scoped
Program loan and may issue only the source-profile violation. It cannot issue
root roles, source-plan rows, or discard. Compatibility uses its separately
owned syntax check; it never borrows source-backed authority.

After `RouteBound`, the outer type is closed:

~~~text
PreparedNormalFileParsedRouteV1
  = Raw(PreparedRawVmParsedSourceV1)
  | Canonical(PreparedCanonicalCoreParsedSourceV1)
~~~

For source-backed Raw, the sealed Raw profile and the whole Script-A sibling
move into one private `RawUnselectedScriptASiblingClosureV1`. It accepts every
current sibling state opaquely:

~~~text
NotApplicable | CompatibilitySource | Deferred | AdmissionMissing
| SourceAuthorityUnavailable | CohortUnresolved | ObservationIncomplete
| IntegrityInvalid | NonCandidate | HandoffReady(rows)
| MovedToParallelHandoff | DispositionTransported
~~~

Raw never matches those states into an A outcome, converts `HandoffReady` to
an error, issues candidate/noncandidate, or creates compiler transport. The
sibling closure and Ready-only root discard are consumed immediately in the
same private scope before the one source-backed AST extraction.

Compatibility Raw is a different route:

~~~text
compatibility AST owner + CompatibilitySource sibling
  -> RawCompatibilitySourceExtractionIssuerV1::issue_once
  -> RawCompatibilityExtracted
~~~

It cannot issue a root receipt, enter R3, or call source-backed extraction.
Unexpected compatibility/sibling combinations are typed parser integrity
rejects. Canonical compatibility ends at `CompatibilitySourceUnavailable`;
neither route retries the other.

Canonical keeps the Script-A sibling in the same move chain through every R3
terminal:

~~~text
PreparedCanonicalCoreParsedSourceV1 { root, script_a, profile, receipt }
  -> Classified(Script, script_a)       -> existing named A transport once
  -> Classified(Main0, script_a)        -> named non-Script sibling discard
  -> Classified(CallableModule, script_a) -> named non-Script sibling discard
  -> RootExecutionReject(script_a)      -> rejection-owner discard
  -> PolicyReject(script_a)             -> rejection-owner discard
~~~

The sibling is never cloned, implicitly dropped, re-paired, or converted by a
second co-seal. Only the Script terminal may create compiler A transport.
Main0, CallableModule, root reject, and policy reject consume it in their
named terminal with Builder/compiler effect zero. Every reject forbids
Raw/Compatibility retry.

The C0 retirement/allowlist is symbol-specific:

~~~text
shared production discard_root_before_a                          = 0
root-discard-required generic handoff constructor/assert         = 0
CanonicalParserSourceHandoffV1::into_parts                       = 0
route-before no-import disposition.ast observation               = 0
Raw `_script_input` drop                                         = 0
Raw pre-terminal NormalParserCallableSourceHandoffV1::into_ast   = 0
PreparedNormalSourcePlanInputV1::from_parser_callable_source     = 0
parser-backed NormalSourceSurfaceInventoryV1::collect            = 0
parser-backed pre-terminal PreparedNormalSourcePlanInput source  = 0
PreparedNormalSourcePlanInputV1::new production caller           = 0
co_seal_script_source_input Raw caller                            = 0
AST-only classifier production caller                            = 0
pre-terminal generic source-backed AST extraction                = 0
canonical reject -> Raw/compatibility fallback or retry          = 0
production source-plan observation issuer                        = 1
second source-plan loan after SourcePlanBound                    = 0
~~~

The pre-cutover production census is fixed to these concrete edges:

| Edge | Current count | C0 count |
| --- | ---: | ---: |
| frontdoor `discard_root_before_a` | 1 | 0 |
| `CanonicalParserSourceHandoffV1::new` root-discard precondition | 1 | 0 |
| frontdoor no-import `disposition.ast()` | 1 | 0 |
| `CanonicalParserSourceHandoffV1::into_parts` | 2 | 0 |
| Raw `_script_input` discard | 1 | 0 |
| Raw `NormalParserCallableSourceHandoffV1::into_ast` | 1 | 0 |
| parser-backed `PreparedNormalSourcePlanInputV1::from_parser_callable_source` | 1 | 0 |
| classifier -> `NormalSourceSurfaceInventoryV1::collect` | 1 | 0 |

The owning files are `normal_file_vm_frontdoor.rs`, its
`parser_source_handoff.rs` and `source_plan_input.rs` siblings, and
`normal_source_plan/classifier.rs`. T0 moves no line in this census.

Global `ParserCallableSourceDispositionV1::into_ast = 0` is deliberately not
claimed: post-terminal consumers may still lower syntax. Exactly these named
source-family consumers are allowed, once each, and may not reclassify:

~~~text
Script          -> SealedNormalScriptSourceV1::prepare_script_recipe
Main0           -> SealedNormalMainSourceV1::prepare_function_source
CallableModule  -> SealedNormalCallableModuleSourceV1::prepare_callable_source
~~~

The reusable guard therefore checks pre-terminal generic extraction zero,
production caller = 1 for each named post-terminal consumer, other production
callers = 0, and post-terminal reclassification = 0. Test-only callers are
allowed only inside the named fixture owner and are counted separately; their
existence never satisfies the production cardinality.

## Parked follow-up ledger

| State | Owner | Evidence | Observable reopen trigger | Non-authority claim |
| --- | --- | --- | --- | --- |
| `ParkedSealed` | sealed Script/Main0/CallableModule consumers | run only after an exact source-plan terminal | re-entry to classification, unconsumed total relation, or a second generic extraction | Recipe/module/physical consumers cannot issue execution role or source-plan family |
| `ParkedSealed` | Raw VM after authorized extraction | begins only after Raw sibling closure and root discard are consumed | second extraction, source reclassification, or fallback | Raw runtime owns execution, not parser source meaning |
| `ParkedSealed` | future no-import/source-plan scan-fusion optimization | begins only after C0 fixes the current no-import observer; D0 has no performance claim | compile-time evidence identifies the sealed boundary as hot | optimization cannot own source-profile or root/source-plan meaning |
| `ParkedSealed` | AST-only source-plan fixture retirement | typed separately and has no parser witness | a production caller or parser-backed authority fabrication appears | AST-only input is fixture evidence, not compatibility or canonical authority |

These rows do not close the six in-bound blockers. The manifest above is
`Exhausted(14)`, while C0 still requires every old pre-terminal edge below to
reach zero. The current no-import observation itself belongs to in-bound
inventory rows 3/4 and is not parked.

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
- Pre-terminal generic source-backed `into_ast()` retires once both reference
  leaves use route-specific handoffs; only the three named post-terminal
  syntax consumers may retain a bounded internal extraction.

## Bounded task sequence

1. `R0-PARKING-CLOSURE-D1` — design rule frozen
   - tracked policy owns the finite finding states and parking predicate;
   - C0 requires inventory `Exhausted`, open/reopened blockers zero, and every
     outside row `ParkedSealed`;
   - parked work need not be implemented before C0.
2. `R1-OWNER-TERMINAL-MANIFEST-D1` — frozen
   - normal/default, canonical source-plan, Raw VM, retained/test, and
     compatibility exits are the 14-row `Exhausted` inventory above.
3. `R2-SOURCE-PLAN-PARITY` — closed as a design matrix
   - Script/empty currently reach classification; valid Main0 and
     CallableModule rows are classified current-change red at the shared
     discard;
   - freeze every positive/reject against total role plus paired source syntax.
4. `R3-AUTHORITY-NAMING-D1` — frozen
   - total execution issuer and production source-plan observation issuer are
     distinct and each unique;
   - narrow root, AST-only fixture, and parser witness are explicit
     non-authorities.
5. `R3-SURFACE-SCHEMA-D1` — frozen
   - `CompleteEmpty | CompleteRows` and exclusive statement observations;
   - exact callable relations are nested with syntax; parallel pairing is
     impossible.
6. `R3-ERROR-CLOSURE-D1` — frozen
   - total-root failures remain exact route-level typed rejects;
   - only Ready enters unchanged `NormalSourcePlanErrorV1` policy vocabulary.
7. `R3-MOVE-POLICY-MANIFEST-D1` — frozen
   - whole source-backed parser owner moves into one HRTB consumer;
   - `SourcePlanBound` is co-sealed, reloan is impossible, parser-backed old
     inventory/pre-terminal extraction retire in C0.
8. `R4-ROUTE-TYPESTATE-D1` — frozen
   - profile/read/parse/source-profile/Raw/Canonical/Compatibility transitions
     are the finite table above; wrong-route and every failure are terminal.
9. `R4-RAW-SCRIPT-A-CLOSURE-D1` — frozen
   - all 12 Script-A sibling variants move opaquely; Raw A interpretation and
     compiler transport are zero.
10. `R4-COMPATIBILITY-SPLIT-D1` — frozen
    - compatibility has a separately named extraction issuer and cannot reach
      source-backed Raw, R3, root discard, or canonical retry.
11. `R4-CALLER-ZERO-D1` — frozen
    - every old pre-terminal symbol is listed above;
    - only three named post-terminal syntax consumers remain.
12. `T0-FRONTDOOR-SPLIT` — next behavior-neutral BoxShape
    - execution row: `NORMAL-ROOT-REFERENCE-FRONTDOOR-TEST-SPLIT-T0`;
    - move only the inline `#[cfg(test)] mod tests` body from the 748-line
      `normal_file_vm_frontdoor.rs` to
      `normal_file_vm_frontdoor/tests.rs`;
    - preserve the inner test-body bytes exactly (baseline SHA-256
      `752ad7a4cbf262cd53efdf66100e36de84ada6dde0b24f4d09267345016af28c`)
      and replace only the wrapper with
      `#[path = "normal_file_vm_frontdoor/tests.rs"] mod tests;`;
    - preserve logical module identity, 14 function names/9 test names,
      imports, the production-prefix hash
      `43a64b93561b77a1b34d1cc9cab85cb3639da7cd279ed67510e6db5c462b5592`,
      route callers, and behavior; leave `result_carrier_p0.rs` untouched;
    - parent production source becomes about 480 lines and both files remain
      below 760;
    - verify `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib
      runner::reference::normal_file_vm_frontdoor -- --test-threads=1`,
      `git diff --check`, and before/after test-name census.
13. `C0-ATOMIC-ROOT-CUTOVER`
   - land issuer, exact-transform preservation, normal/default consumer,
     canonical source-plan consumer, Raw-only discard, retained/test closure,
     compatibility absence, and all selected old-edge retirements atomically;
   - standalone or caller-zero P0/S0 commits are forbidden.

## D0 exit evidence

- the 14-row owner/terminal inventory is `Exhausted`; six implementation
  blockers are assigned to atomic C0 and eight outside rows are `ParkedSealed`;
- total execution issuer, production source-plan observation issuer, and
  parser pairing witness have non-overlapping authority;
- the source-plan surface has exact empty/non-empty coverage, one ownership of
  every relation, fixed policy precedence, and exact typed rejection;
- Raw, Canonical, and Compatibility have a finite route table; Script-A
  sibling ownership reaches every terminal without clone, implicit drop, or
  retry;
- production/test caller guards and the three allowed post-terminal consumers
  are separate;
- T0 has a byte-identical, hash-backed behavior-neutral move manifest.

## C0 acceptance — not yet claimed

After T0, a short design-stop closeout must bind the frozen contracts to exact
new owner files, positive/negative test files, and one reusable lane guard.
C0 then requires every in-bound old edge above to reach zero, every new issuer
and named consumer to have its production cardinality, all six blockers to be
`CutoverBlockerClosed`, and no intermediate commit to expose a second
authority. Until that manifest exists, semantic Rust remains forbidden.

## Stop / NoSafeSlice

- canonical source-plan still classifies Main/Script from raw AST alone;
- reference is treated as one Script-A or one discard route;
- Main0, main/N, Main helper, CallableModule, or ProgramRuntime parity is open;
- `CompleteEmpty` and missing source rows cannot be distinguished;
- source-plan surface and moved source owner can be independently re-paired;
- source-backed pre-terminal generic `into_ast()` remains a bypass;
- post-terminal syntax use is not limited to the three named consumers;
- compatibility and source-backed Raw share one generic extraction authority;
- Raw must interpret a Script-A variant or issue an A/compiler result;
- compatibility needs an empty/default/optional total product;
- typed source failure reaches Raw AST or source-plan inventory;
- narrow and total roots require separately observable partial transitions;
- retained/test helper destructures or drops the field;
- source-plan consumer uses name, ordinal, span, or pointer to reissue total
  role or pair independently transported rows;
- Raw discard receipt can enter canonical source-plan;
- `SourcePlanBound` can issue a second source-plan loan;
- parked closure is prose-only or the declared inventory cannot become
  `Exhausted`;
- parser P0 or consumer S0 must exist caller-zero in a landed intermediate;
- fallback/retry or Builder work is required.
