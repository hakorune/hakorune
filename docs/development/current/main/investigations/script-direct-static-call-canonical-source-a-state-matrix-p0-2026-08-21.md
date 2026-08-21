---
Status: Closed — phase-qualified state matrix accepted
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-SOURCE-A-STATE-MATRIX-P0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-issuer-d0-2026-08-21.md
ProductionCaller: none; docs-only prerequisite
ReplacementCell: parser admission -> handoff -> Source-only A boundary
Classification: BoxShape (design completeness; no code or accepted shape)
NextCard: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-D0
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

Smallest next slice: open the complete-observation D0 against this accepted
matrix; do not add code, fixtures, A facts, Recipe, Join, physical Call, or
production routing in the matrix row.

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

This card is the first concrete application of the tracked
`agent-current-entry-contract-ssot.md` classification-completeness check:
every phase row names its owner, pre-effect behavior, terminal/continuation,
and fallback policy, including the state that is neither selected nor
rejected. The table is deliberately phase-qualified so a similarly named
transport or future C/B state cannot silently become an A disposition.

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
| `parser.ObservationIncomplete` | parser disposition projection (reserved in the current cohort issuer) | preserve the typed coverage gap; no handoff/A effect | source-row projection or typed stop | never round to parser/A `NonCandidate` |
| `parser.NonCandidate` | parser disposition projection (reserved; not issued by the current cohort issuer) | preserve only a complete parser-side zero-row observation | source-row projection; A must reclassify its own outcome | never treat as A package success |
| `parser.DispositionTransported` | future parser-to-C/B transport projection (reserved) | no source reinterpretation | detached future transport terminal | never reuse as parser admission or A state |

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
| `transport.rows.HandoffConsumed` | parser-row/frontdoor embedding sentinel | no A observation | source-plan mapping or terminal | never treat as A consumption |
| `transport.input.HandoffConsumed` | frontdoor input forwarding sentinel | no A observation | compiler carrier mapping or terminal | never treat as A consumption |
| `transport.compiler.HandoffConsumed` | future named A consumer only | A observation begins once | A issuer | impossible before named consumer |
| `transport.DispositionTransported` | future C/B owner | no source reinterpretation | detached future consumer | never reuse as parser/A state |

`HandoffReady` is not `HandoffConsumed`. The current carrier's
`discard_before_a_consumer()` is a named no-A terminal and must disappear or
become unreachable when a real A consumer is opened. `DispositionTransported`
is future C/B vocabulary and must not be used to claim parser consumption.
The Rust spelling `HandoffConsumed` currently appears in both parser/frontdoor
row embedding and compiler transport enums. `transport.rows.HandoffConsumed`
and `transport.input.HandoffConsumed` are internal forwarding/embedding
sentinels; only a future `transport.compiler.HandoffConsumed` may mean that a
named A consumer has taken the carrier. They are phase-qualified states, not
one shared semantic transition.

### 3. Source-only A observation and package

| state | sole issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `A.SourceAuthorityUnavailable` | future A issuer before observation | stop before package/Recipe/entry | `NoSafeSlice` | no Builder/default identity |
| `A.ObservationIncomplete` | future A issuer with identity but missing coverage | stop before package/child effects | `NoSafeSlice` | never `A.CompleteNoDirectStaticRows` |
| `A.CompleteNoDirectStaticRows` | future A issuer after complete clean observation; private witness | no direct-static package/physical effect | future C may issue public `C.NonCandidate` | missing coverage is not absence |
| `A.InputAuthorityReady` | same A issuer, private readiness only | continue inside A; no physical effect | `A.DirectStaticSourceReady` or `A.IntegrityInvalid` | no public second receipt |
| `A.DirectStaticSourceReady` | same A issuer after all source rows/proof/terminal co-seal | move one AST-free package | future C consumes once | no name lookup/retry/re-pairing |
| `A.IntegrityInvalid` | same A issuer with present foreign/duplicate/stale/contradictory rows | reject before Recipe/entry/effects | candidate/session discard | no retry, compatibility, or raw |

The boundary is strict:

```text
missing expected row / coverage gap / unavailable observer -> ObservationIncomplete
present row with foreign, duplicate, stale, or contradictory identity
  -> IntegrityInvalid
complete clean observation with zero direct-static rows -> private CompleteNoDirectStaticRows
```

If the current canonical source family cannot observe a target inventory at
all, it is `A.ObservationIncomplete`/`NoSafeSlice`, not
`A.CompleteNoDirectStaticRows`. Public `C.NonCandidate` is only valid after
the future C owner consumes that private witness and applies its own
disposition contract.

