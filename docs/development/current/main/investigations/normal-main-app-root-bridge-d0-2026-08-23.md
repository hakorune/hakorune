Status: Closed as NoSafeSlice — root-source disposition is missing
Date: 2026-08-23
Decision: NORMAL-MAIN-APP-ROOT-BRIDGE-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-main-app-consumer-d0-2026-08-23.md
PrerequisiteExecutionRow: NORMAL-MAIN-APP-CONSUMER-D0
ProductionCaller: 0; design only
ProductionEdit: none until the finite root mapping is accepted
CeremonyTier: D0 — source disposition to root admission bridge
---

# NORMAL-MAIN-APP-ROOT-BRIDGE-D0

CurrentExecutionRow: NORMAL-MAIN-APP-ROOT-BRIDGE-D0

## Six-line brief

```text
Decision:
  design one named root-admission bridge owned by the existing normal root
  lifecycle; do not let raw expansion or a bool consume the parser fact.
Source authority + canonical issuer:
  issue_parser_main_app_entry_v1 remains the sole Main/App source issuer; the
  bridge consumer issuer is not accepted until its exhaustive mapping is fixed.
Non-authority:
  VerifiedRawRootExpansionV1::from_program as a Main classifier,
  root_is_app_mode, NormalCompileRequest, AST/name/ordinal re-observation,
  compatibility entry selection, and Builder physical state.
Fail-fast boundary:
  consume the move-only parser disposition before root/catalog Builder effects;
  unavailable/incomplete/integrity-invalid evidence must terminate typed.
Smallest next slice:
  fix the finite mapping from every parser disposition/reason to App root,
  Script root, or typed terminal, and define how existing structural expansion
  data is verified without reclassifying Main; no code or receipt yet.
Non-claims:
  ABI/result semantics, root body lowering, child scheduling, MIR/ValueId,
  publication, compatibility retirement, production switch, and performance.
```

## Why this design stop exists

The live root lifecycle currently calls
`VerifiedRawRootExpansionV1::from_program(source_ast)` before and after the
semantic package installation, then derives `is_app_mode()` and writes the
legacy `root_is_app_mode` state. That is a second Main/App observation after
the parser has already issued `ParserMainAppEntryDispositionV1`.

The existing lifecycle is still the only plausible named root orchestrator,
but it is not yet a consumer. This card must decide whether it can consume the
parser disposition by move and use raw expansion only as a structural body/
child projection, or whether a missing authority makes the slice `NoSafeSlice`.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `issue_parser_main_app_entry_v1` | exact parser Main/App disposition | root semantic/physical effects |
| `VerifiedFinalCallableProgramSourceV1` | move-only source transport | root classification |
| `NormalDefaultRootCatalogLifecycle` | candidate one-shot root consumer | parser re-observation or raw-bool authority |
| future `NormalMainAppRootAdmissionIssuerV1` | exhaustive disposition-to-root mapping | AST/name-based repair |
| `VerifiedRawRootExpansionV1` | structural root/body/child projection after admission | Main/App selection from AST |
| `NormalCompileRequestV1` | existing request transport | Main/App semantic issuance |
| `root_is_app_mode` | legacy physical mirror if retained | source authority |
| `ModuleBuilderInvocationSessionV1` | unpublished Builder scope/effects | parser/source classification |

`NormalMainAppRootAdmissionIssuerV1` is a design name only. Do not create it
until the finite mapping and its named consumer are accepted.

## Finite mapping that must be fixed

The parser disposition has five top-level states. `Outside` contains several
different source reasons and may not be collapsed into one Script fallback.

