---
Status: Script lookup reownership and source coverage I0 are pushed as `2fa560cd8f`; A/C cutover remains a bounded design stop
Date: 2026-08-21
Decision: SCRIPT-A-CUTOVER-I0-R0
Parent: docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md
ProductionCaller: NormalDefaultPublishedPipelineV1::compile reaches the selected-normal root; lookup is pre-effect, but resolver/Bundle/Recipe currently run after Builder preparation and no A/C consumer is open
ReplacementCell: source package + neutral window + owned lookup + resolver outcome -> private A capability -> immediate A consumer -> one C disposition -> named direct/non-direct consumers
Classification: T2 BoxCount design stop after lookup reownership; A/C product, Recipe retirement, fallback, physical cutover, and production switch remain closed
NextCard: design SCRIPT-A-CUTOVER-I0-R0, beginning with the pre-effect source handoff
---

# SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0

## Six-line brief

Decision: Accept the parser-owned opaque non-`Clone` preservation token and
keep downstream `SCRIPT-A-CUTOVER-I0-R0` in `design_stop` until its A/C
authority is named. Lookup reownership is closed. The selected fast cell is
the smaller source-coverage handoff below; it transports complete source
coverage without issuing A/C meaning.

Source authority + canonical issuer: the source package HRTB loan, neutral
window issuer, `ScriptDirectStaticCallLookupIssuerV1`, resolver complete/typed
deferred outcome, and retained continuation are the only admissible inputs.
One private `CanonicalScriptASourceCapabilityIssuerV1` must co-seal them once;
one private `CanonicalScriptAObservationIssuerV1` must consume the capability
immediately. One `CanonicalScriptCDispositionIssuerV1` then emits either the
named direct-static disposition or a complete zero/non-direct disposition.

Non-authority: the lookup relation alone, `VerifiedScriptDirectStaticResultBundleV1`,
`ClaimLedger`, `StaticResultPublicationIngress::{Available, Absent, Selected}`,
empty Bundle/Join maps, resolver pointer relations, Builder work-plan/semantic
products, old Recipe, physical/publication owners, fallback, and local green
tests cannot issue A or C meaning.

Fail-fast boundary: the A/C selector must run after source package/window,
owned lookup, resolver outcome, and terminal/continuation are co-sealed, but
before `install_pinned_text_target_capability`, `prepare_normal_default_module`,
Builder effects, Bundle/Recipe/Join, physical work, raw, retry, or fallback.
The current lifecycle does not yet have this boundary because resolver and
downstream products are issued after Builder preparation; moving that boundary
is part of the next bounded task.

Smallest next slice: `SCRIPT-A-SOURCE-COVERAGE-I0` extends the one
source-package HRTB observation with explicit `CompleteEmpty`/`CompleteRows`,
all bounded MethodCall carriers, parser provenance, and source-route evidence.
It does not select targets or issue A/C. The pre-effect A/C capability remains
closed until this source coverage product is transported and audited.

Non-claims: no parser cohort expansion, generic result redesign, Recipe/Join
redesign, physical Call/publication change, compatibility/raw retirement,
fallback, production switch, ABI, backend, performance, or old Recipe deletion.

## Corrected decision

The capability shape from the external review remains useful only after a
smaller semantic unit exists. The previous card revision treated a whole
Program as either `Membership.Candidate` or `Membership.Residual`. That is not
total for the first real positive:

```hako
static box Helpers {
    run(x) { return x }
}
return Helpers.run(1)
```

One Program simultaneously owns three normal responsibilities:

1. the `Helpers.run/1` method declaration/body is transferred to the callable
   catalog, resolver, target, and result authorities;
2. the `Helpers` Box wrapper, source order, registration/materialization, and
   existing static-Box runtime terminal remain live; and
3. the root `MethodCall` subtree is a direct-static candidate while its
   enclosing `Return` and result relation remain the Script terminal.

Calling the Program `Residual` loses the candidate call. Calling the whole
Program `Candidate` loses the provider transfer and retained continuation.
Provider rows and retained terminals are therefore not residuals: they are
ordinary members of the same complete composite Program.

The exact A atom is:

```text
one root MethodCall source site
+ exact static provider declaration/result row
+ receiver / ordered argument / result sites
+ enclosing retained terminal relation
```

The Program-level outcomes are:

```text
CompleteCompositeWithCandidates
CompleteCompositeZeroCandidates
OutsideBoundedCohort(reason)       # capability-before only
ObservationDeferred(typed cause)  # located or explicitly unlocated
Incomplete(missing role/coverage/catalog row)
IntegrityInvalid(foreign/duplicate/stale/contradictory relation)
```

`CompleteCompositeZeroCandidates` is not an empty/default catalog and does
not authorize old Recipe after capability starts. `OutsideBoundedCohort` may
reach compatibility only if a pre-effect owner, exact surface, registry row,
sunset row, and `retire_when` condition already exist.

## Source census and contradiction

The current source proves why implementation remains closed.

- Parser callable-source admission already retains static-only and mixed
  ordinary/static programs as source-backed callable products. This is
  existing language behavior, not evidence that canonical pure-Script
  admission owns the same shape.
- Canonical Script cohort/source rows classify every `BoxDeclaration` as
  compatibility, and `normal_source_plan::inventory` additionally records a
  non-Main Box as unsupported. The canonical source-plan classifier therefore
  rejects the natural positive before Script.
- Selected normal/default classifies the same item as
  `CatalogedNonMainStaticBox` and gives it a real runtime terminal through
  `PreparedRawNonMainStaticBoxLifecycleV1`.
- `SelectedScriptProgramOccurrenceV1` has only `None`, `TopLevelCallable`, and
  `InstanceBox` transfer states. It cannot express static callable transfer.
- `ScriptRootSemanticDecisionV1` consequently maps the static Box through its
  catch-all `Deferred(ExistingRuntimeResponsibility)` while retaining the
  runtime terminal.
- The resolver collapses detailed source deferral to unit
  `ResolveScriptForestOutcomeV1::Deferred`; the lifecycle then stores
  `script_source = None`, so the natural Program cannot issue the selected
  direct-static bundle.
- Existing focused target/bundle tests often use a Script root and a separate
  declaration fixture. They prove kernels, not one-source production
  correspondence.
- The parser product does already hold callable admission and Script-row
  siblings together, but the selected-normal materializer calls
  `into_source_disposition()` and drops the Script-row sibling. The handoff
  also receives no parser witness, and `into_normal_callable_program()` drops
  admission/rows again.
- The callable transform currently verifies callable slots/declarations and
  constructor preservation only. Its `Compatibility` branch can consume the
  source-backed AST without carrying Script-site identity, while
  `VerifiedFinalCallableProgramSourceV1` and `NormalCompileRequestV1` expose no
  composite source payload. Therefore parser identity cannot be paired with
  the transformed AST by assumption.

Thus the earlier “source admission or lookup reownership” choice was wrong.
The natural positive requires both responsibilities, in dependency order and
never mixed into one unbounded slice.

## Parser-composite preservation contract

The first source-admission task has one additional T2 prerequisite. It is not
a second semantic authority; it is the preservation rule for the parser's
single source authority:

```text
parser issuer:
  opaque parser witness
  + static provider declaration/result identity
  + root MethodCall
  + receiver / ordered arguments / result
  + enclosing retained terminal

must move unchanged:
  ParsedProgramWithCallableParameterSourceV1
    -> PreparedNormalCallableProgramSourceV1
    -> transform_normal_callable_program_v1
    -> VerifiedFinalCallableProgramSourceV1
    -> NormalCompileRequestV1
```

The transform boundary must validate missing, foreign, duplicate, stale, and
drifted source rows before the default request is issued. A compatibility
branch that discards the source authority, a `None` witness, AST/name/ordinal
repair, or a second parser scan is a typed stop. The source payload remains
AST-free after the parser loan and carries no target, candidate, Recipe, or
physical identity.

### Owner and stage contract

`VerifiedFinalCallableProgramSourceV1` is explicitly a transport container,
not the new source authority. The parser finalizer issues one private,
non-`Clone` `ParserCompositeSourcePreservationV1` through an internal HRTB
loan; the owned token then moves through every later stage:

```text
ParserCompositeSourceIssuerV1
  -> ParsedProgramWithCallableParameterSourceV1
  -> PreparedNormalCallableProgramSourceV1
  -> transform boundary
  -> VerifiedFinalCallableProgramSourceV1
  -> NormalCompileRequestV1
```

