---
Status: Active design stop
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-PARSER-INPUT-HANDOFF-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-issuer-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: parser-owned AST-free source input handoff for canonical A
Classification: BoxCount (design only; implementation remains closed)
Prerequisite: docs/development/current/main/investigations/script-direct-static-call-canonical-script-source-admission-p0-2026-08-21.md
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-PARSER-INPUT-HANDOFF-D0

This handoff D0 is parked behind the explicit pure-Script source admission
P0. `NoBoxDeclarations` is still a compatibility cohort; this card may not
reinterpret it as a source-backed empty Script window. Only the typed
`CanonicalScriptCohortAdmitted` admission from the prerequisite may enter the
handoff design below. The handoff then co-seals canonical identity/profile/
read-parse receipt and is the first phase that can issue `HandoffReady`.

## Six-line brief

Decision: Define one parser-owned, AST-free handoff that supplies canonical
Script A with complete source coverage and source declaration/config views.
This D0 does not issue resolver, target, Recipe, Join, physical, or
production meaning.

Source authority + canonical issuer: the already-sealed
`CanonicalParserSourceHandoffV1` / `CompletedParserPostpassV1` and the same
front-door source/config receipt are authoritative. A new parser sibling,
`CanonicalScriptSourceInputHandoffV1`, is the only issuer of this parser
input product; it is not a direct-static disposition issuer.

Non-authority: `ASTNode` rescans, `NormalSourcePlanClassifierV1`,
`CanonicalCoreSourcePlanCompileRequestV1`, `MirBuilder`/`comp_ctx`, selected
Builder declaration facts, resolver/target inventories, `RawScriptBodyRecipeV1`,
pointer/path/name/ordinal/digest joins, `ValueId`, `MirType`, and compatibility
success cannot issue or complete this handoff.

Fail-fast boundary: after the parser cohort admission and front-door identity
I0 have issued `CanonicalScriptCohortAdmitted` plus `CanonicalSourceBacked`,
before canonical `prepare_script_recipe()`, resolver
forest construction, Builder install, or child effects. Missing source
coverage, declaration/import/Brand snapshot, or identity/cohort agreement
stops before A can observe the source.

Smallest next slice: docs-only parser handoff design. Fix the owned rows,
coverage proof, lifetime, finite states, A boundary, and sibling placement.
No parser code, source admission, AST rewrite, carrier, fallback, physical
consumer, or production switch is authorized by this D0.

Non-claims: no grammar change, no new direct-static acceptance, no resolver
forest or target/result semantics, no Recipe/Join/publication/Return, no
compatibility/raw retirement, no ABI/backend/performance, and no selected
normal cutover.

## Current gap and owner split

The current parser-backed path exposes:

```text
CanonicalParserSourceHandoffV1
  -> NormalParserCallableSourceHandoffV1
  -> SealedNormalScriptSourceV1
  -> canonical_core_dispatch::compile_script
```

`NormalParserCallableSourceHandoffV1` currently carries parser disposition,
lineage, AST access, and `CompletedParserPostpassV1`. The postpass retains
parser-private box coverage, but the normal source plan does not transport a
canonical Script `ProgramBody` window or a stable AST-free declaration/config
view into A. `CanonicalCoreSourcePlanCompileRequestV1` is only plan/admission/
receipt transport. Neither may silently become the missing issuer.

The parser handoff owns syntax-derived source coverage and declaration/config
facts only. The future A issuer consumes this handoff and separately issues
the resolver forest, direct-static target/result rows, required proof, and
terminal relation. This prevents parser and A from issuing the same semantic
meaning twice.

The handoff must live in a new parser/compiler sibling below the 760/800-line
limits. Do not grow `canonical_core_dispatch.rs` (748 lines),
`normal_default_root_catalog_lifecycle.rs` (719 lines), or the existing
postpass parent merely to hide a second authority. Parser-private coverage
may be projected once into the owned handoff; it may not be reconstructed by
AST scan or Builder key sorting.

## Handoff payload boundary

The future handoff may own only source-backed, AST-free data:

```text
CanonicalSourceBacked identity + parser profile/read-parse receipt
complete retained Script ProgramBody statement window + coverage rows
source-order declaration facts needed by Script semantic resolution
Brand declaration catalog with declaration identity and underlying type
canonical import/config snapshot with exact source provenance
cohort/build-gate disposition and parser source lineage
```

