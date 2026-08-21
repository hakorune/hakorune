---
Status: Ready for design review — phase-qualified state matrix
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-SOURCE-A-STATE-MATRIX-P0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-issuer-d0-2026-08-21.md
ProductionCaller: none; docs-only prerequisite
ReplacementCell: parser admission -> handoff -> Source-only A boundary
Classification: BoxShape (design completeness; no code or accepted shape)
NextCard: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-ISSUER-D0
---

# SCRIPT-DIRECT-STATIC-SOURCE-A-STATE-MATRIX-P0

## Six-line brief

Decision: Make the Source-only A routing contract phase-qualified and
exhaustive before any issuer implementation; every upstream, transport, A,
and future C/B state must have one owner and one explicit transition.

Source authority + canonical issuer: existing parser admission, parser-row
handoff, and compiler carrier remain their current sole issuers; the future
`CanonicalScriptDirectStaticSourceOnlyIssuerV1` alone may issue A facts.

Non-authority: state names in prose, `Option::None`, `unwrap_or(default)`,
`CanonicalSourceBacked` as a standalone alias, AST/name/ordinal/digest joins,
Builder/`comp_ctx`, Recipe/Join, and a missing row cannot issue a disposition.

Fail-fast boundary: classify and preserve the exact phase state before
`prepare_script_recipe()`, `OpenScriptPhysicalEntryV1`, Builder installation,
or child effects; missing coverage is incomplete, while a present foreign or
contradictory row is integrity-invalid.

Smallest next slice: update the A issuer D0 and its pointer to this complete
matrix; do not add code, fixtures, A facts, Recipe, Join, physical Call, or
production routing.

Non-claims: no parser/source admission change, source identity re-issuer,
resolver/target/result/proof package, canonical consumer, fallback/retry,
compatibility/raw retirement, ABI/backend, performance, or production switch.

## Why this P0 exists

The implementation has several real phase-specific enums, but the A issuer
card previously presented only a mixed table. That omission makes a neutral
or transport state look like an A disposition and permits a future reviewer
to collapse `CohortUnresolved`, `AdmissionMissing`, `HandoffReady`, or
`DiscardedBeforeA` into absence. The matrix is a design guard, not a new
runtime enum.

The actual code vocabulary is split across:

```text
parser/callable_parameter_source/canonical_script_source_admission.rs
  CanonicalScriptCohortDispositionV1
parser/callable_parameter_source/script_source_rows_model.rs
  CanonicalScriptSourceRowsDispositionV1
runner/reference/normal_file_vm_frontdoor/script_source_input.rs
  CanonicalScriptSourceInputDispositionV1
mir/compiler/canonical_script_source_a_input.rs
  CanonicalScriptSourceAInputTransportV1
```

The design name `CanonicalSourceBacked` is not a standalone code state. It
means the co-seal of an admitted canonical cohort, a matching `HandoffReady`
payload, and the parser/profile/receipt identity. A lone parser admission or
a lone identity receipt cannot enter A observation.

## Phase-qualified exhaustive state inventory

### 1. Parser admission (before source rows)

| state | sole issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `parser.NotApplicable` | parser root/profile classifier | no Script observation | caller-owned non-Script lane | never fabricate pure Script |
| `parser.CompatibilitySource` | parser cohort/source admission | preserve non-canonical origin | compatibility owner or stop | never become A success |
| `parser.Deferred` | upstream typed admission | preserve reason, zero A effect | deferred owner or `NoSafeSlice` | never empty-success/raw |
| `parser.SourceAuthorityUnavailable` | parser postpass/parameter authority | stop before rows | typed stop | no default/rescan |
| `parser.CohortUnresolved` | exhaustive AST/cohort issuer | stop before handoff | unresolved/compatibility owner or `NoSafeSlice` | never `NonCandidate` |
| `parser.CanonicalScriptCohortAdmitted` | parser cohort issuer with complete parameter source | issue one parser witness | source-row issuance | no clone/reparse/name pairing |
| `parser.IntegrityInvalid` | parser validation | reject before rows/effects | discard terminal | no retry/repair/fallback |

`CanonicalScriptCohortAdmitted` is syntax/cohort evidence only. It does not
mean the retained Script window, declaration views, resolver forest, target
inventory, proof, or terminal relation exists.

### 2. Parser/frontdoor row and carrier transport

| state | sole issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `transport.NotApplicable` | frontdoor family classifier | no carrier effect | Main/Callable owner | never fabricate Script input |
| `transport.CompatibilitySource` | parser-row/frontdoor mapping | preserve typed compatibility | compatibility owner/stop | never canonical A |
| `transport.Deferred` | parser-row/frontdoor mapping | preserve deferred reason | deferred owner/stop | never `NonCandidate` |
| `transport.AdmissionMissing` | parser admission mapping | stop before carrier | non-canonical terminal | never `HandoffReady` |
| `transport.CohortUnresolved` | parser admission mapping | stop before carrier | typed stop | never empty success |
| `transport.SourceAuthorityUnavailable` | frontdoor identity/profile/receipt check | stop before compiler candidate | `NoSafeSlice` | no default/rescan |
| `transport.ObservationIncomplete` | row issuer/frontdoor co-seal | stop before A/Recipe | incomplete terminal | never absence/raw |
| `transport.IntegrityInvalid` | frontdoor co-seal | stop before A/Recipe | integrity terminal | no stale/foreign reuse |
| `transport.NonCandidate` | a complete source-family classifier | no A/Recipe effect | canonical non-direct-static owner | never raw fallback |
| `transport.HandoffReady` | parser rows + profile/receipt co-seal | move once into compiler request | future A or named no-A discard | no silent field drop |
| `transport.DiscardedBeforeA` | current compiler no-A boundary | no candidate publication | terminal discard | never call it consumed |
| `transport.HandoffConsumed` | future named A consumer only | A observation begins once | A issuer | impossible before named consumer |
| `transport.DispositionTransported` | future C/B owner | no source reinterpretation | detached future consumer | never reuse as parser/A state |