The token co-seals the opaque witness, provider callable identity, one nested
root-call row (receiver/ordered arguments/result), and its enclosing terminal.
The handoff witness is required, never `Option`; transform compatibility loss
is `CompatibilityLoss`, never AST fallback. The token's finite source states
are `Ready`, `OutsideBoundedCohort`, `SourceAuthorityUnavailable`,
`Incomplete`, and `IntegrityInvalid`; transform rejects separately as
`WitnessChanged`, `ProviderChanged`, `RootCallChanged`, `ReceiverChanged`,
`ArgumentChanged`, `ResultChanged`, `TerminalChanged`, or `CompositeDropped`.
No A/C/Recipe/physical meaning enters this token.

### Source-site identity decision

AST `Span` is rejected as the token's source-site identity. It has no parser
invocation or owner, `Span::unknown()` aliases unrelated nodes, and macro
generated nodes may carry unknown spans. The existing MIR
`SourceExprSiteV1`/`SourceStmtSiteV1` are resolver-side structural paths, not
parser-issued authority, so they cannot be imported as an identity shortcut.

The minimum missing product is one parser-owned co-sealed site tree:

```text
ParserCompositeSourcePreservationV1
  = ParserInvocationWitnessV1
  + provider callable source site
  + one root MethodCall site
      + receiver
      + ordered argument rows
      + result relation
  + RootReturn or FinalSequence terminal relation
```

The witness is the primary identity. Private structural paths are only
coverage/integrity evidence inside that witness, and are carried as one nested
tree so callers cannot zip independent arrays. `Incomplete` covers missing
provider/call/receiver/argument/result/terminal rows; `IntegrityInvalid` covers
foreign witness, duplicate site, provider-source mismatch, malformed call-tree
relations, order mismatch, or terminal drift. Span is diagnostic evidence only.

### Accepted parser product and sole issuer

The source product is a total disposition, never an optional token:

```rust
enum ParserCompositeSourceDispositionV1 {
    Ready(ParserCompositeSourcePreservationV1),
    OutsideBoundedCohort(ParserCompositeOutsideReasonV1),
    SourceAuthorityUnavailable(ParserCompositeSourceUnavailableV1),
    Incomplete(ParserCompositeIncompleteV1),
    IntegrityInvalid(ParserCompositeIntegrityIssueV1),
}

struct ParserCompositeSourcePreservationV1 {
    invocation: ParserInvocationWitnessV1,
    provider: ParserCompositeStaticProviderV1,
    terminal: ParserCompositeRootTerminalV1,
    _seal: ParserCompositeSourcePreservationSealV1,
}
```

The token and seal are private and non-`Clone`. Cloneable declaration identity
or structural coordinates nested inside it remain comparison/coverage evidence;
they do not make the whole authority replayable.

The bounded `Ready` cohort is exact:

```text
Program
  one non-Main, non-sync static Box
    one ordinary direct method
      parser callable anchor + source site
      declared result syntax = Implicit | Explicit(type syntax)
  final statement
    FinalSequence(MethodCall)
      or RootReturn(value = MethodCall)
      receiver = one exact source subtree
      arguments = exact parser arity in source order, including proven zero
      result = this MethodCall
```

`Implicit` is an observed source state, not missing data. The provider result
syntax is copied under the parser's existing exact declaration loan; it is not
inferred from the method body. Receiver and method names may be retained as
owned syntax/diagnostic inputs, but never as identity or a provider/call join.
The parser proves provider/call co-presence only; it does not prove that the
call resolves to that provider.

The terminal owns the call tree. Its receiver, ordered argument roles, call
result, and terminal relation are not exposed as independently pairable arrays.
Private structural locators are meaningful only under the token's invocation
witness. A real zero-arity syntax issues an empty argument slice; a failed
argument observation is `Incomplete`, never empty.

The sole production constructor is:

```rust
ParserCompositeSourceIssuerV1::issue(
    &CompletedParserPostpassV1,
    &ParserCallableParameterSourceDispositionV1,
) -> ParserCompositeSourceDispositionV1
```

It is called exactly once inside
`ParsedProgramWithCallableParameterSourceV1::new`, where the completed AST,
verified callable anchors, parameter catalog, and parser brand still belong to
one invocation. There is no `from_parts`, public constructor, AST replay,
transformed-AST issuer, or source-string rescan.

### Move and transform contract

The required disposition moves on one spine:

```text
ParsedProgramWithCallableParameterSourceV1
  -> ParserCallableSourceDispositionV1::SourceBacked
  -> PreparedNormalCallableProgramSourceV1
  -> transform parts
  -> VerifiedFinalCallableProgramSourceV1
  -> PreparedNormalDefaultProgramRootV1
  -> NormalCompileRequestV1
```

It is a required field on the parser-backed products above. It is not placed in
the existing optional handoff witness, duplicated as a parallel request field,
or exposed through a public token getter. `PreparedNormalDefaultProgramRootV1`
already owns the final callable source, so the request transports the token by
that existing move relation.

The transform consumes the disposition together with the initial AST. For
`Ready`, it uses the token's private locators to compare the provider result,
root call, receiver, exact argument cardinality/order/subtrees, result relation,
and terminal against the transformed AST. It then moves the same token into the
final source; it never issues a replacement. The final source's existing exact
transform wrapper proves that this validation ran.

Typed transform rejects are role-specific:

```text
WitnessChanged | ProviderChanged | ProviderResultChanged
RootCallChanged | ReceiverChanged
ArgumentCardinalityChanged | ArgumentOrderChanged | ArgumentChanged
ResultChanged | TerminalChanged | CompositeDropped | CompatibilityLoss
```

A source-backed transform that selects compatibility while the disposition is
`Ready` returns `CompatibilityLoss` before consuming the AST into that lane.
Non-ready parser outcomes keep their existing explicit routing; they cannot be
upgraded to `Ready` later.

`Deferred` is deliberately not a parser-preservation state. No parser issuer
for it exists in this cohort. Located/unlocated `ObservationDeferred` remains a
later resolver/membership state, preventing phase meanings from sharing one
enum.

## Accepted authority chain

```text
SourceEnvelopeReady
+ sealed parser-backed Program source
+ opaque parser-invocation relation
  -> move-bound source authority
  -> consuming fixed-output HRTB source loan
  -> total Program item-role partition
       static provider row
         semantic = StaticCallableCatalogTransfer
         runtime  = RetainedExistingTerminal
       executable Script row
         semantic = exact resolved/transparent/diagnostic/deferred role
         runtime  = existing terminal disposition
  -> executable window -> resolver Complete | typed Deferred
  -> provider subtree -> owned AST-free target/result supplier
  -> call-site composite membership
       CompleteCompositeWithCandidates
         -> candidate subcohort only
         -> private capability
         -> immediate named A consumer
       CompleteCompositeZeroCandidates
         -> named preselected non-direct continuation
       OutsideBoundedCohort
         -> registered pre-capability compatibility owner only
       Deferred / Incomplete / IntegrityInvalid
         -> stop before effects
```

The partition has one source row per Program item and two orthogonal fields:
semantic ownership and runtime ownership. A static Box is not duplicated into
two source rows. Its one row co-seals transfer of the callable subtree and
retention of the wrapper/runtime terminal.

Candidate/noncandidate is decided per observed call site after this partition.
The Program aggregate reports whether the complete composite contains a
candidate subcohort; it never consumes provider or continuation rows as if
they were candidates.

## Bounded first cohort

The first positive is deliberately narrow:

```text
provider:
  one parser-backed, non-sync, non-Main static Box
  exactly one ordinary direct method declaration
  exact parser callable identity and parameter source

Script terminal:
  one final root Return or final Sequence expression
  one syntactic MethodCall with an exact receiver subtree
  exact receiver, ordered arguments, result, and parent/terminal sites
  no parser-issued relation from the receiver/call to the provider

excluded:
  Main, sync/interface/record/generic/instance Box
  import/Using expansion
  multiple provider policy
  nested candidate policy beyond already accepted source trees
```

This scope does not claim that a single provider is the final language limit.
It is the smallest existing-language source intersection that can disprove
empty catalogs, separate declaration fixtures, and source/name re-pairing.

## Existing classifier arm audit

Every selected-normal arm must keep an explicit disposition. Only the
`CatalogedNonMainStaticBox` semantic role is the new bounded design question.

