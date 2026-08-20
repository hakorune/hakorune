---
Status: Active design stop
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-INPUT-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-only-a-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one complete Builder-free input envelope for canonical Script source-only A
Classification: BoxCount (design only; implementation remains closed)
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-INPUT-D0

## Six-line brief

Decision: Define the complete canonical input envelope that a future
Builder-free Script direct-static source-only A must receive. This D0 does not
issue A, C, B, Recipe, Join, physical, or production effects.

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

Smallest next slice: this docs-only D0 names the one input owner, lifetime,
complete input set, finite state table, and A/C/B boundary. No carrier,
source admission change, code, fixture, or canonical request mutation is
authorized until this input contract is accepted.

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

## Exhaustive input-envelope state table

The input owner must expose every routing outcome explicitly. No optional
payload, wildcard, `unwrap_or(default)`, or compatibility label may merge
these rows.

| state | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|
| `NotApplicable` | canonical frontdoor proves non-Script, non-canonical-profile, or outside direct-static scope | no input observation and no A effect | caller-owned non-Script dispatch | never fabricate `NonCandidate` or enter Script raw by absence |
| `CompatibilitySource` | parser/source handoff explicitly marks a compatibility cohort | preserve typed compatibility origin; do not issue canonical input | existing compatibility owner or parked design stop | never become `SourceAuthorityUnavailable`, `NonCandidate`, or A success |
| `Deferred` | Script resolver/source admission returns `ResolveScriptForestOutcomeV1::Deferred` | preserve the deferred reason; no partial input observation | explicit deferred owner or `NoSafeSlice` | never become empty input, `NonCandidate`, or raw success |
| `SourceAuthorityUnavailable` | parser lineage/profile/one-shot receipt or canonical Script source is absent or mismatched before observation | typed stop before input package/Recipe/entry effects | `NoSafeSlice` until the issuer/identity is closed | no default identity, AST rescan, or raw fallback |
| `ObservationIncomplete` | source authority exists, but retained window/forest/catalog coverage cannot be observed totally once | typed stop before input package and child effects | `NoSafeSlice` until coverage is total | never round to `NonCandidate`, compatibility, or raw success |
| `InputAuthorityReady` | one compiler child validates the complete window and co-seals all required source-bound inputs | issue one move-only input envelope; no physical effect | future A consumes it exactly once | no second issuer, selected-normal copy, or by-name re-pairing |
| `IntegrityInvalid` | complete input observation finds duplicate, foreign, stale, missing, mixed, or contradictory rows | typed reject before Recipe/entry/child effects | terminal candidate/session discard | no retry, re-pair, `NonCandidate`, compatibility, or raw fallback |
| `Transported` | future C-to-B handoff consumes the ready envelope exactly once | no replay or second source interpretation | detached canonical consumer terminal | no clone, replay, or return to source/raw |

`NoSafeSlice` is a development stop, not a runtime disposition. `InputAuthorityReady`
is a complete source-input package, not a physical permission and not the final
direct-static disposition. `Transported` belongs to the future C-to-B phase;
it is not a second source issuer.

## Exhaustive transitions

```text
Script input
  -> NotApplicable | CompatibilitySource | Deferred
  -> SourceAuthorityUnavailable | ObservationIncomplete
  -> [after complete observation] InputAuthorityReady | IntegrityInvalid
InputAuthorityReady -> A source package | IntegrityInvalid   (future A only)
A source package -> C disposition -> B transport               (future only)
Transported       -> detached terminal only; no replay
```

`SourceAuthorityUnavailable` means observation cannot begin. 
`ObservationIncomplete` means identity is present but total coverage cannot be
issued. `IntegrityInvalid` is reserved for a complete observation whose known
rows fail validation. `CompatibilitySource` is a typed non-canonical lane and
must never be silently treated as missing canonical authority.

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

## A/C/B owner boundary

```text
Input D0 / A ingress
  one compiler child validates and issues InputAuthorityReady once

A  CanonicalScriptDirectStaticSourceOnlyIssuerV1
  consumes the ready envelope and issues the AST-free source package once

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
- `InputAuthorityReady` contains no AST, `ValueId`, MIR/Builder physical
  fact, or Recipe key;
- all required source products are co-sealed as one package; partial optional
  pieces cannot be paired, and a non-final terminal candidate is
  `IntegrityInvalid`, not `NonCandidate`;
- `CompatibilitySource`, `Deferred`, `SourceAuthorityUnavailable`,
  `ObservationIncomplete`, `InputAuthorityReady`, and `IntegrityInvalid`
  remain distinct through the future A/C/B request;
- the future A consumer and C/B retirement edge are named;
- no code, fixture, source admission, carrier, Recipe/Join, physical Call,
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