`HandoffReady` is not `HandoffConsumed`. The current carrier's
`discard_before_a_consumer()` is a named no-A terminal and must disappear or
become unreachable when a real A consumer is opened. `DispositionTransported`
is future C/B vocabulary and must not be used to claim parser consumption.

### 3. Source-only A observation and package

| state | sole issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `A.SourceAuthorityUnavailable` | future A issuer before observation | stop before package/Recipe/entry | `NoSafeSlice` | no Builder/default identity |
| `A.ObservationIncomplete` | future A issuer with identity but missing coverage | stop before package/child effects | `NoSafeSlice` | never `NonCandidate` |
| `A.NonCandidate` | future A issuer after complete clean observation | no direct-static package/physical effect | canonical non-direct-static owner | missing coverage is not absence |
| `A.InputAuthorityReady` | same A issuer, private readiness only | continue inside A; no physical effect | `A.DirectStaticSourceReady` or `A.IntegrityInvalid` | no public second receipt |
| `A.DirectStaticSourceReady` | same A issuer after all source rows/proof/terminal co-seal | move one AST-free package | future C consumes once | no name lookup/retry/re-pairing |
| `A.IntegrityInvalid` | same A issuer with present foreign/duplicate/stale/contradictory rows | reject before Recipe/entry/effects | candidate/session discard | no retry, compatibility, or raw |

The boundary is strict:

```text
missing expected row / coverage gap / unavailable observer -> ObservationIncomplete
present row with foreign, duplicate, stale, or contradictory identity
  -> IntegrityInvalid
complete clean observation with zero direct-static rows -> NonCandidate
```

If the current canonical source family cannot observe a target inventory at
all, it is `A.ObservationIncomplete`/`NoSafeSlice`, not `A.NonCandidate`.
`NonCandidate` is only valid after the issuer proves complete zero-row
coverage.

### 4. Future C/B transport

| state | sole issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `C.DispositionReady` | future C disposition owner consuming A once | typed disposition only | future B transport | no source re-observation |
| `B.DispositionTransported` | future B carrier | no semantic reinterpretation | detached canonical consumer | no clone/replay/raw return |

These are future phase labels, not current parser or A enum variants. They
must not be represented by a bare `Transported` state shared across phases.

## Exhaustive transition contract

```text
parser.NotApplicable / CompatibilitySource / Deferred /
  SourceAuthorityUnavailable / CohortUnresolved / AdmissionMissing /
  IntegrityInvalid
    -> named upstream owner or terminal

parser.CanonicalScriptCohortAdmitted
    -> transport.HandoffReady | transport.ObservationIncomplete
       | transport.IntegrityInvalid

transport.HandoffReady
    -> transport.DiscardedBeforeA
       | transport.HandoffConsumed (future named A consumer only)

transport.HandoffConsumed
    -> A.SourceAuthorityUnavailable | A.ObservationIncomplete
       | A.NonCandidate | A.InputAuthorityReady | A.IntegrityInvalid

A.InputAuthorityReady
    -> A.DirectStaticSourceReady | A.IntegrityInvalid

A.DirectStaticSourceReady
    -> C.DispositionReady -> B.DispositionTransported (future only)
```

No wildcard, `Option::None`, empty carrier, `unwrap_or(false)`, or raw
fallback may create an unlisted edge. Every row has one authority, one
pre-effect rule, one terminal/continuation, and one fallback policy.

## Acceptance and NoSafeSlice

Acceptance for this docs-only P0:

- the four actual code enum families are mapped to phase-qualified rows;
- `CanonicalSourceBacked` is documented as a co-seal alias, not an issuer;
- `CohortUnresolved`, `AdmissionMissing`, `HandoffReady`,
  `DiscardedBeforeA`, `HandoffConsumed`, and `DispositionTransported` each
  have an explicit owner and transition;
- missing/coverage gaps and present-invalid rows have distinct outcomes;
- `NonCandidate` requires complete clean observation;
- the A issuer D0 points to this matrix and then back to its own design stop;
- no code, fixture, parser/source admission, Recipe, Join, physical, or
  production route changes.

Remain at design stop if any state cannot be assigned one owner, if a phase
uses a bare state name that collides with another phase, if a missing row can
become `NonCandidate`, if `HandoffConsumed` can be issued without a named A
consumer, or if the next implementation would need to infer a row by name,
ordinal, pointer, digest, or Builder state.