| Existing arm | Semantic role required by the composite partition | Runtime role | First cohort |
| --- | --- | --- | --- |
| `DirectPrint` | existing lexical-core resolution | retained existing terminal | observed, not selected by the first terminal cohort |
| `DirectIfStatement` | existing If-control resolution | retained existing terminal | observed, not widened |
| `DirectFastMemRegion` | existing lexical-core resolution | retained existing terminal | observed, not widened |
| `DirectPortAwareExpression` | preserve exact resolved/transparent/diagnostic/deferred sub-arm | retained existing terminal | final Return/Sequence call only |
| `DirectStaticConstRuntimeCompletion` | transferred program static metadata | retained existing terminal | outside first cohort |
| `DirectEnumDeclarationRuntimeCompletion` | transferred enum declaration | retained existing terminal | outside first cohort |
| `DirectSelectedUnsupportedStatement` | existing selected diagnostic | retained existing terminal | typed outside/stop; no fallback |
| `RawCompatibility` | top-level callable transfer when the exact occurrence exists | no Script runtime terminal | outside first cohort |
| `CatalogedNonMainStaticBox` | **static callable catalog transfer** | **retained existing terminal** | exact provider row |
| `StaticMainCompatibility` | existing compatibility responsibility | retained existing terminal | outside first cohort |
| `SyncBoxRejection` | existing typed rejection responsibility | existing rejecting terminal | outside first cohort |
| `InstancePrefixCompatibility` | existing instance semantic transfer when its cohort row exists | retained existing terminal | outside first cohort |
| `NonPlainInstanceFullLifecycle` | existing record/instance transfer or typed deferred state | retained existing terminal | outside first cohort |

Any transferred or opaque child subtree must be named under its existing owner.
A wildcard, default, or `Option::None` may not turn an unlisted arm into
complete-zero, outside-cohort success, or compatibility.

## Source-loan contract

The accepted HRTB shape remains consuming and fixed-output:

```rust
impl CanonicalScriptASourceAuthorityV1 {
    fn consume_with_composite_source(
        self,
        use_loan: impl for<'src> FnOnce(
            CanonicalScriptCompositeSourceLoanV1<'src>,
        ) -> CanonicalScriptCompositeDraftV1,
    ) -> CanonicalScriptCompositeMembershipOutcomeV1;
}
```

The method, loan, cursor, and draft constructors are private. The callback
does not return arbitrary `R`; the only output is one AST-free draft. `self`
prevents replay through the same owner. HRTB prevents safe borrowed AST data
from escaping, while private fields and structural guards separately forbid
pointer extraction, cloning, side effects, and ordinal re-pairing.

The cursor lends one already-paired item:

```rust
struct CanonicalScriptBorrowedProgramItemV1<'src> {
    statement_site: SourceStmtSiteV1,
    node: &'src ASTNode,
    parser_role: CanonicalParserProgramRoleV1,
}
```

Parser invocation witness is primary identity. Path/digest/profile/read-parse
are integrity evidence. Ordinal validates order inside the one cursor but is
never a join key.

## Lookup and resolver prerequisites

The runtime static-Box terminal is not catalog authority. The source-backed
callable catalog is the existing declaration authority, but its selected
catalog/result products retain AST bodies or borrowed/pointer relations.
`VerifiedScriptDirectStaticCallTargetInventoryV1` additionally stores AST,
window, declaration, and import addresses and already issues target/
noncandidate meaning. None can become the canonical supplier by renaming.

The future owned supplier must be issued while the source loan is active and
move only AST-free rows out:

```text
provider callable identity and exact source site
+ complete static target catalog
+ complete result-contract catalog
+ target/result catalog relation
+ same parser-invocation relation
```

It carries no AST body/reference, address-derived integer, source pointer,
Builder brand, candidate decision, Recipe key, or physical ID.
`TargetOutsideCatalog` and unavailable result behavior stay typed errors unless
a separate reference Decision changes them.

Resolver deferral must retain the exact existing cause shape. Not every cause
has a source site: `SameScopeRedeclaration { name }` is unlocated today.
Therefore the contract is a sum, not `site: Option<_>` and not a fabricated
site:

```text
Deferred.Located { cause, site }
Deferred.UnlocatedSameScopeRedeclaration { name }
```

Missing rows are `Incomplete`; present foreign/duplicate/stale/contradictory
relations are `IntegrityInvalid`. Neither is resolver deferral or zero.

## Owner map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `ParserCompositeSourceIssuerV1` | one parser witness, exact provider declaration/result syntax, nested root-call/terminal tree | target resolution, candidate/A, Recipe, physical IDs |
| parser/source envelope | parser witness, identity/digest/profile/read-parse transport | Program role, target membership, A |
| sealed parser-backed Program source | owned AST and callable/source lineage | Builder route, candidate decision |
| composite Program partition issuer | one total two-axis semantic/runtime item map | target lookup, Recipe, physical IDs |
| callable declaration authority | exact provider method identities/source subtrees | Script call-site pairing, runtime terminal |
| resolver kernel | executable-window forest and exact typed deferral | source admission, capability issuance |
| pre-effect source handoff issuer | parser-witness-bound AST-free forest, continuation, terminal, and complete call coverage that cross package install | AST, second resolver scan, target/C/Recipe/physical meaning |
| owned lookup issuer | AST-free complete target/result relation | candidate/noncandidate and physical selection |
| composite membership issuer | provider + call + argument + terminal correspondence | A facts, Recipe keys, physical IDs |
| private capability issuer | atomic candidate co-seal and immediate move to A | Program classification, public `Ready`, retry |
| A observation issuer | one complete call-site observation consumed once | provider materialization, continuation, fallback |
| retained continuation owner | provider wrapper and enclosing Script terminal | second target resolver or source observation |
| physical/publication consumers | consume future A/C through existing kernels | second argument matcher or Return writer |

## Finite state and edge table

| State | authority / issuer | pre-effect terminal / continuation | old Recipe / fallback |
| --- | --- | --- | --- |
| `ParserComposite.Ready` | parser composite issuer | move through exact transform guard | unavailable |
| `ParserComposite.OutsideBoundedCohort(reason)` | same issuer | existing explicit nonselected route | cannot become Ready |
| `ParserComposite.SourceAuthorityUnavailable(reason)` | same issuer | typed parser/source terminal | unavailable |
| `ParserComposite.Incomplete(error)` | same issuer | hard stop | unavailable |
| `ParserComposite.IntegrityInvalid(error)` | same issuer | hard stop | unavailable |
| `Transform.Preserved` | exact transform verifier | same token into final source/request | unavailable |
| `Transform.CompatibilityLoss/Changed(role)` | exact transform verifier | hard reject before final source | forbidden |
| `Transport.SourceEnvelopeReady` | envelope owner | bind source authority once | temporary reference edge only before cutover |
| `SourceAuthority.Bound` | private facade | consume source once | no replay/discard-to-Recipe API |
| `Partition.CompleteCompositeWithCandidates` | composite issuer | split candidate subcohort and retained continuation atomically | forbidden |
| `Partition.CompleteCompositeZeroCandidates` | same issuer | named non-direct continuation only | forbidden after capability boundary |
| `Partition.OutsideBoundedCohort(reason)` | source-role issuer | registered pre-effect compatibility owner only | allowed only with registry/sunset |
| `Partition.ObservationDeferred(Located)` | resolver | typed stop | forbidden |
| `Partition.ObservationDeferred(Unlocated)` | resolver | typed stop | forbidden |
| `Partition.Incomplete(error)` | coverage validator | typed stop | forbidden |
| `Partition.IntegrityInvalid(error)` | relation verifier | typed stop | forbidden |
| `PreEffectCompleteSourceObservation` | source/resolver handoff issuer | move once across package install | no AST borrow, re-resolve, or fallback |
| `Capability.Ready` | private issuer | immediate named A consumer | cannot escape/store |
| `Capability.Consumed` | named A issuer | one A call-site product | no replay |
| `NoSafeSlice` | design process | remain on this D0 | never a runtime state |

If candidate capability/A issuance fails, the retained continuation has not
started. No provider registration, Builder mutation, Recipe, physical Call, or
publication may have occurred.

## Production boundary

The default production caller family is now exact:

```text
MirCompiler::compile_with_source*
  -> compile_public_program
  -> NormalCompileRequestV1::for_mir_mode         # AST-only today
  -> NormalDefaultPublishedPipelineV1::compile
  -> complete_normal_default_program_root_catalog_lifecycle_with_target
  -> Builder effects / root lowering / publication
```

The canonical source-envelope path is feature-gated vm-reference and is not
default production credit:

```text
vm-reference
  -> SourceEnvelopeReady
  -> discard_before_a_consumer
  -> prepare_script_recipe
```

Deleting only that reference edge is not an I0/CUT0. Conversely, the default
request is AST-only and lacks the opaque parser witness required by the
accepted source authority. The future series must either move a parser-backed
source authority into the selected default request or select and register an
exact pre-effect compatibility residual. It may not synthesize a witness from
AST, path, digest, or name.

The default lifecycle currently begins Builder effects before its Script
resolver/target decision. A production selector must move before that effect
boundary. `PreparedScriptDeferredResidualRegistryV1` has no production read
consumer and is not the policy residual registry.

## Blocker-removal task queue

These rows are a dependency graph, not selected execution cards. No row earns
implementation credit until it either deletes its own named default-production
old edge or belongs to one predeclared 2–5 commit Refactor Series whose terminal
is the A cutover.

### 0. Accepted design cell — `SCRIPT-DIRECT-STATIC-A-COMPOSITE-PROGRAM-MEMBERSHIP-D0`

