---
Status: Accepted parser-preservation Decision — first fast cell selected inside the bounded cutover series
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0
Parent: docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md
ProductionCaller: default caller is named as evidence, but no parser-backed canonical source authority reaches it; vm-reference is reference-only
ReplacementCell: one parser invocation -> preserved parser composite source -> total two-axis Program partition -> call-site composite membership -> private one-shot capability -> named A consumer
Classification: T2 BoxCount first cell inside the predeclared five-commit series; A/C product and production switch remain closed
NextCard: implement SCRIPT-COMPOSITE-SOURCE-PRESERVATION-I0 on this card
---

# SCRIPT-DIRECT-STATIC-A-SOURCE-CAPABILITY-D0

## Six-line brief

Decision: Accept candidate A: one parser-owned opaque, non-`Clone`
`ParserCompositeSourcePreservationV1`. It owns source preservation only. The
consuming HRTB loan and private capability-to-A handoff remain downstream;
the previous whole-Program `Candidate | Residual` model stays rejected.

Source authority + canonical issuer: `ParserCompositeSourceIssuerV1`, called
once directly inside `ParsedProgramWithCallableParameterSourceV1::new`, is the
sole issuer. It co-seals one parser invocation, one exact direct static provider
declaration/result syntax, and one nested final-root call/terminal site tree.
Later parser/materializer/transform/request types transport this authority but
cannot issue it. A future `CanonicalScriptCompositeProgramMembershipIssuerV1`
alone may issue the two-axis partition and call-site membership.

Non-authority: parser rows alone, source name/digest/ordinal/pointer,
caller-built windows, Builder work-plan/semantic products, the pointer-branded
Script target inventory, runtime static-Box completion, empty/default catalogs,
old Recipe, physical/publication owners, vm-reference reachability, and local
green tests cannot issue composite membership or A meaning.

Fail-fast boundary: a `Ready` parser token that would be dropped, reconstructed,
changed, or converted to compatibility rejects after source-backed transform
selection and before `VerifiedFinalCallableProgramSourceV1` is issued. Later
selector/capability failures remain before Recipe, Builder effects, physical
work, raw, retry, or fallback.

Smallest next slice: `SCRIPT-COMPOSITE-SOURCE-PRESERVATION-I0`: issue and move
one non-sync, non-Main static provider plus one final root MethodCall and its
receiver/ordered arguments/result/terminal tree through parser, materializer,
exact transform validation, final source, and default request. The cell ends at
request transport and belongs to the already-declared A-cutover series.

Non-claims: no generic/sync/instance Box admission, import expansion,
capability/A/C implementation, Recipe/Join, physical Call, publication/Return,
production switch, residual retirement, backend parity, ABI, or performance
claim.

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

### 3. Contingent source reownership — `SCRIPT-SOURCE-REOWN-I0-R0`

```text
Production caller:
  NormalDefaultPublishedPipelineV1::compile

Change:
  issue the neutral two-axis Program window in the default lifecycle
  preserve located and unlocated resolver deferral exhaustively

R0:
  delete the work-plan-local ScriptRootDemandWindowBuilder issuance edge
  delete the unit ResolveScriptForestOutcomeV1::Deferred information-loss edge

Done:
  existing target/resolver/Recipe consumers use the neutral source owner
  selected old window and unit-Deferred edges are caller-zero

Stop:
  mapping/failure policy differs, a cause site must be invented, or the
  parser-backed source authority does not reach this default caller
```

This row is BoxShape/T1 only if the mapping is behavior-identical. Otherwise
it returns to this D0.

### 4. Contingent lookup reownership — `SCRIPT-LOOKUP-REOWN-I0-R0`

```text
Production caller:
  the same default root/catalog lifecycle

Change:
  issue one owned AST-free target/result relation
  connect it immediately to the existing result/physical consumer chain

R0:
  delete VerifiedScriptDirectStaticCallTargetInventoryV1::issue on the
  selected Script edge and its address-brand/attach/take authority

Done:
  selected target/result lookup uses no AST borrow or pointer identity
  TargetOutsideCatalog behavior is unchanged

Stop:
  AST/body references or addresses remain, source admission is needed in
  the same slice, or no selected old edge can be deleted
```

This is a T2 new authority even when runtime behavior is preserved. It is not
a rename of the existing pointer product.

### 5. Series terminal — `SCRIPT-A-CUTOVER-I0-R0`

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

The parser-boundary Decision is closed because the SSOT now fixes:

1. candidate A, one non-`Clone` token, and one sole constructor call site;
2. the exact one-provider/final-root-call cohort and nested role tree;
3. parser witness as primary identity and private paths as coverage only;
4. total parser states distinct from later resolver `Deferred`;
5. the required Parsed -> Prepared -> transform -> VerifiedFinal -> request
   move chain with no parallel `Option` or reconstruction;
6. role-specific transform drift and `CompatibilityLoss` before final source;
7. one atomic implementation cell with focused positive/negative/structural
   evidence and explicit 800-line limits; and
8. a predeclared five-commit series ending in named default A cutover and old
   authority deletion.

`SCRIPT-COMPOSITE-SOURCE-PRESERVATION-I0` closes only when its `Done` block is
observable. Parser-token green does not authorize source admission, resolver,
lookup, A/C, physical work, or a production claim.

## Later-cell stop lines

The selected preservation cell returns to `design_stop` if its own `Stop`
condition is observed. The following conditions prevent advancing into their
corresponding later series cell; they do not authorize widening cell 1:

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

Only the selected preservation owner, its focused fixtures/tests, and its
reusable guard are authorized now. No source admission, fallback, production
switch, or downstream semantic `Verified*`/`Prepared*` receipt is authorized.

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

The current row is therefore `SCRIPT-COMPOSITE-SOURCE-PRESERVATION-I0` with
`work_mode = fast`. The same card owns its implementation contract; later cells
remain closed until this row's focused acceptance is green.

## References

- `docs/development/current/main/investigations/script-direct-static-a-issuer-boundary-d0-2026-08-21.md`
- `docs/development/current/main/investigations/script-direct-static-a-consumer-bind-d0-2026-08-21.md`
- `docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md`
- `src/parser/callable_parameter_source/canonical_script_source_admission.rs`
- `src/parser/normal_callable_program_source/semantic_syntax_loan.rs`
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
