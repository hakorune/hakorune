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

## Worker audit decision — D0-B/C remains open

The read-only audit converged on `NoSafeSlice` for the consumer implementation.
The parser-side App admission is a Keeper, and the normal Program-body loan is
the correct candidate authority for Script.  Neither currently proves the
final transformed root structure required by the existing App/Script lowerer:

```text
AppReady(P1) + final root AST(P2)
  -> exact root/static-child relation is not yet source-sealed
```

In particular, `validate_parser_normal_program_source_transform_v1` preserves
the covered body prefix and composite relation but currently permits additional
transformed root statements.  It therefore cannot yet be used as the root
cohort-preservation proof.  `VerifiedMainExpansionV1::from_program` remains a
second AST/name/ordinal classifier and is not an acceptable repair.

The next design-only tasks are consequently:

```text
D0-G  Root-cohort transform contract
      Decide how the existing parser root issuer rejects App/Script root
      additions, removals, and structural drift after the final transform.

D0-H  Parser-side App/Script structural issuer
      Extend the existing root issuer (not a Builder classifier) with a
      private structural relation: App seal + exact Main relation, or normal
      Program-body HRTB coverage for Script.  Keep parser anchors opaque and
      keep Script-A rows out of the normal-root path.

D0-I  Re-open the consumer decision
      Only after D0-G/H close, select the move-only lifecycle handoff and
      specify its typed zero-effect terminal mapping.  Until then, production
      root consumer count remains zero.
```

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

Therefore the next design decision has only two honest directions:

```text
A. Parser final-transform preservation
   Issue a parser-owned, non-Clone root-cohort preservation token at the
   final transform boundary.  It keeps App/Script structural relation
   opaque and is the only input the future root consumer may move.

B. Exact source-backed root cohort
   Tighten the source-backed root transform contract to an exact root cohort;
   route any added root statements through an explicitly typed compatibility
   or Outside terminal before the normal-root consumer.  This is a policy
   change and needs its own Decision; it must not be smuggled into the
   consumer implementation.
```

Neither option is a consumer SafeSlice yet.  In particular, the following
are not valid repairs: using the current raw expansion, comparing names or
ordinals in MIR, treating appended rows as default Script/App state, or
exposing `ParserMainAppEntrySealV1` to Builder.  D0-G must choose A or B and
name its sole parser issuer before D0-I can reopen implementation.

## Recommended Decision — conditional A

Choose A, conditional on the following preservation contract:

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
  initial App seal + unchanged source-root prefix + no App role drift
ScriptReady:
  unchanged source-root prefix + no new static Main in the transformed tail
both:
  same source product, same parser witness, Program shape, typed drift errors
```

App/Script structural extraction in Builder may then be a projection under an
already-admitted role.  It must not decide the role, reissue source facts, or
scan a second AST to override the token.  Added transformed tail rows remain
outside the source prefix and need an explicit typed policy; they are never
silently folded into the source cohort.

The implementation slice remains closed until the contract can be represented
without a foreign raw `ASTNode` transform input.  If the transform boundary
cannot carry the same-invocation witness into this final validation, revert to
NoSafeSlice and choose a separate transform-session design before adding the
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
paths, names, ordinals, AST pointers, or A rows to MIR.

## Structural projection decision still missing

For `ScriptReady`, the existing normal Program-body source loan is the natural
structural input. It does not need Script-A rows. The resulting Script route
must preserve the existing semantic package/window path without rebuilding a
Script label from AST absence.

For `AppReady`, the existing `VerifiedMainExpansionV1` needs an admitted-source
entry point, not its current classifier entry point. That entry point must
prove, under the same parser invocation and final-source transform:

```text
AppReady seal
  ↔ exact static Main declaration
  ↔ exact main method source relation
  ↔ accepted root/static-child structural projection
```

If this relation cannot be supplied without exposing parser anchors or adding a
second source issuer, the result is `NoSafeSlice`; do not weaken the check to
name, ordinal, digest, or AST shape equality.

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

## D0 tasks

```text
D0-A  Consumer ownership
      Freeze the lifecycle method as the sole named root consumer and census
      every current raw-root caller/bypass. Production consumer count is 0
      until this boundary is implemented.

D0-B  Move-only consume contract
      Select the typestate/HRTB handoff from VerifiedFinal source into the
      lifecycle. No getter, parallel Option, parser-anchor exposure, or second
      observer is permitted.

D0-C  App structural relation
      Decide which existing parser source authority can prove AppReady ↔ the
      structural data required by VerifiedMainExpansionV1. If no authority
      exists, record NoSafeSlice rather than issuing a guessed receipt.

D0-D  Script structural relation
      Define the ScriptReady input from the normal Program-body loan without
      importing CanonicalScriptSourceRowsV1 or reconstructing Script by absence.

D0-E  Pre-effect finite gate
      Fix typed terminal mapping and prove zero target-install, module,
      catalog, work-plan, MIR, and publication effects on every reject.

D0-F  Acceptance packet
      Name positive App/Script cases, every typed terminal negative, foreign
      invocation, parser-transform drift, AppReady-on-A mismatch, raw-classifier
      caller-zero, and one reusable structural guard.
```

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

The next execution may begin only after D0-B/C are accepted and the exact
source authority for App structural projection is named. Until then, the
previous transport I0 remains the complete bounded result and the root
consumer remains intentionally closed.