```text
Change:
  freeze the exact provider + final root call positive
  freeze the per-item semantic/runtime partition and every classifier arm
  freeze provider/call/argument/result/terminal correspondence
  freeze CompleteComposite and all typed failure outcomes

Done:
  one source/parser witness accounts for provider and Script call sites
  provider transfer and retained terminals coexist without residual labels
  candidate is a call-site subcohort, not a whole-Program disposition

Stop:
  any source role needs Builder state, name/ordinal/pointer repair, or a
  second source scan => remain design_stop
```

### 1. Selected fast cell — `SCRIPT-COMPOSITE-SOURCE-PRESERVATION-I0`

Classification: T2 parser source BoxCount and preservation, commit 1 of the
predeclared five-commit cutover series.

```text
Change:
  add callable_parameter_source/composite_source/{model,issuer,transform_guard}
  issue exactly one total parser disposition in Parsed...::new
  move it through Parsed -> Prepared -> transform -> VerifiedFinal -> request
  reject Ready drift or compatibility loss before final source issuance

Contract:
  no generic/sync/instance/import widening
  no target/result/candidate meaning
  token and constructor are private/non-Clone; disposition is never Option
  the same parser witness and nested site tree survive every handoff
  `VerifiedFinalCallableProgramSourceV1` transports but does not issue the
  private preservation token
  existing optional NormalParserCallableSourceHandoff witness is not reused as
  composite authority

Done:
  natural positive reaches NormalCompileRequestV1 with one Ready token
  zero/multi argument and FinalSequence/RootReturn coverage is exact
  provider/result/call/receiver/argument/result/terminal drift rejects by role
  foreign/duplicate/missing rows classify IntegrityInvalid/Incomplete
  Ready-to-compatibility is CompatibilityLoss; no AST fallback runs
  token constructor = 1, token move path = 1, AST-only request cannot mint it
  token AST/Span/MIR-path/raw-pointer fields = 0; public token getter = 0
  every touched production source stays below 800 lines; 760 triggers a split
  focused positive/negative tests and the reusable
  script_direct_static_canonical_parser_source_handoff_guard.sh are green
  callable_parameter_source/README.md, normal_callable_program_source/README.md,
  and the existing check index record the authority/transport boundary

Stop:
  if the issuer needs target/name pairing, Builder state, a second source scan,
  a parallel Option field, token reconstruction, or source-backed fallback,
  return to design_stop
```

This cell is one atomic implementation commit. The external three-commit split
is intentionally folded together: no intermediate commit may leave a parser
token unvalidated or unable to reach the request. Its later semantic consumer
and old-edge retirement are fixed by the same series below.

### Observed implementation evidence — 2026-08-21

The selected fast cell is implementation-complete and is now in closeout. The
observed evidence is:

```text
parser::normal_callable_program_source::tests::composite_source*  9 passed
  - FinalSequence and RootReturn
  - zero, one, and ordered multi-argument calls
  - provider result, call method, receiver, argument value/cardinality,
    and terminal drift rejection
parser::normal_callable_program_source::tests::instance_provider_stays_outside_composite_first_cohort  1 passed
parser::callable_parameter_source::tests::source_session_rejects_foreign_and_duplicate_method_sites  1 passed
mir::compiler::normal_default_pipeline::tests::callable_source_request_carries_parser_composite_ready_token  1 passed
cargo check --quiet  pass
current_state_pointer_guard.sh  pass
routing_classification_completeness_guard.sh  pass
script_direct_static_canonical_parser_source_handoff_guard.sh  pass
git diff --check  pass
```

The parser issuer has one constructor call and the total typed disposition
keeps `OutsideBoundedCohort`, `SourceAuthorityUnavailable`, `Incomplete`, and
`IntegrityInvalid` separate; the existing parser source-session fixture still
proves foreign and duplicate site rejection. No malformed parser product is
fabricated in this cell, and no default/empty/`Option` repair path is added.
The new token remains AST-free/non-`Clone`, and the request keeps it only by
the existing callable-source root move. Full resolver, lookup, A/C, Recipe,
physical, fallback retirement, and production activation remain closed.

### 2. Composite source admission — `SCRIPT-COMPOSITE-SOURCE-ADMIT-I0-R0`

```text
Change:
  consume the preserved parser disposition at the default pre-effect source
  selector; admit the exact provider + Script-terminal Program into one total
  two-axis semantic/runtime partition and source envelope

R0:
  remove the selected Script-row sibling drop and AST-only composite admission
  edge for the bounded cohort

Done:
  Ready source reaches one composite partition issuer under the same witness
  provider transfer and retained static-Box terminal coexist in one item row
  Outside / SourceUnavailable / Incomplete / IntegrityInvalid remain distinct

Non-claims:
  no resolver reownership, target lookup, A/C, Recipe, or physical work
```

R0 execution contract:

```text
source authority:
  VerifiedFinalCallableProgramSourceV1 carrying the parser-owned disposition

canonical issuer:
  CanonicalScriptCompositeProgramPartitionIssuerV1
  one call inside the parser-backed neutral-window loan callback

product:
  one private witness-bound two-axis Program partition;
  each bounded source item has semantic ownership and runtime ownership,
  with the static provider row co-sealing callable transfer plus retained
  existing terminal responsibility

fail-fast:
  SourceAuthorityUnavailable / Incomplete / IntegrityInvalid stop before
  resolver, lookup, candidate, Recipe, Builder effects, or fallback;
  OutsideBoundedCohort remains an explicit pre-capability disposition

transport:
  the partition is consumed by the existing Script admission handoff;
  no parallel request Option, AST/name/ordinal re-pairing, or vm-reference
  production credit is introduced
```

R0 implementation evidence:

```text
src/mir/builder/normal_script_composite_partition.rs
  sole issuer + two-axis provider/terminal rows + focused source/admission tests
src/parser/callable_parameter_source/composite_source/loan.rs
  one HRTB loan; AST references cannot escape into the partition product
src/mir/builder/normal_default_root_catalog_lifecycle.rs
  one pre-effect issue call; hard partition states stop before target install
src/mir/builder/program_root_work_plan.rs
  selected Script window receives the ready partition by borrowed handoff
```

The R0 product does not resolve the receiver or select a target. A root
`MethodCall` therefore remains the existing lexical/resolver item, while the
provider declaration is the only new semantic transfer. The intentionally
failed full lifecycle probe reached a later missing-variable resolver site;
that route is outside this cell's non-claims and is recorded as the next
source/resolver reownership problem rather than widened here.

### Bounded series declaration — `SCRIPT-DIRECT-STATIC-A-CUTOVER-SERIES`

The series is fixed before implementation and has one terminal: default
production A cutover. No new task row may be inserted after it starts.

```text
1. SCRIPT-COMPOSITE-SOURCE-PRESERVATION-I0
   sole parser token + exact transform guard + default-request transport
2. SCRIPT-COMPOSITE-SOURCE-ADMIT-I0-R0
   bounded cohort + source envelope + two-axis Program partition
3. SCRIPT-SOURCE-REOWN-I0-R0
   neutral two-axis window + located/unlocated Deferred, old Builder window out
4. SCRIPT-LOOKUP-REOWN-I0-R0
   owned AST-free target/result relation, pointer lookup out
5. SCRIPT-A-CUTOVER-I0-R0
   pre-effect selector -> private capability -> named A/C/physical chain;
   old Recipe/fallback edges caller-zero
```

If the first cell cannot preserve the source token exactly, the series returns
to design stop and does not add an adapter or compatibility fallback.

### 3. Source reownership audit — `SCRIPT-SOURCE-REOWN-I0-R0`

The worker audit first closed the ambiguity: the old production issuer was a
Builder chain, not a neutral window owner:

```text
PreparedProgramRootWorkPlanV1
  -> ScriptRootSemanticDecisionV1
  -> ScriptRootDemandWindowBuilderV1
  -> VerifiedScriptRootDemandWindowV1
```

The same window was consumed by resolver traversal, semantic
source/continuation, target inventory, and Recipe. A wrapper or rename would
have left the competing Builder authority alive. The R0 parser composite
partition was only a bounded staging product and could not replace the full
Program window. Phase2 now removes that production edge and installs the
neutral issuer documented in 3b/3c.

The source reownership cell is therefore ordered internally, while remaining
one bounded prerequisite cell in the declared cutover series:

#### 3a. Typed resolver deferral — `SCRIPT-SOURCE-REOWN-DEFERRED-I0-R0`

