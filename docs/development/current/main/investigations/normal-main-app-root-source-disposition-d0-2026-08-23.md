Status: I0 implementation complete — root consumer remains design-stopped
Date: 2026-08-23
Decision: NORMAL-MAIN-APP-ROOT-SOURCE-DISPOSITION-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-root-bridge-d0-2026-08-23.md
ProductionCaller: 0; design only
ProductionEdit: parser/transport I0 landed; root semantic consumer remains closed
CeremonyTier: D0 — parser source-root disposition before the Builder bridge
---

# NORMAL-MAIN-APP-ROOT-SOURCE-DISPOSITION-D0

## Six-line brief

```text
Decision:
  design one parser-owned total source-root disposition before the normal
  Builder root lifecycle consumes App/Script meaning.
Source authority + canonical issuer:
  existing parser issuers remain the source facts owners; a future
  ParserNormalRootSourceDispositionIssuerV1 may co-seal them once inside
  ParsedProgramWithCallableParameterSourceV1::new.
Non-authority:
  ParserMainAppEntry Outside as Script evidence, raw AST expansion,
  root_is_app_mode, NormalCompileRequest, Builder state, names, ordinals,
  compatibility retry, and a negation-based Script classifier.
Fail-fast boundary:
  source-root disposition must be complete and same-invocation before it is
  transported to the root lifecycle; missing/contradictory rows terminate
  before root/catalog Builder effects.
Smallest next slice:
  implement one parser-owned source-root disposition transport with explicit
  root consume versus A-route discard; no root lowering or raw classifier.
Non-claims:
  root body lowering, ABI/result semantics, child scheduling, MIR/ValueId,
  publication, compatibility retirement, production switch, and performance.
```

## Why the bridge cannot be implemented yet

The current source-backed product transports
`ParserMainAppEntryDispositionV1`, but its `Outside` arm only means that the
first App cohort was not admitted. It does not prove that the source is a
Script root. The direct counterexample is a source with a static `Main` whose
`main` has nonzero arity: the old raw expansion calls it App, while turning it
into Script would silently erase the parser's explicit unsupported state.

The parser already has a positive pure-Script admission in
`CanonicalScriptCohortAdmissionV1` and exact rows in
`CanonicalScriptSourceRowsV1`. Those products are currently sibling parser
products and are not part of the normal callable final-source transport. A
Builder adapter cannot safely reconstruct their meaning later.

The D0-A audit found an additional ownership constraint. The rows already
have a separate one-shot move chain:

```text
CanonicalScriptSourceRowsV1
  -> CanonicalScriptParserSourceHandoffV1
  -> CanonicalScriptSourceAInputTransportV1
  -> SourcePlanEnvelope
```

The normal callable path deliberately moves those rows to
`MovedToParallelHandoff` and does not receive them. Therefore the root source
disposition must not take ownership of `CanonicalScriptSourceRowsV1` merely to
obtain a Script label. The root path may own the existing opaque
`CanonicalScriptCohortAdmissionV1` witness and validate its same-invocation
relation; the A rows remain owned by the A handoff. The two frontdoors must be
mutually exclusive and must have an explicit consume/discard order.

## Candidate source authority and issuer

The intended authority split is:

| Owner | Owns | Must not own |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | exact App `Main.main/0` source relation | Script admission or root effects |
| `issue_canonical_script_cohort` plus source-row issuer | positive pure-Script cohort and exact Script rows | App selection or Builder effects |
| future `ParserNormalRootSourceDispositionIssuerV1` | one same-invocation co-seal of the existing source facts | AST re-observation, semantic root lowering, MIR |
| future normal root bridge | consume the moved source disposition once | parser classification or raw-bool repair |
| `VerifiedRawRootExpansionV1` | structural projection after explicit admission | App/Script selection |

The accepted I0 issuer is the private
`issue_parser_normal_root_source_v1` facade. It is called once from
`ParsedProgramWithCallableParameterSourceV1::new`; the root consumer remains a
separate future cell.

## Required total mapping

The successor must settle a closed mapping similar to this one:

| Existing parser evidence | Required source-root state | Rule |
| --- | --- | --- |
| `AppMainReady(seal)` plus matching StaticBox cohort | `AppRootReady` | exact App seal; no raw classifier |
| `CanonicalScriptCohortAdmitted` plus `HandoffReady(rows)` with the same parser brand | `ScriptRootReady` | positive Script evidence only |
| `Outside(any reason)` without a positive Script witness | typed `Outside` terminal | never Script by negation |
| Script cohort deferred/unresolved/incomplete/invalid | typed Script-source terminal | no empty/default rows |
| source unavailable/incomplete/integrity-invalid | matching typed source terminal | zero root effects |
| explicit compatibility source | existing compatibility owner | no synthetic App/Script state |

The exact nested reason vocabulary must be preserved. In particular,
`ProgramCohort`, `MultipleParentRows`, `BuildGatePath`,
`UnsupportedMemberKind`, `DirectMethodCohort`, `NonMainStaticBox`,
`NonMainMethod`, and `NonZeroMainArity` may not be collapsed into a generic
Script fallback or erased into `Option::None`.

The issuer must also reject a contradictory pair, for example an
`AppMainReady` seal alongside a non-compatibility Script admission, as a
typed same-invocation integrity error.

## Required move shape

The preferred shape is one parser-owned source transport, not parallel root
fields:

```text
ParsedProgramWithCallableParameterSourceV1
  -> ParserNormalRootSourceDispositionV1
  -> PreparedNormalCallableProgramSourceV1
  -> VerifiedFinalCallableProgramSourceV1
  -> PreparedNormalDefaultProgramRootV1
  -> NormalCompileRequestV1
  -> one normal root consumer
```

The root consumer may match the disposition, but it must not need parser
private AST anchors or recreate the source relation. The existing Main/App
transport may be replaced by this unified transport only in a bounded series
that preserves the parser sole issuer and removes the redundant sibling field.
Adding a second independent root field is not accepted.

## Design tasks

```text
D0-A  Source witness census — completed as a constraint
      CanonicalScriptCohortAdmissionV1 can be the root's opaque positive
      Script witness. CanonicalScriptSourceRowsV1 remains the existing A
      handoff's one-shot owner and may not be moved into a parallel root
      product. The root/A ordering is still unresolved.

D0-B  Total mapping contract
      Enumerate every Main/App and Script disposition, including all nested
      Outside and source-row states, and assign AppRootReady, ScriptRootReady,
      or one typed terminal. No default arm.

D0-C  Cross-module move contract
      Choose the single parser-owned transport shape and visibility. Preserve
      non-Clone ownership, same parser brand, and exactly-once consumption.
      Specify whether the A frontdoor explicitly discards the root witness
      before moving its rows, and reject AppReady on that route.

D0-D  Structural expansion contract
      Define an App-admitted structural projection that validates exact
      source relation without calling from_program as a classifier. Define
      the Script structural projection separately.

D0-E  Successor I0 acceptance packet
      Name one root consumer, prove zero effects before disposition consume,
      plan positive/negative transport tests, and define one reusable guard.
```

## NoSafeSlice conditions

Remain at `design_stop` if any of the following holds:

```text
no positive Script witness can be transported without a second authority
the Script rows are needed by two consumers but cannot be co-sealed/borrowed
AppMainReady and structural expansion cannot be related by parser identity
any Outside mapping requires raw AST reclassification or a fallback
the unified transport needs a parallel Option/default field
the root lifecycle must consume before source disposition is complete
the structural projection has to choose App/Script itself
the design changes ABI, body lowering, MIR, publication, or compatibility
the bounded slice exceeds the 760-line split trigger
```

Before the accepted I0 Decision, implementation, fixtures, request changes,
root writes, fallback/retry, and production switching were forbidden. Those
restrictions remain for the root-consumer cell; this I0 only transports the
parser-owned disposition and closes the A-route discard boundary.

## D0-A evidence packet

```text
parser Script admission issuer              = 1
parser Script rows issuer                   = 1
parser App admission issuer                 = 1
normal callable rows consumer                = 0
reference Script A handoff move chain        = 1
compiler A production consumer               = 0 (discard boundary only)
AST/name/ordinal reconstruction              = forbidden
AppMainEntry seal exposed to MIR             = forbidden
```

The remaining design question was not whether the parser may issue both source
facts; it already does so in one invocation. It was how one root-only opaque
witness and the existing A-only rows are selected without allowing either
frontdoor to silently discard the other's Ready state. D0-B/C is now fixed by
the following bounded state machine:

```text
normal callable root path:
  ParserNormalRootSourceDispositionV1::AppReady/ScriptReady
    -> move-consume once by the future root bridge

reference Script-A path:
  move the same root disposition to DiscardedBeforeRoot
    -> then move CanonicalScriptSourceRowsV1 to the existing A handoff

AppReady on the Script-A path:
  typed reject; it may not be silently discarded
```

The root disposition owns only the opaque `AppMainReady` seal or the existing
`CanonicalScriptCohortAdmissionV1`; it never owns or clones
`CanonicalScriptSourceRowsV1`. The rows remain the sole A-handoff product.
The parser issuer validates the rows' same-invocation witness before issuing
`ScriptReady`, while the later root structural projection uses the existing
parser normal-program source authority rather than re-reading A rows.

This is a transport/source-admission product, not App/Script semantic root
meaning. The root consumer and structural projection remain the next cell.

## Accepted bounded I0 contract

## NORMAL-MAIN-APP-ROOT-SOURCE-DISPOSITION-I0

The source-owned issuer is a private parser facade called once from
`ParsedProgramWithCallableParameterSourceV1::new`. It consumes the existing
App disposition, Script cohort disposition, Script-row disposition, and
same-invocation normal source authority relation; it does not scan the AST.

The accepted mapping is:

| Existing evidence | I0 state | Rule |
| --- | --- | --- |
| `AppMainReady` + StaticBox-compatible Script state | `AppReady` | move-only opaque App seal |
| `Outside(ProgramCohort)` + `CanonicalScriptCohortAdmitted` + `HandoffReady(rows)` with the same witness | `ScriptReady` | move-only opaque Script cohort witness |
| any other `Outside` reason | `Outside(reason)` | preserve nested reason; no Script fallback |
| Script/source unavailable, deferred, incomplete, or invalid | typed terminal | no empty/default row |
| contradictory App/Script/row brands | `IntegrityInvalid` | reject before downstream effect |

`ScriptReady` does not contain the A rows. A-route discard is an explicit
terminal transition, and normal-root consumption is the only later consumer
of `AppReady`/`ScriptReady`.

The I0 may replace the redundant Main/App transport sibling with the unified
source-root transport, but it may not add a parallel `Option` field, expose
parser anchor getters to MIR, or change `NormalCompileRequestV1`.

## I0 acceptance and non-claims

Positive/negative evidence must cover AppReady, pure ScriptReady, each typed
terminal family, foreign/contradictory parser witnesses, AppReady on the A
route, and explicit root-discard-before-A ordering. All rejects must occur
before root/Builder effects. A reusable transport guard must prove one parser
issuer, one root field through Prepared/Final source, one A-only rows owner,
zero root reclassification, and zero fallback/retry.

This I0 does not claim a root consumer, App/Script lowering, structural
expansion, ABI/result semantics, MIR/ValueId, publication, compatibility
retirement, production switch, or performance.

## I0 implementation evidence

```text
parser root co-seal issuer                         = 1
unified root field Parsed -> Prepared -> Final     = 1
Script rows remain separate A-only product          = 1
AppReady on A route                                = typed reject
Script root on A route                             = explicit DiscardedBeforeA
AST/name/ordinal root reclassification             = 0
root semantic consumer / Builder effect            = 0
fallback/retry edge added                          = 0
```

Focused verification on 2026-08-23:

```text
cargo check --profile quick                         = pass (baseline warnings only)
cargo test --profile quick parser::callable_parameter_source
  passed                                             = 41
  known baseline failure                             = 1
  current-change failure                             = 0
frontend_main_app_entry_i0_guard.sh                 = pass
frontend_main_app_entry_transport_i0_guard.sh       = pass
current_state_pointer_guard.sh                      = pass
git diff --check                                    = pass
```

The one failing test is the pre-existing
`unchanged_parser_scan_loop_box_has_four_methods_and_fifteen_rows` assertion
(`Some("i64")` versus `None`); it is classified as known baseline debt and is
not caused by this transport slice. No root consumer, root lowering, A/C
meaning, fallback, or production switch was opened by I0.

The reusable acceptance guard is
`tools/checks/frontend_main_app_entry_transport_i0_guard.sh`. It must prove
one root co-seal issuer, one unified root field through retained/Prepared/
VerifiedFinal source, A-only Script rows, explicit A discard with AppReady
reject, no root reclassification/fallback, and the 760-line boundary.
