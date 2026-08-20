---
Status: Accepted design stop — parser input handoff D0 selected
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

Decision: Define one explicit pure-Script cohort admission before the parser
input handoff. `NoBoxDeclarations` is not promoted by a boolean or by the
absence of Box nodes. This P0 fixes cohort states only; it issues no canonical
identity, parser window, semantic A package, or source grammar change.

Source authority + canonical issuer: the one parser invocation's typed
`CompletedParserPostpassV1` cohort, its
`ParserCallableParameterSourceDispositionV1::Complete`, and the same parser
brand are authoritative. A parser-only sibling
`CanonicalScriptCohortAdmissionV1` may later issue the typed cohort admission.
The front-door `CanonicalParserSourceHandoffV1` profile/read-parse receipt is
co-sealed at the following parser-input boundary, not reissued by this P0.
Neither product is an A disposition or a direct-static target issuer.

Non-authority: `CompletedParserPostpassV1::is_source_backed()`, AST “no boxes”
inference, current `NoBoxDeclarations` compatibility, empty catalogs,
`NormalSourcePlanClassifierV1`, Builder/`comp_ctx`, AST/pointer/name/ordinal/
digest joins, resolver/target inventories, and successful compatibility/raw
lowering cannot issue canonical Script admission.

Fail-fast boundary: immediately after
`parse_postpass_s0()` and
`finish_callable_parameter_source_for_normal()` complete for the same parser
invocation, and before `into_source_disposition()`, source-plan
classification, Recipe/resolver observation, Builder effects, or child
descent. Missing/foreign cohort or parameter authority stops here; front-door
identity/profile/receipt validation belongs to the following parser-input
handoff.

Smallest next slice: this docs-only P0. Fix the pure-Script cohort predicate,
the exhaustive top-level shape table, finite states, and the handoff
dependency. The handoff itself owns later window/declaration/config rows.
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
the cohort admission authority. The parser-only cohort issuer must run before
the front-door profile/receipt co-seal; the latter belongs to the parser input
handoff.

The parser handoff D0 is parked behind this P0. It may consume only a typed
`CanonicalScriptCohortAdmitted` value; it must not reinterpret
`NoBoxDeclarations` as an empty source window. The handoff then co-seals the
front-door identity/profile/read-parse receipt and issues the retained
ProgramBody window plus source-order declaration/Brand/import/config syntax
rows. A later A issuer owns resolver forest, target/result, proof, and
terminal meaning.

## Pure-Script admission rule

The later implementation must issue `CanonicalScriptCohortAdmissionV1` only
when all of these facts come from the same parser invocation:

```text
the parser postpass cohort is explicitly PureScript by the exhaustive shape
  table below, not inferred from an empty Box list
callable parameter source is Complete, not SelectedBuildGateUnsupported
the parser brand and cohort are internally consistent and one-shot
```

The predicate is a source-cohort decision, not a direct-static candidate
decision. A pure Script program with zero direct-static calls is still a valid
cohort admission input; A later owns `NonCandidate`. Conversely, a missing
parser row belongs to the later handoff's `ObservationIncomplete`, not to a
zero-row cohort admission.

`from_initial_compatibility` must remain distinguishable from a true
source-backed product. The implementation must inspect a typed cohort/source
variant, not call `is_source_backed()` and not use `unwrap_or(false)` or an
empty catalog as the missing branch.

## Exhaustive top-level shape table

`classify_program()` currently ignores many non-Box top-level variants. The
P0 implementation must replace that implicit `_ => {}` behavior with one
finite source-owned table. The table below is a design contract; a shape not
listed here is `CohortUnresolved`, never pure Script by default.

| top-level shape | cohort disposition | requirement / next owner |
|---|---|---|
| `FunctionDeclaration` and ordinary executable Script items | `PureScriptSyntax` | allowed in the cohort; parser handoff later issues body/coordinate rows |
| `UsingStatement` / `ImportStatement` | `PureScriptSyntax` | allowed only with explicit syntax/config rows in parser handoff; no name inference |
| `BrandDeclaration` / `TypeAliasDeclaration` | `PureScriptSyntax` | allowed only with explicit declaration syntax rows; A later owns resolved meaning |
| `EnumDeclaration` / `GlobalVar` / `StaticConstTable` | `PureScriptSyntax` | allowed only with explicit syntax/coverage rows; no empty-catalog shortcut |
| `BoxDeclaration` (ordinary, static, record, interface, sync, or any Box) | `CompatibilitySource` | existing Box/compatibility owner; never pure Script |
| `BuildGate` remaining after pruning | `CompatibilitySource` or `Deferred` | selected gate must be fully classified; no pure admission while it remains |
| nested `Program`, unknown/future AST item, or unsupported top-level shape | `CohortUnresolved` | typed stop/`NoSafeSlice`; no empty source handoff or raw fallback |
| non-`Program` root | `NotApplicable` | caller-owned non-Script lane |

`PureScriptSyntax` is only a cohort label. It does not prove complete
ProgramBody, declaration, Brand, import/config, or source identity coverage;
those rows are issued by the parser input handoff D0. `Using`/`Import` and
the declaration variants remain explicitly listed so future parser changes
cannot silently widen the cohort.