```text
Source authority:
  existing ShadowResolveErrorV0, including its owned source site when present

Issuer:
  FunctionSemanticResolverSessionV1 at the existing Script forest boundary

Product:
  ResolveScriptForestOutcomeV1::Deferred(ScriptResolverDeferredV1)
  ResolveScriptOutcomeV1::Deferred(ScriptResolverDeferredV1)

Mapping:
  UnresolvedName / unsupported statement/expression/assignment /
  arity overflow / non-local exit -> Located { cause, site }
  SameScopeRedeclaration -> Unlocated { cause, name }

Done:
  no unit Deferred producer or consumer; no fabricated site and no
  site: Option<_> unknown-state merge; the existing Deferred runtime owner
  receives the typed observation without target/Recipe meaning

Non-claims:
  no window move, lookup, A/C, Recipe, physical, fallback retirement, or
  production selector change
```

This is the only fast/BoxShape substep. Its focused gate must cover every
located cause, the unlocated redeclaration, and the existing Complete/error
split. It may not hide the typed value behind `None` or convert it to raw
compatibility. The current runtime behavior remains unchanged until the
neutral window cell explicitly replaces that owner.

Implementation receipt: `98f36e88b9` adds the exhaustive source-error mapping,
typed forest/function outcomes, a small normal-lifecycle adapter, and the
existing raw-port transport. `NormalScriptRootLoweringMode::Unavailable`
keeps App/compatibility routes with no Script window distinct from resolver
deferral. The focused
`script_deferral_preserves_located_and_unlocated_states` test passes, the
normal Script semantic-source filter reports 17 existing passes, and
`cargo check --profile quick` passes. Its other 50 failures are baseline: a
representative `real_print_fixture_uses_the_selected_normal_request` failure
reproduces unchanged on parent `36a1908966` with
`[freeze:contract][mir/instance-constructor-source/cohort-missing]`. No target,
Recipe, physical, fallback, or production route was changed.

#### 3b. Neutral source window — `SCRIPT-SOURCE-REOWN-WINDOW-I0-R0`

The design stop is closed by the worker census and the following type-level
Decision. The sole issuer is a source/resolver-boundary owner, not Builder:

```text
ParserNormalProgramSourceAuthorityV1
  -> PreparedCanonicalScriptNeutralProgramWindowV1::issue
  -> PreparedScriptRootAdmissionV1
  -> fixed resolver demand view / existing consumers
```

The existing `VerifiedScriptRootDemandWindowV1` remains the consumer contract;
its `seal` is called by the neutral issuer exactly once in production. The
Builder `ScriptRootDemandWindowBuilderV1`, `ScriptRootSemanticDecisionV1`, and
`SelectedScriptProgramOccurrenceV1` are removed from the production edge. A
test-only compatibility helper may remain temporarily while its tests move to
the neutral issuer, but it cannot be called by the default lifecycle.

The parser product gets one atomic, non-`Clone`, AST-free authority. It is
issued inside `ParsedProgramWithCallableParameterSourceV1::new` and carries the
same invocation witness, generic top-level ProgramBody coverage rows, and the
already-issued composite disposition:

```rust
struct ParserNormalProgramSourceAuthorityV1 {
    invocation: ParserInvocationWitnessV1,
    body_rows: Box<[ParserNormalProgramBodySourceRowV1]>,
    composite: ParserCompositeSourceDispositionV1,
    _seal: ParserNormalProgramSourceAuthoritySealV1,
}
```

The source-backed move chain is mandatory and has no parallel `Option` field:

```text
ParsedProgramWithCallableParameterSourceV1
  -> PreparedNormalCallableProgramSourceV1
  -> VerifiedFinalCallableProgramSourceV1
  -> VerifiedResolvedCallableSemanticBatchV1
  -> NormalCompileRequestV1 / PreparedNormalDefaultProgramRootV1
```

`CanonicalScriptSourceRowsV1` remains the older pure-Script/front-door
projection. It is not promoted into the mixed callable authority and cannot be
used as a default empty catalog. The new authority is issued for the
source-backed Program body independently of canonical pure-Script admission,
so the static-provider + root-call shape is not rejected by a parallel cohort
classifier before it reaches the selected normal caller.

The authority exposes one higher-ranked loan. Its cursor yields one paired
parser row and AST node; no caller receives separate arrays to zip, and no AST
reference can escape:

```rust
with_normal_program_source_loan(
    &self,
    use_loan: impl for<'src> FnOnce(ParserNormalProgramSourceLoanV1<'src>) -> R,
) -> Result<R, ParserNormalProgramSourceLoanRejectV1>;
```

The neutral issuer consumes this loan once and co-seals the composite
partition, instance-Box transfer coverage, and the existing Script window.
The instance transfer product is witness-bound to the same invocation; an
ordinal-only adapter is forbidden. The resulting window is total over the
real Program body and contains only AST-free `ProgramBody` site rows with two
axes:

```text
structural source role   +   retained runtime-terminal role
```

The issuer may call the existing runtime classifier as a shape predicate, but
it alone cannot issue window membership. The neutral mapping owns the finite
semantic/runtime projection and never imports target inventory, Recipe keys,
MIR IDs, pointers, names, or AST references into the window.

The default lifecycle order is fixed:

```text
source-backed final source
  -> semantic package owns the move-only parser authority
  -> one neutral source loan
  -> window + composite partition + instance transfer co-seal
  -> typed failure, if any, before target install/effects
  -> work plan transports the already-prepared admission
  -> existing resolver/lookup/Recipe consumers continue unchanged
```

`PreparedProgramRootWorkPlanV1` receives `PreparedScriptRootAdmissionV1`; it
does not construct or seal a window. `normal_default_root_catalog_lifecycle`
does not call `package.source_ast()` to rebuild source membership. The old
`composite_partition` root field and its separate source observation are
removed once the neutral issuer owns the combined call.

Required cutover evidence:

```text
ParserNormalProgramSourceAuthority issuer    = 1
neutral window issuer production caller      = 1
VerifiedScriptRootDemandWindowV1::seal      = 1
ScriptRootDemandWindowBuilder production call = 0
ScriptRootSemanticDecision production call   = 0
SelectedScriptProgramOccurrence window calls = 0
all Program site coverage and real empty      = explicit
window/source failure                         < target install and effects
AST/pointer/name/Recipe/MIR authority         = 0
```

### Ordered implementation tasks

`SCRIPT-SOURCE-AUTHORITY-HANDOFF-I0`:

1. Add the parser-only non-`Clone` body authority and paired HRTB cursor under
   `src/parser/callable_parameter_source/`. Keep composite state nested under
   the same invocation seal; do not reuse canonical pure-Script rows as a
   complete mixed-source authority.
2. Move the authority through Parsed → Prepared → Final source. Transform
   validation must reject body count/shape or exact source-tree drift before a
   compatibility AST is returned. No reconstruction from name, Span, ordinal,
   digest, pointer, or transformed AST is allowed.
3. Make the semantic batch/package transport the authority without cloning or
   splitting the source AST. Add only a scoped loan method; do not make the
   semantic package a second issuer.

`SCRIPT-SOURCE-REOWN-WINDOW-I0-R0`:

1. Add `PreparedCanonicalScriptNeutralProgramWindowV1::issue` under the
   Builder boundary, with the issuer itself consuming the parser authority
   loan. Co-issue the existing R0 composite partition, witness-bound
   instance transfer, and constructor-source cohort in this callback.
2. Replace the default lifecycle's Builder window construction with the
   prepared admission returned by the neutral issuer. Pass that admission into
   the work plan as transport. Keep resolver, continuation, target inventory,
   bundle, and Recipe signatures unchanged in this row.
3. Delete the selected production edges to
   `ScriptRootDemandWindowBuilderV1`, `ScriptRootSemanticDecisionV1`, and
   `SelectedScriptProgramOccurrenceV1`; move their positive/negative tests to
   the neutral issuer or delete obsolete test-only authority.

The parser authority handoff is already landed in the preceding commit
`aa1aecf495`. This window row is the single follow-up implementation slice:

```text
SCRIPT-SOURCE-REOWN-WINDOW-I0-R0
  one neutral issuer + work-plan transport + old production edges deleted
```

No later commit may land an unused parser authority or a new downstream
semantic receipt without its named neutral consumer in the same series.

Positive evidence:

```text
real empty Program -> explicit CompleteEmpty/zero-row window
one ordinary statement -> one paired row and existing semantic/runtime axes
static provider + final Sequence/RootReturn -> provider transfer + retained terminal
instance Box -> witness-bound transfer, no ordinal-only adapter
multiple arguments -> composite order preserved by one parser loan
unchanged transform -> authority moves to final source/request
```

Negative evidence:

```text
foreign parser witness -> IntegrityInvalid
missing/duplicate/gap body row -> Incomplete or IntegrityInvalid
provider/call/terminal drift -> typed transform rejection
Ready source -> Compatibility -> CompatibilityLoss
window failure -> zero target-install/Recipe/physical effects
Builder window/semantic caller on default edge -> guard failure
```

