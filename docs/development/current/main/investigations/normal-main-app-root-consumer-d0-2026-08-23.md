Status: Design stop — `NamedConsumerMissing` / root structural relation is not closed
Date: 2026-08-23
Decision: NORMAL-MAIN-APP-ROOT-CONSUMER-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-root-source-disposition-d0-2026-08-23.md
ProductionCaller: 0; design only
ProductionEdit: none; root consumer, raw classifier retirement, and root lowering remain closed
CeremonyTier: D0 — source-root consumer boundary before Builder effects
---

# NORMAL-MAIN-APP-ROOT-CONSUMER-D0

## Six-line brief

```text
Decision:
  select ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle_with_target
  as the only future named consumer of the parser-owned root disposition.
Source authority + canonical issuer:
  issue_parser_normal_root_source_v1 remains the sole source-root issuer;
  ParserNormalProgramSourceAuthorityV1 remains the exact body-coverage owner.
  The exact App structural projection issuer is still missing and must be named
  before any new Verified* product or root bridge implementation.
Non-authority:
  VerifiedRawRootExpansionV1::from_program as a classifier, root_is_app_mode,
  bool flags, AST/name/ordinal rescans, NormalCompileRequest, Builder state,
  semantic-package presence, compatibility retry, and raw fallback.
Fail-fast boundary:
  consume the root disposition and complete its structural relation before
  target installation, module preparation, catalog installation, work-plan
  admission, MIR effects, or publication.
Smallest next slice:
  design the move-only root-consumer input and exact App/Script structural
  projection relation; do not implement the consumer until both are closed.
Non-claims:
  root lowering, ABI/result semantics, child scheduling, MIR/ValueId, raw
  retirement, compatibility policy, fallback, production switch, and perf.
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

The active design-only tasks are consolidated below as D0-G through D0-L.
Until that packet closes, the production root consumer count remains zero.

### Concrete D0-G/H outcome

The current data model cannot close this relation by inspection alone:

```text
ASTNode::Program top-level rows
  -> no parser invocation/source anchor
BoxMethodInventoryV1
  -> method placement/provenance, not top-level source identity
transform guard
  -> preserves the covered prefix but permits appended statements
```

The active design decision is the narrowed A-prime described below: a
parser-owned, non-Clone exact root-cohort relation followed by a move-only
consumer.  A transformed top-level addition is a typed terminal in this
bounded lane; it is not silently admitted and it does not open a compatibility
fallback.  If A-prime cannot be issued by one parser authority, the result is
`NoSafeSlice`; do not switch to a Builder classifier or quietly broaden the
root policy.

The following are not valid repairs: using the current raw expansion,
comparing names or ordinals in MIR, treating appended rows as default
Script/App state, or exposing `ParserMainAppEntrySealV1` to Builder.

## Recommended design — conditional A-prime, implementation still stopped

Choose a narrowed A-prime, conditional on the following preservation contract:

```text
one parser-owned source product
  -> one final-transform validation
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
scan a second AST to override the token.  For this bounded root lane, an added
transformed top-level tail is a typed terminal (`RootCohortOutside` or an
equivalent source-owned terminal), not an implicitly accepted App/Script row.
Compatibility handling, if needed later, is a separate policy decision.

The implementation slice remains closed until the contract can be represented
without a foreign raw `ASTNode` transform input.  If the transform boundary
cannot carry the same-invocation witness into this final validation, remain at
`NoSafeSlice` and choose a separate transform-session design before adding the
root consumer.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | exact parser App `Main.main/0` relation | Builder route or MIR |
| `issue_canonical_script_cohort` / Script-row issuer | positive Script cohort and A rows | App selection or root effects |
| `issue_parser_normal_root_source_v1` | same-invocation App/Script disposition and typed terminals | structural lowering or Builder state |
| `ParserNormalProgramSourceAuthorityV1` | exact Program-body coverage and HRTB AST/source loan | App/Script selection, target, MIR |
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

## Consolidated design tasks after the worker audit

These tasks supersede the earlier informal D0-B/C wording. They are design
tasks only while `work_mode = design_stop`; no code, fixture, fallback, or
semantic `Verified*`/`Prepared*` product may be added yet.

```text
D0-G  Exact final root-cohort policy
      Decide whether the source-backed normal root lane admits only an exact
      final Program root cohort. Define typed handling for any transformed
      top-level addition, removal, or role drift. Ready must never include a
      silently accepted suffix.

D0-H  Parser-owned App structural co-seal
      Name the one parser-side issuer that co-seals Main admission, exact
      root/static-child relation, same invocation witness, and final-transform
      preservation. Keep ParserMainAppEntrySealV1 and all parser anchors
      private. If the existing issuer family cannot do this without a second
      authority, record NoSafeSlice instead of creating a Builder receipt.

D0-I  Narrow Script root view
      Define a paired HRTB root-consumer view from
      ParserNormalProgramSourceAuthorityV1. It must exclude raw Program
      access, Script-A rows, names, ordinals, pointers, and absence-based
      App/Script selection. Specify the exact final-root coverage it proves.

D0-J  Move-only named consumer boundary
      Choose consume(self) or a scoped HRTB callback from
      PreparedNormalDefaultProgramRootV1 into the single lifecycle consumer.
      Avoid self-referential borrowed/owned input, parallel Option state, and
      public getters. Specify AppReady, ScriptReady, and every typed terminal.

D0-K  Raw-observer retirement census
      Enumerate every production caller of VerifiedRawRootExpansionV1::from_program,
      VerifiedMainExpansionV1::from_program, root_is_app_mode, and the second
      post-catalog scan. Define the exact caller-zero and fallback-zero proof
      required after the future consumer is connected.

D0-L  Pre-effect acceptance packet
      Fix positive App/Script cases, foreign invocation, foreign final AST,
      prefix/root drift, Main arity, extra static child, appended root tail,
      AppReady-on-Script-A, and missing/duplicate rows. Every reject must
      prove zero target/module/catalog/work-plan/MIR/publication effects.
      Include one reusable structural guard and the 760/800-line budget.
```

`D0-G` through `D0-L` close only when the source-to-Recipe sentence is
deterministic and the named consumer can be described without a raw AST
classifier. Until then, the correct status is `NoSafeSlice`, not a partial
consumer implementation.

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
```

## NoSafeSlice conditions

Remain at `design_stop` if any one holds:

```text
no single named root consumer can be connected before Builder effects
App structural projection has no source-owned exact relation
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

## Task status and execution gate

The earlier D0-A through D0-F outline is retained as history in the parent
card, but the active task list is now the consolidated D0-G through D0-L
packet above.  The fresh worker audit did not authorize implementation: the
production root consumer count remains `0`, and this card remains
`design_stop` until the App co-seal, narrow Script view, and raw-observer
retirement contract are all closed.

## Non-claims

This card does not authorize:

```text
root consumer implementation
VerifiedMainExpansionV1 API changes
new root semantic receipt
root/static-child/body lowering
ProgramRootWorkPlan bool removal
root_is_app_mode removal
MIR/ValueId/CFG/publication changes
compatibility/raw retirement
fallback/retry
production switch
performance work
```

The next execution may begin only after D0-G through D0-L are accepted and the
exact source authority for App structural projection is named. Until then, the
previous transport I0 remains the complete bounded result and the root
consumer remains intentionally closed.