## Exhaustive admission state table

| state | issuer / authority | before effects | terminal / continuation | fallback |
|---|---|---|---|---|
| `NotApplicable` | front-door profile/window proves the request is outside canonical pure Script scope | no Script admission or semantic observation | caller-owned non-Script lane | never fabricate an empty Script handoff |
| `CompatibilitySource` | parser postpass/cohort explicitly issues `InterfaceBox`, `StaticBox`, `RecordBox`, `MixedProgram`, `TopLevelBuildGate`, `NoBoxDeclarations`, or `NonProgram` compatibility | retain typed compatibility origin; no canonical Script rows | existing compatibility owner or parked stop | never promote by `NoBoxDeclarations`, success, or AST shape |
| `Deferred` | upstream source/resolver admission explicitly defers the request | preserve reason; no parser input or A effect | deferred owner or `NoSafeSlice` | never become pure Script, empty, or raw success |
| `SourceAuthorityUnavailable` | same-invocation postpass, parser brand, or `ParserCallableParameterSourceDispositionV1::Complete` is absent/foreign (including `SelectedBuildGateUnsupported`) | freeze before source rows, Recipe, resolver, Builder, or child effects | `NoSafeSlice` until the named issuer exists | front-door identity is not guessed here; no AST rescan, default profile, or Builder reconstruction |
| `ObservationIncomplete` | a typed candidate cohort exists but ProgramBody/declaration/Brand/import/config rows are not total | freeze before parser handoff publication and semantic observation | `NoSafeSlice` until coverage is complete | never round to pure Script, `NonCandidate`, or empty catalogs |
| `CohortUnresolved` | the exhaustive top-level shape table cannot classify a retained item | typed stop before parser handoff/source rows and all semantic effects | `NoSafeSlice` or a separately named compatibility owner | never use empty catalog, `NonCandidate`, or raw fallback |
| `CanonicalScriptCohortAdmitted` | the future parser-only `CanonicalScriptCohortAdmissionV1` co-seals the pure-Script cohort, parameter-source completeness, and one parser brand | issue one non-Clone cohort admission; no identity/A/Recipe/physical effect | parser input handoff D0 consumes once and adds identity/coverage | no clone, reparse, AST/name pairing, or compatibility fallback |
| `IntegrityInvalid` | complete observation finds duplicate, foreign, stale, mixed-cohort, cardinality, or source/config drift | typed reject before parser handoff, resolver, Builder, and child effects | terminal source candidate discard | no retry, repair-by-AST, compatibility, or raw fallback |
| `NonCandidate` (A-only) | future A issuer completes semantic observation and proves zero direct-static candidates | no direct-static package or physical effect | canonical non-direct-static owner | this P0/parser issuer never emits it |
| `Transported` (future C/B) | future parser-handoff consumer moves the typed admission/semantic result once | no replay or second source interpretation | detached consumer terminal | this P0 never emits or reuses it |

`CanonicalScriptCohortAdmitted` is the only public pure-Script cohort admission
from this row. It is not the canonical identity state and not `HandoffReady`.
`NoBoxDeclarations` remains `CompatibilitySource` until the new cohort issuer
proves the exhaustive pure-Script shape rule. `NonCandidate` belongs to A, and
`Transported` belongs to a future consumer; neither may be used to hide a
missing parser row. Every routing arm must match this table exhaustively.

## Ownership and phase boundary

```text
parser cohort admission
  -> CanonicalScriptCohortAdmitted
front-door identity/profile/receipt co-seal
  -> CanonicalSourceBacked
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
- the issuer consumes the same-invocation postpass/parameter-source product
  and parser brand exactly once; front-door identity is a later co-seal;
- complete ProgramBody, declaration, Brand, import/config, and identity
  coverage is required before the parser handoff can issue `HandoffReady`;
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
   `CanonicalScriptCohortAdmitted`;
6. the parser admission issues identity, resolver/target/result/proof/terminal
   meaning instead of leaving those to the named downstream owner;
7. the parser handoff or A issuer is implemented before this finite table is
   accepted and guarded;
8. the new source product crosses the 760-line split trigger or 800-line hard
   stop, or requires a second parser invocation/compatibility fallback.

Until these are closed, the parser input handoff, A issuer, Recipe/Join,
physical bridge, canonical production consumer, and old-route retirement stay
parked. This is a development `NoSafeSlice`, not a compiler disposition.

## P0 closeout receipt

- the parser-only cohort issuer is separated from front-door identity and the
  later parser input handoff;
- `NoBoxDeclarations`, `SelectedBuildGateUnsupported`, empty catalogs, and
  unknown top-level shapes have explicit non-success states;
- every retained top-level AST shape is assigned to `PureScriptSyntax`,
  `CompatibilitySource`, `NotApplicable`, or `CohortUnresolved` before any
  semantic effect;
- `CanonicalScriptCohortAdmitted` is the only public pure-Script cohort
  outcome, and it carries no identity, AST-free window, resolver, Recipe, or
  physical meaning;
- worker read-only audits and the classification-completeness guard agree on
  the boundary; no code or fixture was changed by this P0;
- the next selected row is
  `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-PARSER-INPUT-HANDOFF-D0`.