The window task returns to design stop if the parser-backed authority cannot
reach the default caller without `package.source_ast()` reconstruction, if
instance transfer coverage needs an unowned adapter, or if the old semantic
mapping requires a second source observation. It does not open lookup, A/C,
Recipe retirement, fallback retirement, production cutover, or performance.

#### 3c. Window implementation receipt — `SCRIPT-SOURCE-REOWN-WINDOW-I0-R0`

The phase2 implementation closes the neutral source boundary described above.
The production graph is now:

```text
VerifiedNormalCallableSemanticPackageV1
  -> PreparedCanonicalScriptNeutralProgramWindowV1::issue
       -> one ParserNormalProgramSourceLoanV1
       -> composite partition
       -> instance-Box transfer cohort
       -> constructor source cohort
       -> one VerifiedScriptRootDemandWindowV1::seal
  -> PreparedScriptRootAdmissionV1
  -> PreparedProgramRootWorkPlanV1 transport
```

The neutral aggregate stores only source admission, parser-witness-bound
source rows, and the existing source cohorts. It does not store AST, pointer,
target inventory, Recipe/Join, MIR identity, or physical meaning. The
instance-transfer product no longer stores a `BTreeSet<usize>` source
authority, and constructor coverage is issued from the same parser loan rather
than a lifecycle `package.source_ast()` scan.

The old Builder window/decision/occurrence modules remain only for legacy unit
fixtures. Their production callers are zero. Neutral source failure is before
target installation and Builder effects; the work plan receives the prepared
admission and does not rebuild the window.

Observed phase2 evidence:

```text
CARGO_BUILD_JOBS=4 cargo check --profile quick
  pass
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib neutral_issuer
  2 passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_script_instance_box_transfer
  4 passed
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_script_composite_partition
  4 passed
tools/checks/script_direct_static_source_reown_window_r0_guard.sh
  pass
tools/checks/script_direct_static_composite_source_admission_r0_guard.sh
  pass
```

This closes source-window reownership. Lookup reownership was the next
design/authority cell and is recorded as implementation-complete below;
target/result pointer retirement, A/C, Recipe retirement, fallback retirement,
and production cutover remain separately bounded.

### 4. Contingent lookup reownership — `SCRIPT-LOOKUP-REOWN-I0-R0`

Decision: ACCEPT the lookup reownership, with one provenance/co-seal
correction. The old pointer inventory is not renamed or adapted. A single
source-package lookup issuer observes the root call tree while the parser
loan is live, snapshots only AST-free rows, and moves the result relation to
the existing Script result bundle.

Source authority + canonical issuer:

```text
VerifiedNormalCallableSemanticPackageV1
  + PreparedCanonicalScriptNeutralProgramWindowV1
  + package-owned declaration catalog
  + transient generic target/result catalogs
    -> ScriptDirectStaticCallLookupIssuerV1::issue
       (one HRTB loan callback)
    -> VerifiedScriptDirectStaticCallLookupV1
```

The issuer is the only production constructor. Its output contains the
parser invocation witness, exact `SourceExprSiteV1` coverage, canonical target
keys, result representation, and required argument ordinals. It contains no
AST reference, pointer/address, catalog borrow, Builder product, candidate
decision, Recipe key, or MIR/physical ID. Generic declaration/import/result
brands may validate the co-seal inside the issuing scope; none may escape in
the output. The package and loan are accepted as one source authority so a
foreign catalog cannot be paired by name, ordinal, digest, or pointer.

Non-authority:

```text
VerifiedScriptDirectStaticCallTargetInventoryV1
PreparedScriptRootAdmissionV1::script_direct_static_targets
AST/name/ordinal/digest/pointer pairing
caller-built empty/default catalogs
Builder window or semantic source adapter
ResultBundle/Recipe/Join/physical products
```

Fail-fast boundary:

```text
semantic package
  -> neutral source window
  -> generic catalog co-seal + owned Script lookup relation
  -> owned static-result publication owner
  -> reject before target-capability install, Builder effects, or fallback
```

`TargetOutsideCatalog`, missing result representation, source-loan failure,
foreign witness, duplicate site, and contradictory coverage are typed lookup
rejections. The old `attach/take` edge is removed from
`PreparedScriptRootAdmissionV1`; the relation remains a lifecycle-local
one-shot value and is consumed exactly once by the existing result bundle.

Smallest next slice:

```text
SCRIPT-LOOKUP-REOWN-I0-R0
  1. add the owned AST-free relation and one source-package issuer;
  2. preflight the generic catalogs and static publication owner before
     target-capability/Builder effects;
  3. remove admission attach/take and the production pointer inventory edge;
  4. adapt ResultBundle to consume the relation without changing Recipe/Join.
```

Acceptance:

```text
production lookup issuer = 1
old inventory issue/attach/take/brand edge = 0
lookup output AST/reference/address/pointer fields = 0
foreign/missing/duplicate/target-outside/result-unavailable = typed reject
lookup failure -> target capability, Builder effect, Recipe, fallback = 0
existing ResultBundle -> Recipe -> Join behavior remains the consumer
```

Non-claims:

```text
A/C capability, candidate/noncandidate disposition, physical changes,
Recipe retirement, fallback retirement, production switch, source cohort
expansion, generic catalog redesign, ABI/backend/performance
```

This is a T2 new authority even when runtime behavior is preserved. It is not
a rename of the existing pointer product. The temporary generic catalogs are
inputs only; the owned relation is the sole selected Script lookup product.

### 4a. Lookup implementation receipt — `873eacad33`

`SCRIPT-LOOKUP-REOWN-I0-R0` is implementation-complete and pushed. The
source-package facade now issues one non-`Clone` lookup relation from the same
semantic package and neutral-window parser provenance. It co-seals the
package-owned declaration catalog, transient target/result catalogs, exact
MethodCall source sites, canonical target keys, result representation, and
required argument ordinals. The relation is moved by value exactly once into
the existing ResultBundle; no AST, catalog borrow, pointer/address, Builder
product, candidate meaning, Recipe key, or physical ID leaves the issuer.

The production order is now observable:

```text
semantic package
  -> neutral source window
  -> ScriptDirectStaticCallLookupIssuerV1::issue
  -> owned static-result publication owner
  -> pinned target capability / Builder effects
```

Lookup/static-publication failure stops at the pre-effect boundary. The old
pointer inventory issue/attach/take/brand edge is gone from production, and
the old inventory remains only inside focused test adapters. ResultBundle,
Recipe, Join, claim, and physical consumers were adapted only at their input
boundary; their later authority was not reopened.

Observed evidence:

```text
cargo check --profile quick
  pass; existing repository warnings only
cargo test --profile quick --lib mir::builder::normal_script_direct_static_lookup
  3 passed
cargo test --profile quick --lib mir::builder::normal_script_direct_static_result_bundle
  2 passed
cargo test --profile quick --lib mir::source_call_target::script_direct_static_tests
  8 passed
tools/checks/script_direct_static_target_guard.sh
  OK
tools/checks/script_direct_static_source_reown_window_r0_guard.sh
  PASS
tools/checks/script_direct_static_composite_source_admission_r0_guard.sh
  PASS
tools/checks/current_state_pointer_guard.sh
  ok
git diff --check
  pass
```

The next boundary is `SCRIPT-A-CUTOVER-I0-R0` design stop. A/C capability,
candidate/noncandidate disposition, named consumers, Recipe retirement,
fallback retirement, production switch, and performance remain unopened.

### 5. Series terminal — `SCRIPT-A-CUTOVER-I0-R0`

Decision: **Conditional Accept; keep `design_stop` until the following A/C
contract is fixed in code-independent SSOT.** The lookup reownership is a
valid input boundary, but it is not itself A. The existing Bundle/Recipe/Join
chain is a valid later direct-static consumer, but it is not itself C.

The two read-only audits reached the same result:

```text
lookup = owned target/result facts only
ClaimLedger = operational candidate consumer, not C disposition authority
empty Bundle/Join = not a zero-candidate witness
generic activation plan = legacy generic owner; not the canonical Script A
```

Worker premise audit is required before Fast path: the resolver product still
has no parser-witness bridge, the owned lookup still lacks complete
zero-candidate coverage, and the live lifecycle places resolver/consumer work
after Builder preparation.

The ownership audit adds one lifetime constraint: the current
`VerifiedScriptSemanticSourceV1<'source>` borrows the Program AST and cannot
survive the move of the callable source into the installed semantic package.
Therefore the pre-effect product must own an AST-free forest, continuation,
parser provenance, and complete call coverage first; the later borrowed Script
wrapper may consume that product after installation, but may not re-resolve or
re-scan the AST.

