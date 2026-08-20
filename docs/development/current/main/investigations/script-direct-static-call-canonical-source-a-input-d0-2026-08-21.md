---
Status: Closed — carrier D0 selected
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-INPUT-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-only-a-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one complete Builder-free input envelope for canonical Script source-only A
Classification: BoxCount (design only; implementation remains closed)
NextCard: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-carrier-d0-2026-08-21.md
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-INPUT-D0

## Six-line brief

Decision: Close the semantic input contract and split the missing transport into
one carrier D0. The carrier moves parser-backed input to the future A boundary;
it does not issue A, C, B, Recipe, Join, physical, or production effects.

Source authority + canonical issuer: the parser-backed
`SealedNormalScriptSourceV1` and its already-sealed parser lineage are the
source authority. A future compiler child
`CanonicalScriptDirectStaticSourceOnlyIssuerV1` is the single issuer of the
input-backed source package; this D0 must not invent a second semantic issuer.

Non-authority: `NormalSourcePlanClassifierV1`,
`CanonicalCoreSourcePlanCompileRequestV1`, selected-normal
`normal_default_root_catalog_lifecycle`, `Builder`/`comp_ctx`,
`RawScriptBodyRecipeV1`, AST/path/name/pointer/ordinal joins, digest-only
equality, `ValueId`/`MirType`, and compatibility success cannot complete the
input envelope.

Fail-fast boundary: after Script-family and identity validation, but before
`prepare_script_recipe()` or `OpenScriptPhysicalEntryV1`, the envelope must
have one complete retained Script window, declaration/import/brand views,
resolver forest, target/result catalogs, required-argument proof, and exact
source rows. Missing, partial, foreign, or contradictory input stops before
Recipe/entry/child effects.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-CARRIER-D0`
fixes the one move-only parser/frontdoor-to-compiler lifetime and its
phase-qualified states. A semantic issuer remains closed until that carrier
is accepted.

Non-claims: no A/C/B implementation, Recipe/Join issuance, physical Call or
publication, Return/signature, canonical production switch, raw or
compatibility retirement, ABI/backend/performance work, or selected-normal
cutover.

## Why this prerequisite is separate

The parser/source identity prerequisite is closed by
`script-direct-static-call-canonical-source-identity-i0-2026-08-21.md`.
The remaining canonical gap is not identity transport; it is that the current
canonical Script path has no complete semantic input owner:

```text
canonical_core_dispatch::compile_script
  -> prepare_script_recipe()
  -> OpenScriptPhysicalEntryV1