It must not own or infer:

```text
resolver forest, target/result catalog, Recipe/Join, required proof
ValueId, MIR block, physical instruction, Builder ordinal, AST pointer
direct-static disposition, performance, or fallback permission
```

The declaration/Brand/import/config rows here are parser-owned syntax and
source-coordinate views only (`DeclarationSyntax`, `BrandSyntax`, and
`ImportConfigSyntax`). They are not the resolved semantic forest or target
meaning. A consumes this handoff once and issues the resolved forest,
direct-static target/result rows, required proof, and terminal relation; A
does not reissue parser identity or re-scan the AST to recreate these rows.
For the current canonical normal-file profile, a non-empty `Using`/`Import`
surface is rejected upstream by the explicit no-import profile; the admitted
handoff therefore carries an explicit empty import/config proof. A future
import-capable cohort requires a separate source-profile admission row and may
not widen this handoff by default.

The handoff is non-Clone and move-only across the canonical source-plan
boundary. A temporary borrow of parser-owned postpass data is allowed during
one issuance, but the retained rows must carry one source identity and exact
coverage. `FunctionOwnerIdV1` is not a source key; names, paths, ordinals,
filenames, and digest equality cannot re-pair rows later.

## Phase boundary and transition ownership

The parser handoff and A are adjacent phases, not two names for one issuer.
The transition is finite and one-way:

| phase | owned states | sole issuer / consumer | allowed transition |
|---|---|---|---|
| cohort admission | `CanonicalScriptCohortAdmitted`, `CohortUnresolved`, `CompatibilitySource`, `Deferred` | parser-only cohort issuer | only `CanonicalScriptCohortAdmitted` may reach front-door identity co-seal |
| identity-I0 | `CanonicalSourceBacked`, `NotApplicable`, `CompatibilitySource`, `Deferred` | parser frontdoor and typed source disposition | only `CanonicalSourceBacked` plus the cohort admission may enter parser-input observation |
| parser-input | `SourceAuthorityUnavailable`, `ObservationIncomplete`, `HandoffReady`, `HandoffConsumed`, `IntegrityInvalid` | `CanonicalScriptSourceInputHandoffV1` | `HandoffReady` moves once to `HandoffConsumed`; the other rows stop or remain in their typed owner |
| A observation | `NonCandidate`, private `InputAuthorityReady`, later `DirectStaticSourceReady` | future `CanonicalScriptDirectStaticSourceOnlyIssuerV1` | A may issue semantic resolver/target/result/proof/terminal meaning exactly once |
| C/B future | typed disposition, `DispositionTransported` | future canonical disposition and transport owners | C consumes A once; B transports the typed result only |

`CanonicalScriptCohortAdmitted` is a parser cohort state, not identity or
parser-input readiness. `CanonicalSourceBacked` is an upstream identity state,
not parser-input readiness, and it cannot enter this handoff without the
cohort admission in the same co-seal. `HandoffReady` is the only public parser
product and never means that a direct-static candidate exists. `NonCandidate`
is A's complete, integrity-clean zero-candidate result and cannot be issued by
the parser handoff. `HandoffConsumed`, `AInputTransported`, and
`DispositionTransported` are later lifecycle states and cannot be returned to
the parser or used to reissue source meaning. A missing/partial parser row is
therefore never converted into `NonCandidate`, compatibility success, or an
empty A input.

## Exhaustive handoff state table