Observed callpoints: `normal_script_resolution.rs` currently creates the
borrowed Script wrapper from AST/window/facts only, while
`normal_default_root_catalog_lifecycle.rs` moves the callable source into the
semantic package before `prepare_install` and runs resolver/Bundle/Recipe
after `prepare_normal_default_module`. This is an ownership boundary, not a
mere call-order rearrangement.

#### A/C authority contract

```text
source package + neutral window
  + owned lookup relation
  + complete resolver source / retained continuation
  + complete terminal and call coverage
    -> CanonicalScriptASourceCapabilityIssuerV1::issue_into_a
       (private, one-shot, non-Clone capability)
    -> CanonicalScriptAObservationIssuerV1::consume
       (the only A consumer)
    -> CanonicalScriptCDispositionIssuerV1::issue_into_named_consumer
       (the only C issuer)
```

The private capability co-seals only AST-free source/Facts inputs:

```text
ParserInvocationWitnessV1
source-window completeness and terminal coverage
resolver-complete Script source rows and retained continuation
owned Script lookup target/result rows
complete call inventory, including explicit zero/noncandidate coverage
owned pre-effect forest/continuation handoff that can cross package install
```

It must not contain AST references, pointer/address identities, `ValueId`,
`MirType`, `BasicBlockId`, Recipe keys, Join keys, physical IDs, or a public
`Ready` return. A cannot reconstruct missing rows from names, ordinals, digest,
or the Builder work plan.

The existing generic `VerifiedCallableResultActivationPlanV1` remains
non-authority for this cell: it re-observes generic declaration bodies and
uses pointer-branded borrowed catalogs, while the canonical Script source
relation already has a parser witness and one source-package issuer.

#### Finite A/C states and named consumers

| Phase | Sole issuer | State | Named next consumer | Forbidden edge |
| --- | --- | --- | --- | --- |
| source | lookup/resolver co-seal | `SourceAuthorityUnavailable`, `ObservationDeferred`, `Incomplete`, `IntegrityInvalid` | typed outer rejection | Builder, Recipe, retry, fallback |
| A | `CanonicalScriptASourceCapabilityIssuerV1` | private `Ready` | `CanonicalScriptAObservationIssuerV1` only | store, discard-to-Recipe, compatibility |
| A | `CanonicalScriptAObservationIssuerV1` | `CompleteWithCandidates` or `CompleteZeroCandidates` | one C issuer | second source scan |
| C | `CanonicalScriptCDispositionIssuerV1` | `DispositionReady` | `CanonicalScriptDirectStaticConsumerV1` | generic reclassification |
| C | `CanonicalScriptCDispositionIssuerV1` | `NonCandidate(complete witness)` | `CanonicalScriptNonDirectContinuationConsumerV1` | ClaimLedger `Absent` as proof |
| terminal | named consumers | consumed exactly once | existing direct-static Facts/Recipe/Join/physical chain or retained non-direct continuation | old Recipe/fallback |

`CompleteZeroCandidates` is issued only from complete call coverage. It must
distinguish at least: no MethodCall in the bounded terminal, observed call
with bound/dynamic/reserved receiver, and observed call whose target/result
contract is not eligible. A missing call row, empty Bundle, or claim ingress
`Absent` is `Incomplete`/a later operational outcome, never zero proof.

#### Required pre-effect boundary

The target capability and Builder mutation boundary must become:

```text
semantic package
  -> neutral window
  -> owned lookup + complete call coverage
  -> resolver Complete / typed Deferred
  -> AST-free PreEffectCompleteSourceObservation co-seal
  -> private A issue and immediate consume
  -> C disposition and named consumer bind
  -> install_pinned_text_target_capability
  -> prepare_normal_default_module / Builder effects
```

Today the resolver and Bundle/Recipe/Join construction occur after Builder
preparation. That is the concrete remaining lifecycle blocker. The next task
must move only the source/A/C decision to this boundary; it must not redesign
Recipe, Join, physical lowering, or publication.

#### Bounded implementation task sequence after design closure

These are five disjoint implementation cards, not permission to start while
this section is open:

1. `SCRIPT-A-SOURCE-COVERAGE-I0` — extend the one source-package HRTB issuer
   to retain every bounded Script MethodCall carrier, with explicit
   `CompleteEmpty`/`CompleteRows`, source route/noncandidate evidence, and
   typed missing/foreign/duplicate coverage. Keep target selection and C
   meaning out of this product.
2. `SCRIPT-A-PREFLIGHT-SOURCE-HANDOFF-I0` — pass the same parser witness into
   the resolver source owner and issue an owned AST-free
   `PreEffectCompleteSourceObservation` containing forest, continuation,
   terminal coverage, and the source coverage product. Move this handoff
   before pinned-target/Builder effects; after package installation, construct
   the existing borrowed Script wrapper by consuming the handoff, with no
   second resolver or AST scan.
3. `SCRIPT-A-CAPABILITY-I0` — define the private non-Clone A capability from
   that handoff plus owned target/result facts, issue it once, and consume it
   immediately in the named A issuer. Add positive, explicit-zero, foreign,
   missing, duplicate, and pre-effect failure evidence. Do not touch
   Recipe/Join/physical semantics.
4. `SCRIPT-C-DISPOSITION-CONSUMER-I0` — define the closed C disposition
   (`DispositionReady` versus explicit `NonCandidate`), bind the two named
   consumers, and adapt the existing direct-static Facts/Recipe/Join chain
   only at its input boundary. No generic activation-plan reuse, fallback, or
   semantic reclassification downstream.
5. `SCRIPT-A-CUTOVER-I0` — move the A/C facade before pinned-target/Builder
   effects, remove the post-effect candidate decision for this migrated
   surface, and add structural guards proving issuer counts, consumer counts,
   zero-witness preservation, and zero old/fallback edges. Keep old Recipe
   retirement and production-wide switch as later explicit rows unless the
   same series' retirement evidence is complete.

#### Selected execution brief — `SCRIPT-A-SOURCE-COVERAGE-I0`

Change: extend the existing source-package HRTB observation so one parser
invocation emits a complete AST-free MethodCall coverage product alongside
the already-owned selected target/result rows. The observation loop remains
single-pass; it must not be recreated from Builder products or run a second
AST scan.

Contract: coverage is parser-witnessed and has explicit
`CompleteEmpty`/`CompleteRows` states. Each bounded MethodCall row retains its
site, receiver site, ordered argument sites, result site, and source-route
disposition. Ordinary qualified-unbound rows are marked source-eligible;
bound/current-owner/dynamic/type-operation/reserved rows retain typed
non-direct evidence. Target selection, result candidate meaning, C, Recipe,
Join, and physical identity stay outside this product.

Done: the production lookup transports the coverage product; focused evidence
covers one ordinary row, a true empty Script, explicit non-direct routes,
foreign invocation, and target-outside-catalog rejection. The production error
vocabulary and structural guard retain missing, receiver-mismatch, and
duplicate coverage rejection paths; existing test adapters remain test-only;
the reusable target guard and source-size limit stay green.

Stop: if complete coverage requires a second AST observer, if missing rows
collapse into `CompleteEmpty`, or if target/C/Recipe meaning leaks into the
coverage product, return to design stop and record the missing authority.

Worker audit requested for this fast cell: one read-only top-down review of
the changed coverage/lookup boundary is allowed before closeout. The question
is limited to source authority, complete-zero/foreign/duplicate preservation,
and target/C/Recipe/physical responsibility leakage; the worker must not edit
files or authorize the next pre-effect A/C slice.

#### Closeout evidence — `SCRIPT-A-SOURCE-COVERAGE-I0`

The production lookup now keeps the single parser-package HRTB observation as
`VerifiedScriptCallCoverageV1`: a real zero-call Script is
`CompleteEmpty`; every observed MethodCall is retained in `CompleteRows` with
parser provenance, receiver/ordered-argument/result sites, and typed source
route evidence. Ordinary qualified-unbound rows continue to the existing
target/result lookup; bound/current-owner/dynamic/type-operation/reserved rows
remain visible as `NonDirect` coverage and do not become A/C meaning.

Focused evidence is green for one ordinary selected row, true empty coverage,
bound/dynamic routes, type-operation/reserved routes, foreign invocation, and
target-outside-catalog rejection. The production typed error paths retain
missing projected sites, receiver-site mismatch, and duplicate coverage
rejection. The production edge still has one HRTB observation and no second
AST scan; the coverage product has no AST, pointer, target, candidate/C,
Recipe/Join, MIR, physical, or fallback identity. The reusable Script guard,
current-state pointer guard, source-size check, and diff check are green. The
worker returned no report before shutdown; the same read-only boundary audit
was completed by the main agent and did not widen the cell.

Closeout boundary: this cell is complete, but the parser-witnessed coverage is
not yet the pre-effect A capability. Under the parent
`SCRIPT-A-CUTOVER-I0-R0` design stop, `SCRIPT-A-PREFLIGHT-SOURCE-HANDOFF-I0`
is the first design/implementation decision and must define how the resolver
complete/deferred outcome and retained terminal cross package install.

