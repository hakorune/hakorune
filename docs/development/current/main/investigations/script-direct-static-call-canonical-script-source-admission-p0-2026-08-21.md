---
Status: Active design stop
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SCRIPT-SOURCE-ADMISSION-P0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-issuer-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: explicit parser-backed pure Script source admission before the parser input handoff
Classification: BoxCount (design only; implementation remains closed)
NextCard: docs/development/current/main/investigations/script-direct-static-call-canonical-source-parser-input-handoff-d0-2026-08-21.md
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SCRIPT-SOURCE-ADMISSION-P0

## Six-line brief

Decision: Define one explicit canonical pure-Script source-backed cohort before
the parser input handoff. `NoBoxDeclarations` is not promoted by a boolean or
by the absence of Box nodes. This P0 fixes admission states only; it issues no
semantic A package and changes no source grammar.

Source authority + canonical issuer: the one parser invocation's total
`CompletedParserPostpassV1`, its `ParserCallableParameterSourceCatalogV1`,
and the front-door `CanonicalParserSourceHandoffV1` profile/read-parse receipt
are authoritative. A parser-only sibling
`CanonicalScriptSourceAdmissionV1` may later issue the typed pure-Script
admission; it is not an A disposition or a direct-static target issuer.

Non-authority: `CompletedParserPostpassV1::is_source_backed()`, AST “no boxes”
inference, current `NoBoxDeclarations` compatibility, empty catalogs,
`NormalSourcePlanClassifierV1`, Builder/`comp_ctx`, AST/pointer/name/ordinal/
digest joins, resolver/target inventories, and successful compatibility/raw
lowering cannot issue canonical Script admission.

Fail-fast boundary: after
`parse_with_callable_parameter_source()` has produced one parser product and
the canonical identity/profile/receipt is validated, but before
`into_source_disposition()`, source-plan classification, Recipe/resolver
observation, Builder effects, or child descent. Missing or mixed Script rows
stop before any semantic effect.

Smallest next slice: this docs-only P0. Fix the pure-Script cohort predicate,
complete source-row requirements, finite states, and the handoff dependency.
The following implementation row is
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SCRIPT-PARSER-INPUT-HANDOFF-I0`.

Non-claims: no parser grammar change, AST rewrite, resolver forest,
direct-static target/result, required proof, Recipe/Join, physical Call,
publication/Return, compatibility/raw retirement, fallback, production switch,
ABI/backend, or performance claim.

## Why this predecessor is required

The current parser seam is:

```text
parse_with_callable_parameter_source()
  -> parse_postpass_s0()
  -> finish_callable_parameter_source_for_normal()
  -> ParsedProgramWithCallableParameterSourceV1::new()
  -> into_source_disposition()
```

`OpenParserPostpassProductV1::finalize` and the ordinary Box source seal are
not a pure-Script issuer. The postpass currently classifies a Program with no
Box declarations as `NoBoxDeclarations`, and
`CompletedParserPostpassV1::from_initial_compatibility` may still retain an
`Initial` program while returning a compatibility cohort. Therefore
`is_source_backed()` is an implementation detail of one internal variant, not
the canonical Script admission authority.

The parser handoff D0 is parked behind this P0. It may consume only a typed
pure-Script admission; it must not reinterpret `NoBoxDeclarations` as an
empty source window. A complete parser handoff needs one retained ProgramBody
window, source-order declaration/Brand/import/config syntax rows, and the same
front-door identity. A later A issuer will own resolver forest, target/result,
proof, and terminal meaning.

## Pure-Script admission rule

The later implementation must issue `CanonicalScriptSourceAdmissionV1` only
when all of these facts come from the same parser invocation and selected
front-door configuration:

```text
canonical source identity/profile/read-parse receipt is valid
ProgramBody is the retained top-level source window
no BoxDeclaration, interface, record, static Box, or BuildGate remains in the
  selected/pruned source cohort
all retained top-level declarations, Brand syntax rows, and import/config rows
  are complete in source order and tied to the same parser product
