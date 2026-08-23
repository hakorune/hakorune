Status: Prerequisite design accepted — exact-transform I0 is ready; root consumer remains closed
Date: 2026-08-23
Decision: NORMAL-MAIN-APP-ROOT-CONSUMER-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-root-source-disposition-d0-2026-08-23.md
ProductionCaller: root consumer 0; G0 replaces one existing normal callable transform edge
ProductionEdit: next slice is limited to the source-transform disposition; root consumer, raw classifier retirement, and root lowering remain closed
CeremonyTier: D0 — source-root consumer boundary before Builder effects
---

# NORMAL-MAIN-APP-ROOT-CONSUMER-D0

## Six-line brief

```text
Decision:
  keep ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle_with_target
  as the only future root consumer, but first close the normal callable macro
  boundary as ExactUnchanged versus an actual generated-tail compatibility row.
Source authority + canonical issuer:
  issue_parser_normal_root_source_v1 remains the sole source-root issuer;
  ParserNormalProgramSourceAuthorityV1 remains the exact body-coverage owner;
  the macro/test-harness owner alone may issue GeneratedTail, and the existing
  parser final-source issuer alone may seal ExactUnchanged source.
Non-authority:
  VerifiedRawRootExpansionV1::from_program as a classifier, root_is_app_mode,
  env flags, AST equality or a raw AST callback as transform authority,
  AST/name/ordinal rescans, NormalCompileRequest, Builder state, compatibility
  retry, and raw fallback.
Fail-fast boundary:
  classify an actual generated tail before ParserNormalRootPreservedV1 or any
  final source product can exist; later root consume still precedes target,
  module, catalog, work-plan, MIR, and publication effects.
Smallest next slice:
  NORMAL-CALLABLE-SOURCE-TRANSFORM-DISPOSITION-I0: replace the production raw
  AST callback with ExactUnchanged and route only an actually generated test
  tail to typed compatibility; do not implement the root consumer.
Non-claims:
  generated-tail canonical semantics, root lowering, ABI/result semantics,
  child scheduling, MIR/ValueId, raw retirement, fallback, root production
  switch, test-flag semantics, and performance.
```

## Current evidence

The previous transport I0 is landed as `eeac2da553`, with closeout pointer
`ef88ba6c53`. The source-root disposition now moves through the parser source
chain as one required field:

```text
ParsedProgramWithCallableParameterSourceV1
  -> PreparedNormalCallableProgramSourceV1
  -> VerifiedFinalCallableProgramSourceV1
  -> PreparedNormalDefaultProgramRootV1
```

`CanonicalScriptSourceRowsV1` remains the separate Script-A handoff product.
The reference A route explicitly discards the root disposition and rejects
`AppReady`; no root consumer exists yet.

The current root lifecycle still does this twice:

```text
PreparedNormalDefaultProgramRootV1
  -> VerifiedRawRootExpansionV1::from_program(source_ast)
  -> is_app_mode: bool
  -> Builder/root work plan
```

The second scan occurs after catalog installation, before lowerer invocation.
This is a second root observer and is not allowed to become the canonical
consumer of the parser disposition.

## Fresh top-down worker audit — `NoSafeSlice` confirmed

The requested read-only top-down audit confirms `NoSafeSlice` for consumer
implementation.  The parser-side App admission is a Keeper, and the normal
Program-body loan is the correct candidate authority for Script.  Neither
currently proves the final transformed root structure required by the existing
App/Script lowerer:

```text
AppReady(P1) + final root AST(P2)
  -> exact root/static-child relation is not yet source-sealed
```

In particular, `validate_parser_normal_program_source_transform_v1` preserves
the covered body prefix and composite relation but currently permits additional
transformed root statements.  It therefore cannot yet be used as the root
cohort-preservation proof.  `VerifiedMainExpansionV1::from_program` remains a
second AST/name/ordinal classifier and is not an acceptable repair.

The current issuer chain is only an admission chain:

