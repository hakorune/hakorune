Status: Design stop — source-root representation missing
Date: 2026-08-23
Decision: NORMAL-MAIN-APP-ROOT-SOURCE-DISPOSITION-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-root-bridge-d0-2026-08-23.md
ProductionCaller: 0; design only
ProductionEdit: none until one total source-root disposition is accepted
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
  decide the exact positive Script witness and the exhaustive App/Script/
  terminal mapping; no implementation or new receipt until that decision.
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

## Candidate source authority and issuer

The intended authority split is:

| Owner | Owns | Must not own |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | exact App `Main.main/0` source relation | Script admission or root effects |
| `issue_canonical_script_cohort` plus source-row issuer | positive pure-Script cohort and exact Script rows | App selection or Builder effects |
| future `ParserNormalRootSourceDispositionIssuerV1` | one same-invocation co-seal of the existing source facts | AST re-observation, semantic root lowering, MIR |
| future normal root bridge | consume the moved source disposition once | parser classification or raw-bool repair |
| `VerifiedRawRootExpansionV1` | structural projection after explicit admission | App/Script selection |

The future issuer is a design name only. It must not be introduced until the
mapping and move shape below are accepted.

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
D0-A  Source witness census
      Prove whether CanonicalScriptCohortAdmissionV1 plus HandoffReady rows
      can be moved into the normal callable source without a second owner or
      breaking the existing Script A handoff.

D0-B  Total mapping contract
      Enumerate every Main/App and Script disposition, including all nested
      Outside and source-row states, and assign AppRootReady, ScriptRootReady,
      or one typed terminal. No default arm.

D0-C  Cross-module move contract
      Choose the single parser-owned transport shape and visibility. Preserve
      non-Clone ownership, same parser brand, and exactly-once consumption.

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

Until this card reaches an accepted Decision, implementation, fixtures,
request changes, root writes, fallback/retry, and production switching remain
forbidden.