### 4. Future C/B transport

| state | sole issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `C.NonCandidate` | future C disposition owner after consuming the private A zero-row witness | no direct-static physical effect | future B transport or non-direct-static owner | C never re-observes parser source |
| `C.DispositionReady` | future C disposition owner consuming A once | typed disposition only | future B transport | no source re-observation |
| `B.DispositionTransported` | future B carrier | no semantic reinterpretation | detached canonical consumer | no clone/replay/raw return |

These are future phase labels, not current parser or A enum variants. They
must not be represented by a bare `Transported` state shared across phases.

## Exhaustive transition contract

```text
parser.NotApplicable / parser.CompatibilitySource / parser.Deferred /
  parser.SourceAuthorityUnavailable / parser.CohortUnresolved /
  parser.IntegrityInvalid / parser.ObservationIncomplete /
  parser.NonCandidate / parser.DispositionTransported
    -> named upstream owner or terminal

parser.CanonicalScriptCohortAdmitted
    -> transport.HandoffReady | transport.ObservationIncomplete
       | transport.IntegrityInvalid

transport.NotApplicable / transport.CompatibilitySource / transport.Deferred /
  transport.AdmissionMissing / transport.SourceAuthorityUnavailable /
  transport.CohortUnresolved / transport.ObservationIncomplete /
  transport.IntegrityInvalid / transport.NonCandidate /
  transport.DispositionTransported
    -> named transport owner or terminal

transport.rows.HandoffReady
    -> transport.rows.HandoffConsumed (internal embedding only)

transport.input.HandoffReady
    -> transport.compiler.HandoffReady | transport.DiscardedBeforeA

transport.compiler.HandoffReady
    -> transport.DiscardedBeforeA
       | transport.compiler.HandoffConsumed (future named A consumer only)

transport.rows.HandoffConsumed / transport.input.HandoffConsumed
    -> named frontdoor/source-plan mapping or terminal; never A observation

transport.compiler.HandoffConsumed
    -> A.SourceAuthorityUnavailable | A.ObservationIncomplete
       | A.CompleteNoDirectStaticRows | A.InputAuthorityReady | A.IntegrityInvalid

A.InputAuthorityReady
    -> A.DirectStaticSourceReady | A.IntegrityInvalid

A.DirectStaticSourceReady
    -> C.DispositionReady -> B.DispositionTransported (future only)

A.CompleteNoDirectStaticRows
    -> C.NonCandidate -> B.DispositionTransported (future only)

A.SourceAuthorityUnavailable / A.ObservationIncomplete /
  A.CompleteNoDirectStaticRows / A.IntegrityInvalid
    -> named A terminal or `NoSafeSlice`; never a silent ordinary route

C.NonCandidate
    -> B.DispositionTransported (future only)

C.DispositionReady
    -> B.DispositionTransported (future only)

B.DispositionTransported
    -> detached canonical consumer terminal (future only)
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
- private `A.CompleteNoDirectStaticRows` requires complete clean observation;
  public `C.NonCandidate` is issued only by future C;
- the A issuer D0 was reconciled against this matrix and the next observation
  D0 now owns the source-envelope co-seal prerequisite;
- no code, fixture, parser/source admission, Recipe, Join, physical, or
  production route changes.

## Closeout evidence (2026-08-21)

- all four actual enum families were re-read from source and mapped without a
  phase-free alias;
- parser, transport, A, C, and B transitions are phase-qualified and every
  inventory row appears in the transition contract;
- parser/frontdoor `HandoffConsumed` sentinels are distinct from the future
  compiler A-consumer transition;
- `ObservationIncomplete`, `IntegrityInvalid`, private complete-zero A, and
  public C `NonCandidate` remain distinct;
- source-plan identity, parser invocation brand, and HandoffReady co-seal is
  recorded as the next observation-D0 prerequisite; duplicate primitive-field
  pairing is not accepted;
- `bash tools/checks/current_state_pointer_guard.sh` — PASS;
- `bash tools/checks/routing_classification_completeness_guard.sh` — PASS;
- `git diff --check` — PASS;
- no code, fixture, Recipe, Join, physical, fallback, or production change was
  opened by this P0.

Remain at design stop if any state cannot be assigned one owner, if a phase
uses a bare state name that collides with another phase, if a missing row can
become `A.CompleteNoDirectStaticRows` or `C.NonCandidate`, if `HandoffConsumed` can be issued without a named A
consumer, or if the next implementation would need to infer a row by name,
ordinal, pointer, digest, or Builder state.