```

The selected Builder lifecycle can currently issue pieces of the needed
target/result/Recipe/Join products, but those products are Builder-owned,
retain temporary pointer identity, and are not a canonical source authority.
This D0 therefore defines the input boundary before any attempt to move or
reuse those products.

## Audit correction: source rows are pre-A input, not an A package

The landed parser handoff is a valid AST-free syntax/coverage input, but it is
not yet the complete A package. It currently contains:

```text
ProgramBody ordinal/kind rows
declaration and Brand syntax snapshots
explicit NoImports configuration
parser brand plus one-read/one-parse lineage/receipt
```

It does not contain the resolver forest, target/result catalog, ordered
semantic sites, required-argument proof, or final Sequence/root-Return
relation. Those are future A-issued facts. The two layers must not be joined
later by statement ordinal, pointer, name, path, or digest equality.

The current code also drops `script_input` at
`source_plan_input.rs::into_canonical_core_compile_request()` because the
canonical request has no A consumer. That is an intentional current
non-claim, but it is a real transport boundary. A future carrier must replace
that named discard; an A implementation may not be added directly at
`compile_script()` while the input has already been dropped.

### Upstream parser and handoff state inventory

These states are issued before A observation and are not A dispositions:

| state | phase | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|---|
| `CanonicalScriptCohortAdmitted` | parser admission | parser cohort issuer with complete parameter source | allow one row scan | `HandoffReady` if rows seal | never infer from `is_source_backed()` |
| `CohortUnresolved` | parser admission | parser cohort issuer lacks a canonical pure-Script decision | no rows or A effect | typed stop/compatibility owner | never empty-success or A candidate |
| `AdmissionMissing` | parser admission | selected build-gate/unsupported parameter source | no rows or A effect | explicit non-canonical owner | never `HandoffReady` |
| `CompatibilitySource` | upstream | parser/source admission | preserve compatibility origin | compatibility owner or stop | never canonical A |
| `Deferred` | upstream | source/resolver admission | preserve deferred reason | deferred owner or stop | never `NonCandidate` |
| `SourceAuthorityUnavailable` | upstream | lineage/profile/receipt preflight | stop before carrier | `NoSafeSlice` | no default identity or rescan |
| `HandoffReady` | parser/frontdoor transport | parser rows plus profile/receipt co-seal | move once; no semantic A effect | carrier takes it or explicit `DiscardedBeforeA` | no `script_input: _` drop |
| `DiscardedBeforeA` | transport terminal | named no-A rejection/discard owner | candidate is not published | terminal discard | never call it `HandoffConsumed` |

`HandoffConsumed` is reserved for a real named A consumer. The existing enum
variant is transport vocabulary, not proof that A ran. `DispositionTransported`
belongs to the future C-to-B lifecycle and is not a parser or A outcome.

## Exhaustive input-envelope state table

The input owner must expose every routing outcome explicitly. No optional
payload, wildcard, `unwrap_or(default)`, or compatibility label may merge
these rows.

| state | phase | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|---|
| ingress | `NotApplicable` | canonical frontdoor proves non-Script, non-canonical-profile, or outside direct-static scope | no input observation and no A effect | caller-owned non-Script dispatch | never fabricate `NonCandidate` or enter Script raw by absence |
| ingress | `CompatibilitySource` | parser/source handoff explicitly marks a compatibility cohort | preserve typed compatibility origin; do not issue canonical input | existing compatibility owner or parked design stop | never become `SourceAuthorityUnavailable`, `NonCandidate`, or A success |
| ingress | `Deferred` | Script resolver/source admission returns `ResolveScriptForestOutcomeV1::Deferred` | preserve the deferred reason; no partial input observation | explicit deferred owner or `NoSafeSlice` | never become empty input, `NonCandidate`, or raw success |
| ingress | `SourceAuthorityUnavailable` | parser lineage/profile/one-shot receipt or canonical Script source is absent or mismatched before observation | typed stop before input package/Recipe/entry effects | `NoSafeSlice` until the issuer/identity is closed | no default identity, AST rescan, or raw fallback |
| ingress | `ObservationIncomplete` | source authority exists, but retained window/forest/catalog coverage cannot be observed totally once | typed stop before input package and child effects | `NoSafeSlice` until coverage is total | never round to `NonCandidate`, compatibility, or raw success |
| ingress | `NonCandidate` | one issuer completes the retained-window observation and proves every row explicitly outside direct-static scope | no A input package or physical effect | continue only through the canonical non-direct-static source owner | missing coverage is not absence; never enter raw or compatibility by default |
| ingress | `InputAuthorityReady` | the single A issuer has validated the complete window and co-sealed all required source-bound inputs; this is private readiness, not a second issuer | continue inside that issuer toward one A package; no physical effect | future `DirectStaticSourceReady` is the only semantic package that leaves A | no public carrier, second issuer, selected-normal copy, or by-name re-pairing |
| ingress | `IntegrityInvalid` | complete input observation finds duplicate, foreign, stale, or contradictory rows | typed reject before Recipe/entry/child effects | terminal candidate/session discard | no retry, re-pair, `NonCandidate`, compatibility, or raw fallback |
| C-to-B | `Transported` | future C-to-B handoff consumes the ready envelope exactly once | no replay or second source interpretation | detached canonical consumer terminal | no clone, replay, or return to source/raw |

`NoSafeSlice` is a development stop, not a runtime disposition. `NonCandidate`
requires complete, integrity-clean observation; it is not a synonym for a
missing or partial input. `InputAuthorityReady` is private readiness inside
the sole A issuer, not a public source package, physical permission, or final
direct-static disposition. `Transported` belongs to the future C-to-B phase,
not to the A issuer, and is not a second source issuer.

## Exhaustive transitions

```text
Script input
  -> NotApplicable | CompatibilitySource | Deferred
  -> SourceAuthorityUnavailable | ObservationIncomplete
  -> [after complete observation] NonCandidate | InputAuthorityReady | IntegrityInvalid