| Parser state | Required bridge decision | Pre-effect rule | Fallback |
| --- | --- | --- | --- |
| `AppMainReady(seal)` | exact App-root admission, or explicit stop | no effects before co-seal | none |
| `Outside(ProgramCohort)` | decide whether this source belongs to Script root | no guessed Script promotion | none |
| `Outside(StaticParent(reason))` | decide per preserved parent reason; no reason erasure | typed stop until mapped | none |
| `Outside(NonMainStaticBox)` | decide whether non-Main static source is Script-owned | no Main re-selection | none |
| `Outside(NonMainMethod)` | decide missing-main terminal versus another source owner | no raw expansion repair | none |
| `Outside(NonZeroMainArity)` | decide explicit unsupported terminal versus separate policy | no arity-blind App route | none |
| `SourceAuthorityUnavailable(reason)` | typed terminal | zero root/Builder effects | none |
| `Incomplete(reason)` | typed terminal | zero root/Builder effects | none |
| `IntegrityInvalid(reason)` | typed terminal | zero root/Builder effects | none |
| compatibility lane | retain existing explicit compatibility owner | separate lane | no synthetic parser state |
| `MappingUnresolved` | design state only | remain NoSafeSlice | none |

The nested `StaticParent` reason vocabulary remains intact. The bridge may not
turn it into `Outside`/`Script` by default merely because a raw expansion can
be constructed.

## Candidate boundary

The intended one-way shape, subject to this Decision, is:

```text
VerifiedFinalCallableProgramSourceV1
  -> PreparedNormalDefaultProgramRootV1 transport
  -> NormalDefaultRootCatalogLifecycle::consume_main_app_entry_once
  -> NormalMainAppRootAdmissionV1       (only if issuer is accepted)
  -> structural root/body/child projection
  -> existing unpublished root lowering
```

The bridge must not call `VerifiedRawRootExpansionV1::from_program` as the
source of App/Script meaning. If structural AST projection needs a new
parser/source relation, that relation is a separate `NoSafeSlice`; do not
hide it in an adapter.

## NoSafeSlice conditions

Remain at design stop if any condition holds:

```text
one parser state/reason has no explicit App/Script/terminal mapping
Outside reasons can only be handled by a raw AST reclassification
root body/child structural data has no authority after Main classification is removed
the lifecycle cannot consume the disposition exactly once before effects
root_is_app_mode must remain the acceptance authority
NormalCompileRequest must be changed merely to carry a duplicated field
compatibility fallback is required for a parser non-ready state
a new semantic receipt has no named consumer and retirement edge
the bridge would require a second resolver/source scan
any touched production file reaches the 760-line split trigger
```

## Design acceptance

This card may authorize a successor I0 only when the evidence packet contains:

```text
one named bridge consumer and one issuer
exhaustive top-level and nested Outside mapping
typed terminal chronology before Builder effects
structural expansion relation that is not a classifier
root_is_app_mode no longer acts as source authority
no parser disposition re-observation or raw-bool reconstruction
no fallback/retry/reselection edge
positive/negative fixture plan and reusable guard
```

Until then, implementation, fixtures, new semantic `Verified*`/`Prepared*`
products, request changes, root writes, and production switching are forbidden.

## Worker and source audit decision

Three read-only audits and the direct source inspection agree on the following
boundary:

```text
AppMainReady(seal)
  -> may become an App-root candidate after an exact structural co-seal

Outside(any reason)
  -> typed Main/App outside terminal
  -> never Script by default

SourceAuthorityUnavailable / Incomplete / IntegrityInvalid
  -> typed source terminal
```

The existing `ParserMainAppEntryDispositionV1` does not contain a positive
Script-root witness. The repository has a separate parser-owned
`CanonicalScriptCohortAdmissionV1` / `CanonicalScriptSourceRowsV1` pair for
the pure Script cohort, but that pair is not transported through
`VerifiedFinalCallableProgramSourceV1` into the normal root lifecycle. The
root bridge therefore cannot safely decide Script by taking the negation of
Main/App, by inspecting `VerifiedRawRootExpansionV1`, or by reading the raw
`root_is_app_mode` boolean.

This closes the bridge D0 as a design finding rather than authorizing an I0:

```text
Missing authority:
  one parser-owned, same-invocation total root-source disposition that
  co-seals the positive App witness, the positive Script witness when one
  exists, and typed terminal reasons for every other state.

Forbidden repair:
  Outside -> Script fallback, AST reclassification, raw bool reconstruction,
  name/ordinal pairing, compatibility retry, or a second parser scan.
```

The successor design task is recorded in
`normal-main-app-root-source-disposition-d0-2026-08-23.md`. No Rust code,
fixture, request field, root write, or new semantic receipt was added by this
audit.