callable parameter source is Complete, not SelectedBuildGateUnsupported
the cohort is explicitly PureScript, not inferred from an empty Box list
```

The predicate is a source-cohort decision, not a direct-static candidate
decision. A pure Script program with zero direct-static calls is still a valid
admission input; A later owns `NonCandidate`. Conversely, a missing declaration
row or an AST-only compatibility product is not a zero-row pure Script input.

`from_initial_compatibility` must remain distinguishable from a true
source-backed product. The implementation must inspect a typed cohort/source
variant, not call `is_source_backed()` and not use `unwrap_or(false)` or an
empty catalog as the missing branch.

## Exhaustive admission state table

| state | issuer / authority | before effects | terminal / continuation | fallback |
|---|---|---|---|---|
| `NotApplicable` | front-door profile/window proves the request is outside canonical pure Script scope | no Script admission or semantic observation | caller-owned non-Script lane | never fabricate an empty Script handoff |
| `CompatibilitySource` | parser postpass/cohort explicitly issues `InterfaceBox`, `StaticBox`, `RecordBox`, `MixedProgram`, `TopLevelBuildGate`, `NoBoxDeclarations`, or `NonProgram` compatibility | retain typed compatibility origin; no canonical Script rows | existing compatibility owner or parked stop | never promote by `NoBoxDeclarations`, success, or AST shape |
| `Deferred` | upstream source/resolver admission explicitly defers the request | preserve reason; no parser input or A effect | deferred owner or `NoSafeSlice` | never become pure Script, empty, or raw success |
| `SourceAuthorityUnavailable` | canonical identity, parser receipt, profile, or same-invocation source product is absent/foreign | freeze before source rows, Recipe, resolver, Builder, or child effects | `NoSafeSlice` until the named issuer exists | no AST rescan, default profile, or Builder reconstruction |
| `ObservationIncomplete` | a typed candidate cohort exists but ProgramBody/declaration/Brand/import/config rows are not total | freeze before parser handoff publication and semantic observation | `NoSafeSlice` until coverage is complete | never round to pure Script, `NonCandidate`, or empty catalogs |
| `CanonicalScriptSourceBacked` | the future parser-only `CanonicalScriptSourceAdmissionV1` co-seals the complete pure-Script cohort with one identity and source-row coverage | issue one non-Clone admission; no A/Recipe/physical effect | parser input handoff D0 consumes once | no clone, reparse, AST/name pairing, or compatibility fallback |
| `IntegrityInvalid` | complete observation finds duplicate, foreign, stale, mixed-cohort, cardinality, or source/config drift | typed reject before parser handoff, resolver, Builder, and child effects | terminal source candidate discard | no retry, repair-by-AST, compatibility, or raw fallback |
| `NonCandidate` (A-only) | future A issuer completes semantic observation and proves zero direct-static candidates | no direct-static package or physical effect | canonical non-direct-static owner | this P0/parser issuer never emits it |
| `Transported` (future C/B) | future parser-handoff consumer moves the typed admission/semantic result once | no replay or second source interpretation | detached consumer terminal | this P0 never emits or reuses it |

`CanonicalScriptSourceBacked` is the only public pure-Script admission from
this row. `NoBoxDeclarations` remains `CompatibilitySource` until the new
cohort issuer proves all pure-Script rows. `NonCandidate` belongs to A, and
`Transported` belongs to a future consumer; neither may be used to hide a
missing parser row. Every routing arm must match this table exhaustively.

## Ownership and phase boundary

```text
parser identity / cohort admission
  -> CanonicalScriptSourceBacked
parser input handoff D0
  -> complete ProgramBody + syntax/config rows
A observation D0
  -> resolver forest + target/result + proof + terminal
C/B future
  -> typed direct-static disposition and transport
```

The parser owns syntax and source coordinates only. It must not issue a
resolved target, representation eligibility, direct-static disposition, or
physical destination. A consumes the parser admission once and issues those
semantic rows; it must not re-observe the AST or reissue parser identity.

The admission product is non-Clone and AST-free at its public boundary. It
may borrow parser-owned data during one issuance, but it must not retain AST
pointers, Builder ordinals, `ValueId`, MIR blocks, Recipe keys, or physical
IDs. The next parser handoff D0 is the named consumer; no other consumer is
opened by this P0.

## Acceptance for this design stop

- one explicit pure-Script cohort state is named; current compatibility
  cohorts, including `NoBoxDeclarations`, remain unchanged;
- the issuer consumes the same parser identity/profile/read-parse receipt and
  same-invocation postpass/parameter-source product exactly once;
- complete ProgramBody, declaration, Brand, import/config, and cohort coverage
  is required before `CanonicalScriptSourceBacked` can be issued;
- a pure Script program with zero direct-static calls is admitted here and is
  later classified by A as `NonCandidate`, while missing rows remain
  `ObservationIncomplete`;
- compatibility, deferred, source-loss, and integrity-invalid rows are not
  converted into empty catalogs, source-backed success, or raw fallback;
- the parser input handoff D0 is the only next consumer and receives a typed,
  non-Clone admission without semantic A meaning;
- no code, fixture, grammar, semantic receipt, fallback, physical consumer,
  production switch, or performance run is opened by this P0.

## NoSafeSlice conditions

Remain at this design stop if any condition holds:

1. `is_source_backed()` or absence of Box nodes is the only admission test;
2. `NoBoxDeclarations` is silently promoted or represented as empty success;
3. a mixed/static/record/interface/BuildGate/AST-only source can enter the pure
   Script cohort without an explicit typed rule;
4. declaration, Brand, import/config, or ProgramBody rows can only be rebuilt
   by AST scan, pointer/name/ordinal/digest pairing, or Builder state;
5. a missing row is rounded to `NonCandidate`, `CompatibilitySource`, or
   `CanonicalScriptSourceBacked`;
6. the parser admission issues resolver/target/result/proof/terminal meaning;
7. the parser handoff or A issuer is implemented before this finite table is
   accepted and guarded;
8. the new source product crosses the 760-line split trigger or 800-line hard
   stop, or requires a second parser invocation/compatibility fallback.

Until these are closed, the parser input handoff, A issuer, Recipe/Join,
physical bridge, canonical production consumer, and old-route retirement stay
parked. This is a development `NoSafeSlice`, not a compiler disposition.