```text
ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once
  -> issue_parser_main_app_entry_v1
  -> issue_parser_normal_root_source_v1
```

`ParserMainAppEntrySealV1` proves the Main declaration/method admission, but it
does not yet co-seal the root body, static-child relation, and final transformed
AST.  That missing co-seal is the actual blocker; adding a Builder-side
projector before it exists would create a second authority.

For Script, `ParserNormalProgramSourceAuthorityV1` and its HRTB loan are the
right source owner, but the current loan still exposes `program()` and the
transform guard permits an appended root suffix.  The future root boundary
must therefore use a narrower root-only view and must not hand raw AST access
to the Builder consumer.

The ordered prerequisite series is fixed below.  G0 alone is open; until J0
lands, the production root consumer count remains zero.

### Second top-down worker audit — the transform boundary is the real predecessor

The App admission data is stronger than the first audit needed to assume:
`ParserMainAppEntrySealV1` already proves exact static `Main`, exactly one
direct member, `main/0`, and one callable identity under the parser invocation.
The remaining unsafe input is the final transform itself:

```text
parser-owned source prefix
  -> ParserNormalCallableTransformSessionV1::finish(FnOnce -> ASTNode)
  -> maybe_expand_and_dump
  -> test harness may append top-level setup/call rows
  -> prefix-only root preservation currently accepts the unowned suffix
```

The direct counterexample is a Script-ready top-level `test_*` function.  The
test harness can append its call after all parser-owned rows.  The appended row
has no parser invocation relation, yet the current prefix check can still issue
a Ready root token.  Conversely, changing only the check to exact length would
silently reject the existing test-harness feature.

The accepted correction is an upstream transform disposition:

```text
normal callable macro owner
  ├─ ExactUnchanged
  │    -> ParserNormalCallableTransformSessionV1::finish_exact(self)
  │    -> exact final source/root preservation
  │
  └─ TestHarnessGeneratedTail(transformed AST)
       -> explicit Compatibility(TestHarnessGeneratedTail)
       -> parser final source/root token count = 0
```

This is not a retry after canonical rejection.  The macro owner issues the
disposition while performing the transform, before the parser final-source
attempt starts.  The parser does not reread `NYASH_TEST_RUN`, infer suffix
origin from AST shape, or compare a foreign AST to recover authority.

If composite source preservation is Ready, an actual generated tail remains a
typed `CompatibilityLoss` rejection, matching the existing registered-macro
and default-derive rule.  It may not discard that stronger source authority to
enter compatibility.  If the macro engine mutates the AST despite no named
registered/default transform, the result is a typed unclassified-mutation
reject, not `ExactUnchanged` and not an anonymous compatibility row.

Canonicalizing generated test rows in the root token is rejected for this
slice.  That would require a second macro-generated source authority, complete
generated rows, and Recipe coverage for those rows.  The bounded root lane
instead contains parser source only.

The following are not valid repairs: using the current raw expansion,
comparing names or ordinals in MIR, treating appended rows as default
Script/App state, or exposing `ParserMainAppEntrySealV1` to Builder.

## Recommended design — A-prime behind one exact-transform gate

Choose a narrowed A-prime after the accepted transform prerequisite:

```text
one parser-owned source product
  -> one macro-owned ExactUnchanged disposition
  -> one parser finish_exact move
  -> one opaque, non-Clone root-preservation token
  -> one move-only normal-root consumer
```

The token is issued inside the existing parser root-source authority family;
it is not a Builder classifier and it exposes no parser anchor, name, ordinal,
digest, pointer, or Script-A row.  Its private proof must establish:

```text
AppReady:
  exact parser Main admission + exact root/static-child relation + unchanged
  final-transform root cohort + no App role drift
ScriptReady:
  exact normal Program-body root relation + unchanged final-transform root
  cohort + no new Main role
both:
  same source product, same parser witness, exact root shape, typed drift errors
```