InputAuthorityReady -> DirectStaticSourceReady | IntegrityInvalid (inside A only)
A source package -> C disposition -> B transport               (future only)
Transported       -> detached terminal only; no replay
```

`SourceAuthorityUnavailable` means observation cannot begin. 
`ObservationIncomplete` means identity is present but total coverage cannot be
issued. `NonCandidate` means complete observation found zero direct-static
rows, and continues through the canonical non-direct-static owner only.
`IntegrityInvalid` is reserved for a complete observation whose known rows
fail validation. Missing expected rows or a coverage gap is
`ObservationIncomplete`, not `IntegrityInvalid`. `CompatibilitySource` is a typed non-canonical lane and must
never be silently treated as missing canonical authority. `Transported` is a
future C-to-B state, not an A outcome.

## Required input envelope

The future A issuer may borrow existing source-backed owners only through one
co-sealed, AST-free envelope containing:

```text
source identity + parser profile + one-read/one-parse lineage
retained Script ProgramBody window and exact coverage receipt
declaration/import/brand semantic views
one resolver semantic forest for that retained source
exact direct-static target and result facts
ordered source-bound argument/result sites
required-callee-argument proof
terminal destination / continuation relation
```

The envelope must retain no AST, `ValueId`, MIR block, Builder ordinal,
physical instruction, or Recipe key. A temporary resolver borrow is allowed
only while the compiler child validates and co-seals the owned envelope. A
filename, pointer, path, name, ordinal, or digest equality cannot pair any
two rows after issuance.

The envelope is one all-or-none source package. The five independent optional
pieces currently present in selected lowering input (bundle, publication
owner, Recipe, Join, and required-argument proof) are not a canonical input
authority and may not be combined opportunistically. A missing piece is
`ObservationIncomplete` before issuance; a present-but-foreign, stale, or
contradictory piece is `IntegrityInvalid`. In particular, a target row that is
observed but fails the final Sequence/root-Return terminal relation is
`IntegrityInvalid`, never `NonCandidate` or `ObservationIncomplete`.

## Future issuer contract (design only)

The future `CanonicalScriptDirectStaticSourceOnlyIssuerV1` must perform one
source-backed issuance in this order, without calling the selected Builder
lifecycle:

1. validate parser lineage, profile, digest, UTF-8 length, and one-read/
   one-parse receipt;
2. issue the complete retained `ProgramBody` window and its coverage receipt;
3. co-seal declaration facts, the Brand catalog, and the canonical static
   import/config snapshot under that same source identity;
4. issue one resolver forest from those views and the window;
5. issue a target inventory whose rows include explicit noncandidate reasons;
6. co-seal target/result rows, ordered receiver/argument/result sites, and the
   required-callee-argument proof;
7. issue and verify the final-Sequence/root-Return terminal relation;
8. co-seal the existing source facts needed by Bundle, Publication, Recipe,
   Join, and proof as one all-or-none package;
9. move one AST-free input envelope to the future A issuer.

`FunctionOwnerIdV1` is an invocation-local forest brand, not source identity.
The source package must not use it, AST pointers, names, paths, ordinals, or
digest equality as a later join key. Existing
`normal_default_root_catalog_lifecycle`, `MirBuilder`, `comp_ctx`, and the
selected direct-static products may be audited for behavior but are not this
issuer. `canonical_core_dispatch.rs` and the Builder lifecycle remain thin;
semantic growth belongs in a new sibling owner below the 760/800-line limits.

## A/C/B owner boundary

```text
Input D0 / A ingress
  one A issuer validates its complete source input and reaches private
  InputAuthorityReady; no separate semantic input issuer is published

A  CanonicalScriptDirectStaticSourceOnlyIssuerV1
  is the sole issuer; it moves from private InputAuthorityReady to one
  AST-free DirectStaticSourceReady package

C  CanonicalScriptDirectStaticDispositionV1
  is the sole disposition owner; it consumes A's package and issues no
  second target/result/Recipe/Join/physical fact

B  CanonicalCoreSourcePlanCompileRequestV1
  is typed transport only; an absent optional payload may not select
  RawScriptBodyRecipeV1
```

There is intentionally one canonical source issuer family. The Input D0
does not authorize `normal_default_root_catalog_lifecycle` or `comp_ctx` to
become a second issuer, and it does not move selected Builder products into
the canonical path by pointer or name.

## Acceptance for this design stop

- the finite table includes `CompatibilitySource`, the neutral
  `ObservationIncomplete`, terminal `IntegrityInvalid`, and future
  `Transported`, with one owner and one fallback policy each;
- parser identity is consumed from the already-landed lineage; no second
  read, parse, resolver pass, AST scan, or digest-only join is introduced;
- one complete retained Script window and its coverage receipt are named;
- declaration/import/brand views, resolver forest, target/result catalogs,
  required proof, and terminal/source rows have one lifetime and one issuer;
- the issuer contract names explicit noncandidate reasons and verifies the
  complete target/window cardinality before issuing `NonCandidate` or
  `InputAuthorityReady`;
- private `InputAuthorityReady` contains no AST, `ValueId`, MIR/Builder
  physical fact, or Recipe key and is never a second public authority;
- all required source products are co-sealed as one package; partial optional
  pieces cannot be paired, and a non-final terminal candidate is
  `IntegrityInvalid`, not `NonCandidate`;
- a complete, integrity-clean zero-candidate observation is the explicit
  `NonCandidate` state and continues only through the canonical non-direct-
  static owner;
- `CompatibilitySource`, `Deferred`, `SourceAuthorityUnavailable`,
  `ObservationIncomplete`, `InputAuthorityReady`, and `IntegrityInvalid`
  remain distinct through the future A/C/B request;
- the future A consumer and C/B retirement edge are named;
- no code, fixture, source admission, Recipe/Join, physical Call,
  publication, production switch, fallback, or performance run is opened.

## NoSafeSlice conditions

Remain at this D0 if any condition holds:

1. the canonical frontdoor can only see selected-normal Builder state;
2. a required input has no source-backed issuer or complete lifetime;
3. selected Builder products must be paired by pointer, path, name,
   ordinal, filename, or digest-only equality;
4. compatibility or deferred input must be rounded to `SourceAuthorityUnavailable`
   or `NonCandidate`;
5. the envelope needs AST, `ValueId`, MIR block, physical, or Recipe-key data;
6. A/C/B would issue the same source meaning more than once;
7. an absent optional payload would still select `RawScriptBodyRecipeV1`;
8. implementing the input requires source admission, production switch, raw
   retirement, or a second resolver/semantic pass.

Until all are closed, the next row remains design-only and no canonical
physical consumer or production claim is allowed.