No implementation card may introduce a parallel `Option` A receipt or expose
capability `Ready` to dispatch. If complete call coverage cannot be issued
without a second AST observer, stop and add that missing source-authority
cell instead of adapting Builder semantic products.

```text
one pre-effect selector
  CompleteCompositeWithCandidates
    -> private capability
    -> immediate named A consumer
    -> existing direct-static physical/publication consumer
    + retained continuation consumed once
  CompleteCompositeZeroCandidates
    -> named non-direct continuation
  OutsideBoundedCohort
    -> registered compatibility owner
  Deferred / Incomplete / IntegrityInvalid
    -> stop before effects

same-series delete set:
  old default source-window authority
  unit Deferred edge
  pointer-branded selected lookup authority
  post-effect candidate selection for the migrated surface

guards:
  default production caller >= 1
  selected old candidate edges = 0
  capability issuer = 1; named A consumer = 1; orphan Ready = 0
  error/candidate -> Recipe/raw/retry/fallback = 0
```

Cells 1–5 form one
predeclared 2–5 commit series whose terminal closes this cutover. If that bound
is not credible, remain parked. A forward source/lookup substrate may not land
and wait for a later CUT0.

### 6. Reference/residual retirement — `SCRIPT-RECIPE-RETIRE-I0-R0`

After every registered residual has a source-backed Facts/Recipe completion
owner, retire the canonical reference Recipe edge. This cleanup is not default
production credit.

```text
SealedNormalScriptSourceV1::prepare_script_recipe callers = 0
normal_source_plan::script_recipe::prepare callers         = 0
OpenScriptPhysicalEntryV1::open canonical callers          = 0
dispatch ScriptRecipe stage/error/restore edges            = 0
fallback / retry                                            = 0
```

The Builder-specific `VerifiedScriptDirectStaticRecipeV1` is a different
product and is not silently included in this retirement row.

## Acceptance for the selected fast cell

R0 closes only when the following are observable:

1. the parser-preservation I0 token reaches the default root by move and one
   scoped HRTB loan;
2. `CanonicalScriptCompositeProgramPartitionIssuerV1` has exactly one
   production caller and issues no AST-bearing product;
3. the exact one-provider/final-root-call cohort emits one provider-transfer
   row plus one retained root-terminal row under the same parser witness;
4. `Outside`, `SourceAuthorityUnavailable`, `Incomplete`, and
   `IntegrityInvalid` stay distinct, with the latter three stopping before
   effects and fallback;
5. the selected Script demand window consumes the provider transfer without
   re-resolving or re-pairing the root call;
6. focused positive/negative/structural evidence, the reusable R0 guard, and
   the source-size limits are green; and
7. the next source/resolver reownership cell is named without opening lookup,
   A/C, Recipe, physical, or production cutover work.

The parser-preservation I0 is already pushed and closed. R0 green does not
authorize resolver, lookup, A/C, physical work, old Recipe retirement, or a
production claim.

Observed R0 evidence:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_script_composite_partition
  4 passed
RUSTFLAGS='-Awarnings' CARGO_BUILD_JOBS=4 cargo check --quiet
  pass
tools/checks/script_direct_static_composite_source_admission_r0_guard.sh
  pass
tools/checks/current_state_pointer_guard.sh
  pass
git diff --check
  pass
```

The compatibility-only `verified_expansion_disposition_reaches_script_and_app_root_lowering`
fixture still stops at the pre-existing constructor-source cohort contract;
it does not exercise the parser-backed R0 source and remains known baseline
debt. A full composite lifecycle fixture is deliberately not part of R0,
because its later resolver/lookup failure would widen the cell.

## Later-cell stop lines

The selected R0 cell returns to `design_stop` if its own stop condition is
observed. The following conditions prevent advancing into their corresponding
later series cell; they do not authorize widening R0:

- whole Program `Candidate | Residual` is used as the source partition;
- provider transfer or retained terminal is called a residual;
- canonical source admission still rejects the only natural positive after
  the preservation cell reaches the request;
- the source-backed default request fails to transport the accepted token;
- static Box remains an untyped catch-all resolver deferral;
- parser rows or caller-provided windows are treated as semantic membership;
- target/result inputs remain borrowed, AST-bearing, pointer-branded, or
  caller-built empty/default catalogs;
- resolver causes are collapsed to unit or a source site is invented;
- actual empty and missing coverage share one representation;
- capability classifies the Program, escapes `Ready`, can be stored, or has
  more than one consumer;
- selection happens after Builder effects;
- `PreparedScriptDeferredResidualRegistryV1` is treated as the policy registry;
- vm-reference reachability is counted as default production;
- a source/lookup substrate lands without deleting a named old edge or without
  a predeclared bounded series ending in cutover;
- candidate, error, or A failure can retry old Recipe, raw, compatibility,
  ordinary static lowering, or a second resolver.

Only the selected R0 owner, its focused fixtures/tests, and its reusable guard
were authorized in that historical cell. The lookup receipt above closes the
lookup reownership row; the next authorized work is the A/C design stop. No
A/C implementation, fallback, production switch, or downstream semantic
`Verified*`/`Prepared*` receipt is authorized yet.

## Review receipt

The external proposal's candidate A is accepted for opaque parser identity,
non-`Clone` move ownership, exact transform validation, and request transport.
Two corrections keep it compatible with repository policy: parser preservation
does not borrow the later resolver's `Deferred` state, and the proposed three
partial commits are one atomic cell inside the existing five-commit cutover
series so no orphan token lands.

The first audit identified the exact composite atom, the missing static-Box
transfer state, the retained runtime terminal, and the selected resolver
deferral that prevents a natural positive. The second fixed the production
graph: only `compile_with_source* -> NormalDefaultPublishedPipelineV1` earns
default credit; vm-reference does not. It also showed that source observation
and lookup cannot be standalone I0 rows today, that not every deferred cause
has a site, and that any forward prerequisites must belong to a bounded series
ending in the A cutover.

The row `SCRIPT-SOURCE-REOWN-WINDOW-I0-R0` is now implementation-complete and
pushed as `c97b40dc3d`. Together with the preceding parser handoff
`aa1aecf495`, the authority is required through Parsed -> Prepared -> Final ->
semantic package, one HRTB paired cursor feeds the neutral issuer, and the
default Builder window/decision/occurrence callers are zero. The lookup
reownership is now implementation-complete and pushed as `873eacad33`; its
production pointer inventory caller/attach/take edge is zero and its owned
relation is consumed once by ResultBundle. The next frontier is the
design-stop row `SCRIPT-A-CUTOVER-I0-R0`. No A/C implementation, Recipe,
physical, fallback, or production cutover cell is opened by this receipt.

## References

- `docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md`
- `docs/development/current/main/investigations/script-direct-static-a-consumer-bind-d0-2026-08-21.md`
- `docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md`
- `src/parser/callable_parameter_source/canonical_script_source_admission.rs`
- `src/parser/callable_parameter_source/composite_source/model.rs`
- `src/parser/callable_parameter_source/composite_source/issuer.rs`
- `src/parser/callable_parameter_source/composite_source/transform_guard.rs`
- `src/parser/normal_callable_program_source/semantic_syntax_loan.rs`
- `src/parser/normal_callable_program_source/README.md`
- `src/parser/callable_parameter_source/composite_source/loan.rs`
- `src/mir/compiler/normal_default_pipeline_tests.rs`
- `src/mir/builder/normal_script_composite_partition.rs`
- `src/mir/builder/README.md`
- `src/mir/builder/normal_script_direct_static_lookup.rs`
- `src/mir/builder/normal_script_direct_static_lookup_tests.rs`
- `tools/checks/script_direct_static_target_guard.sh`
- `tools/checks/script_direct_static_composite_source_admission_r0_guard.sh`
- `tools/checks/script_direct_static_canonical_parser_source_handoff_guard.sh`
- `src/mir/compiler/normal_source_plan/inventory.rs`
- `src/mir/compiler/normal_source_plan/classifier.rs`
- `src/mir/compiler/canonical_core_dispatch.rs`
- `src/mir/compiler/normal_default_pipeline.rs`
- `src/mir/builder/normal_default_root_catalog_lifecycle.rs`
- `src/mir/builder/normal_script_program_item_admission.rs`
- `src/mir/builder/normal_script_selected_occurrence.rs`
- `src/mir/builder/normal_script_root_admission_witness.rs`
- `src/mir/builder/normal_script_runtime_work.rs`
- `src/mir/resolved_semantics/owner_resolver.rs`
- `src/mir/resolved_semantics/shadow/product.rs`
- `src/mir/source_call_target/script_direct_static.rs`
- `src/mir/callable_result_representation/solver.rs`