| state | phase | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|---|
| `CanonicalScriptCohortAdmitted` | upstream cohort | parser-only `CanonicalScriptCohortAdmissionV1` | retain typed cohort; do not observe rows or effects | must co-seal with `CanonicalSourceBacked` before parser-input observation | never enter from `NoBoxDeclarations`, empty catalog, or AST shape |
| `CohortUnresolved` | upstream cohort | parser shape table | typed stop before identity/handoff rows and effects | `NoSafeSlice` or separately named compatibility owner | never become `CanonicalSourceBacked`, empty, or `NonCandidate` |
| `CanonicalSourceBacked` | upstream | `CanonicalParserSourceHandoffV1` issues identity-I0 | pass exact source receipt only when paired with `CanonicalScriptCohortAdmitted`; no A/physical effect | handoff observation may begin after co-seal | identity alone cannot enter; never reissue from AST or default profile |
| `NotApplicable` | parser ingress | parser/frontdoor proves non-Script or outside canonical source cohort | no Script rows are observed | caller-owned family dispatch | never fabricate an empty canonical handoff |
| `CompatibilitySource` | parser ingress | postpass/parser disposition marks compatibility | preserve typed compatibility origin; no canonical rows | compatibility owner or parked stop | never become source-backed or A success |
| `Deferred` | upstream admission | source/resolver admission marks Deferred before handoff | preserve reason; no partial handoff | deferred owner or `NoSafeSlice` | never become empty coverage or raw success |
| `SourceAuthorityUnavailable` | handoff preflight | identity, parser receipt, source cohort, or config snapshot is absent/foreign | typed stop before row issuance | `NoSafeSlice` until authority exists | no AST rescan, default config, or Builder fallback |
| `ObservationIncomplete` | handoff observation | authority exists but window/coverage/declaration/config rows cannot be totalized once | typed stop before A/resolver/Recipe effects | `NoSafeSlice` until coverage is total | never round to `NonCandidate` or empty handoff |
| `HandoffReady` | handoff terminal | one parser sibling co-seals every required source row, identity, and coverage proof | issue one move-only parser handoff; no semantic A disposition | one A consumer moves it to `HandoffConsumed` | no clone, second parser pass, or name re-pairing |
| `HandoffConsumed` | A ingress | the named A consumer takes `HandoffReady` once | no parser replay or second source interpretation | A observation begins or ends in its own state | no return to parser, compatibility, or raw route |
| `IntegrityInvalid` | handoff verification | complete observation finds duplicate, foreign, stale, cohort, cardinality, or source drift | typed reject before A/resolver/child effects | terminal source candidate discard | no retry, repair-by-AST, compatibility, or raw fallback |
| `NonCandidate` | A-only continuation | A later proves complete direct-static observation has zero candidates | no direct-static package; parser handoff remains source input | canonical non-direct-static owner | parser never issues this state or treats missing rows as it |
| `AInputTransported` | future A transport | future A input owner moves the consumed handoff once | no replay or second source interpretation | detached A input terminal | no clone, return to parser, or raw path |
| `DispositionTransported` | future C/B | future B moves a C disposition once | no source re-observation | detached physical consumer terminal | never reuse as parser or A state |

`HandoffReady` is the only public parser product. `HandoffConsumed` belongs to
the named A consumer; `AInputTransported` and `DispositionTransported` are
future lifecycle states, not parser issuers. `NonCandidate` belongs to the
later A observation, not this issuer. A complete zero direct-static candidate
is not known at this layer; empty or partial parser coverage is never evidence
of `NonCandidate`.

## Acceptance for this design stop

- one parser sibling and one parser-backed source authority are fixed;
- `CanonicalSourceBacked` is consumed from identity-I0 and not reissued;
- the handoff owns complete Script window coverage, declaration/Brand facts,
  canonical import/config snapshot, and source lineage under one identity;
- resolver, target/result, Recipe/Join, proof, terminal, physical, and
  performance meaning remain with future A/C/B owners;
- all ten phase-qualified states have one owner, pre-effect behavior,
  continuation, and fallback policy;
- `HandoffReady` is non-Clone and contains no AST/pointer/ValueId/MIR/Builder
  physical fact or Recipe key;
- duplicate/foreign/missing/partial/cohort-drift rows reject before A;
- no code, fixture, grammar/source admission, fallback, physical consumer,
  production switch, or performance run is opened by this D0.

## NoSafeSlice conditions

Remain at this D0 if any condition holds:

1. parser coverage can only be recovered by AST scan or Builder reconstruction;
2. declaration/Brand/import/config facts have no parser/source-backed issuer;
3. the handoff must issue resolver/target/Recipe/Join meaning itself;
4. source rows can only be paired by pointer, path, name, ordinal, filename,
   or digest equality;
5. compatibility/deferred/partial coverage is rounded to empty or ready;
6. `HandoffReady` can be cloned, replayed, or reissued by canonical planning;
7. implementation requires semantic growth in the 748/719-line owners or
   crosses the 760/800-line limits;
8. a missing parser row would be repaired by raw fallback, source admission
   expansion, or production switching rather than a named issuer.

Until these are closed, A issuer implementation remains unavailable and no
canonical physical or production claim is allowed.