App/Script structural extraction in Builder may then be a projection under an
already-admitted role.  It must not decide the role, reissue source facts, or
scan a second AST to override the token.  An actual generated test tail never
enters this bounded root lane: it is classified by its generator before parser
finalization.  Unknown additions, removals, reorderings, or replacements remain
typed rejects.

The first implementation slice is now bounded and accepted.  The root consumer
itself remains closed until exact-root co-seal and the narrow HRTB view land.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | exact parser App `Main.main/0` relation | Builder route or MIR |
| `issue_canonical_script_cohort` / Script-row issuer | positive Script cohort and A rows | App selection or root effects |
| `issue_parser_normal_root_source_v1` | same-invocation App/Script disposition and typed terminals | structural lowering or Builder state |
| `ParserNormalProgramSourceAuthorityV1` | exact Program-body coverage and HRTB AST/source loan | App/Script selection, target, MIR |
| normal callable macro/test-harness transform issuer | whether this invocation actually generated a test tail and its transformed compatibility AST | parser/source authority or root admission |
| `ParserNormalCallableTransformSessionV1::finish_exact` | one-shot transition from unchanged parser source to final source issuance | arbitrary foreign AST acceptance or compatibility selection |
| `ParserNormalRootPreservationIssuerV1` | exact final App/Script preservation after the transform gate | macro-tail provenance or Builder projection |
| `PreparedNormalDefaultProgramRootV1` | move-only transport wrapper | source reclassification |
| `ModuleBuilderInvocationSessionV1` lifecycle | future one-time root consumer and unpublished session | parser truth or second root classifier |
| `VerifiedRawRootExpansionV1` | legacy structural projection candidate only | App/Script authority |
| `PreparedProgramRootWorkPlanV1` | already-admitted root work plan | root selection or source inference |

The key missing authority is not another App/Script classifier. It is an
exact, source-backed relation from `AppReady` to the structural data needed by
the existing App lowerer. The existing `VerifiedMainExpansionV1::from_program`
does not provide that relation: it independently searches for `Main`, accepts
shapes the parser admission rejects, and does not carry the parser witness.

## Candidate consumer and required API shape

The only suitable consumer boundary is the beginning of
`complete_normal_default_program_root_catalog_lifecycle_with_target`, before
`install_pinned_text_target_capability` and before
`prepare_normal_default_module`.

The consumer must be move-only and opaque across the parser/MIR boundary. A
plain getter is rejected because it permits an unconsumed disposition and a
second raw observer. The design must choose one of these equivalent shapes:

```text
PreparedNormalDefaultProgramRootV1
  -> consume_root_source(self)
  -> RootConsumerInputV1
  -> lifecycle
```

or an HRTB callback whose callback owns the complete unpublished root session
and cannot return parser anchors or a borrowed AST. The resulting type must
make a second consume impossible without adding a parallel `Option` state.

The consumer input may expose only closed `App` / `Script` root roles plus
typed terminal errors. It must not expose `ParserMainAppEntrySealV1`, source
paths, names, ordinals, AST pointers, or A rows to MIR.  The safest shape is a
parser-owned HRTB callback that lends a root-only view for the duration of the
named consumer; a self-referential struct containing an owned root AST and a
borrow into that same AST is not allowed.

The current `ParserNormalProgramSourceLoanV1::program()` is therefore not a
valid root-consumer boundary.  D0-I must either narrow it or introduce a
separate `with_normal_root_consumer_view(...)` API whose view has no raw
Program accessor.

## Structural projection decision still missing

For `ScriptReady`, the existing normal Program-body source authority is the
natural structural input. It does not need Script-A rows. The resulting Script
route must preserve the existing semantic package/window path without
rebuilding a Script label from AST absence.  The view must be paired and
HRTB-scoped; raw `program()` access is not part of the contract.

For `AppReady`, the existing `VerifiedMainExpansionV1` needs an admitted-source
entry point, not its current classifier entry point.  The parser-side root
issuer (the existing final-transform preservation issuer, or a parser-private
sub-issuer called only by it) must first co-seal the exact relation.  The
Builder entry point must then accept that proof rather than search the AST.
The relation must prove, under the same parser invocation and final-source
transform:

```text
AppReady seal
  ↔ exact static Main declaration
  ↔ exact main method source relation
  ↔ accepted root/static-child structural projection
```

If this relation cannot be supplied without exposing parser anchors or adding a
second source issuer, the result is `NoSafeSlice`; do not weaken the check to
name, ordinal, digest, pointer, or AST shape equality.

## Accepted bounded task series after both worker audits

The series is ordered by authority.  Only the first row is open.  A later row
does not become executable merely because an earlier focused test is green.

```text
G0 / NORMAL-CALLABLE-SOURCE-TRANSFORM-DISPOSITION-I0  [open, BoxCount]
      Make the macro/test-harness owner return Unchanged or actual
      GeneratedTail.  Only Unchanged may call a no-argument finish_exact;
      GeneratedTail enters the existing typed compatibility lane before root
      token issuance.  Remove the production FnOnce -> ASTNode finish API.

G1 / NORMAL-MAIN-ROOT-EXACT-COHORT-I0                 [closed behind G0]
      Require source body count == initial count == final count and exact full
      statement preservation.  Addition, removal, reorder, or replacement is
      a typed reject.  Do not add logic to main_expansion.rs.

H1 / NORMAL-MAIN-APP-ROOT-RELATION-I0                 [closed behind G1]
      In the existing final-source issuer, co-seal the private App admission,
      exactly one matching callable identity, its paired final slot, the same
      invocation, Main-only-member relation, and NoStaticChildren.  Do not
      expose parser sites, anchors, names, or ordinals.

I0 / NORMAL-ROOT-CONSUMER-LOAN-I0                     [closed behind H1]
      Add one parser-owned HRTB root view.  App lends only typed root body plus
      CallableMainIsRoot/NoStaticChildren; Script lends a paired statement
      cursor.  There is no raw Program getter and no Script-A row.

J0 / NORMAL-MAIN-APP-ROOT-CONSUMER-I0                 [closed behind I0]
      Move the final root source once into the named lifecycle consumer before
      target/module/catalog/work-plan effects.  All non-ready states terminate
      typed; Ready cannot return to the old raw route.

K0 / NORMAL-MAIN-APP-RAW-ROOT-OBSERVER-R0             [closed behind J0]
      Remove the three selected-normal raw root observers: lifecycle preflight,
      source-backed callable-catalog classification, and post-catalog rescan.
      Remove bool-based selection and prove fallback/retry zero.  Compatibility
      and test-owned raw observers remain under their existing owner.
```

The deterministic source sentence is:

```text
parser exact App/Script source
  -> macro owner ExactUnchanged
  -> parser exact-root relation
  -> narrow HRTB root view
  -> one pre-effect lifecycle consumer
  -> existing root Recipe/work plan
```

Generated test tails leave this sentence before parser final-source issuance.

## Finite state table

| Root state | Consumer result | Effects before terminal | Allowed next step | Fallback |
| --- | --- | ---: | --- | --- |
| `AppReady` | admitted App root input, only after exact structural relation | 0 | root lifecycle | none |
| `ScriptReady` | admitted Script root input from normal body authority | 0 | existing Script semantic path | none |
| `Outside` | typed root terminal | 0 | discard session | none |
| `ScriptTerminal(_)` | typed Script-source terminal | 0 | discard session | none |
| `SourceAuthorityUnavailable(_)` | typed source terminal | 0 | discard session | none |
| `Incomplete(_)` | typed incomplete terminal | 0 | discard session | none |
| `IntegrityInvalid(_)` | typed integrity terminal | 0 | discard session | none |
| `DiscardedBeforeA` | route mismatch on normal root path | 0 | typed reject | none |
| compatibility outer lane | existing compatibility owner | existing compatibility policy | compatibility route only | no synthetic root state |

`AppReady` and `ScriptReady` are not candidate/noncandidate dispositions. They
are source-root admission states. The root consumer must not turn a terminal
state into Script, App, compatibility, or a legacy retry.

## Counterexamples that must remain rejected

```text
static box Main { main(argument) { return argument } }
  parser: Outside(NonZeroMainArity)
  raw AST classifier: may choose App
  required result: typed terminal, no Builder effect

static box Main { main() {} helper() {} }
  parser: IntegrityInvalid(StaticParentMemberCoverageMismatch)
  raw expansion: may accept extra static children
  required result: typed terminal, no Builder effect

parser App/Script witness from invocation P1 + final AST/source from P2
  required result: ParserWitnessMismatch / integrity terminal

AppReady on Script-A frontdoor
  required result: existing typed reject, never Script discard

top-level test_* function with an actual generated harness call
  required result: Compatibility(TestHarnessGeneratedTail), root token count 0

composite-ready source with an actual generated harness tail
  required result: CompatibilityLoss, no compatibility origin and no root token

P1 source session + a structurally identical raw AST produced independently
  required result: impossible on the production exact API; no raw callback
```

## NoSafeSlice conditions

Remain at `design_stop` if any one holds:

```text
no single named root consumer can be connected before Builder effects
App structural projection has no source-owned exact relation
actual generated tail cannot be distinguished by the generator itself
the parser must reread an env flag or infer suffix origin from AST structure
the production final-source API still accepts FnOnce(&ASTNode) -> ASTNode
an unclassified macro mutation can enter ExactUnchanged or compatibility
composite Ready can be discarded for generated-tail compatibility
VerifiedRawRootExpansionV1::from_program must remain the App/Script classifier
root disposition must be exposed through parser anchors, names, or ordinals
root consumer needs Script-A rows or clones them
consume requires a parallel Option/default/compatibility state
non-ready states cannot terminate before target/module/catalog effects
root lifecycle must retain a second AST scan as an authority
root bool or semantic-package presence remains the selection input
the source-to-Recipe sentence cannot be stated in one deterministic line
the bounded slice needs root lowering, ABI, publication, fallback, or switch
```

## G0 acceptance and execution gate

The main-thread source audit and the independent top-down worker audit agree on
the same finite G0 mapping.  The following evidence is required for the bounded
implementation:

```text
macro disabled or actual no-op
  -> SourceBacked via finish_exact; transformed AST argument count = 0

actual nonempty test-harness tail, composite source not Ready
  -> Compatibility(TestHarnessGeneratedTail); root token count = 0

actual nonempty test-harness tail, composite source Ready
  -> typed CompatibilityLoss; compatibility origin count = 0

registered macro/default derive
  -> existing disposition unchanged

unexpected macro-engine mutation outside named dispositions
  -> typed reject; no source-backed/compatibility fallback
```

Structural guards must prove one production `finish_exact` caller, zero
production raw transform callbacks, one GeneratedTail issuer, zero parser env
reads, zero post-reject retry, and zero root token issuance on generated-tail
paths.  Focused positive/negative tests, `cargo check`, pointer guard, and
`git diff --check` close G0.  Production source remains below the 760-line split
trigger and 800-line hard stop; `main_expansion.rs` (743 lines) and
`program_root_work_plan.rs` (721 lines) are forbidden edit targets in G0-H1.

G0 changes one bounded transform disposition and is therefore the selected
BoxCount.  It does not authorize G1, H1, I0, J0, or K0 in the same change.

## Non-claims

This card does not authorize:

```text
root consumer implementation
VerifiedMainExpansionV1 API changes
new root semantic receipt
canonical semantics for generated test-harness rows
root/static-child/body lowering
ProgramRootWorkPlan bool removal
root_is_app_mode removal
MIR/ValueId/CFG/publication changes
compatibility retirement
fallback/retry
production switch
performance work
```

The next execution is only
`NORMAL-CALLABLE-SOURCE-TRANSFORM-DISPOSITION-I0`.  The previous root transport
I0 remains complete, and the root consumer remains intentionally caller-zero
until the ordered prerequisite series reaches J0.
