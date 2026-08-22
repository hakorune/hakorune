---
Status: Parked — canonical input-authority prerequisite selected
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-disposition-d0-2026-08-20.md
NextCard: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-input-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one Builder-free canonical Script source-only disposition issuer
Classification: BoxCount (design only; implementation remains closed)
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0

## Six-line brief

Decision: Define one Builder-free source-only A that issues one complete
canonical Script direct-static source package from the parser-backed sealed
Script source and its stable source identity. The later C owner alone turns
that package into a disposition. Do not implement A, C, B, or physical
effects in this D0.

Source authority + canonical issuer: `SealedNormalScriptSourceV1` together
with the parser/source identity, profile, and source digest are the input
authority. A future `CanonicalScriptDirectStaticSourceOnlyIssuerV1` is the
single source-observation/package issuer; it may borrow a temporary resolver
forest and existing target, result, Recipe, Join, and required-argument-proof
products, then retain only AST-free source-bound rows. A later C owner alone
turns that package into the canonical disposition; A does not issue a second
physical or transport decision.

Non-authority: `normal_default_root_catalog_lifecycle` and `Builder` state,
`comp_ctx`, `RawScriptBodyRecipeV1`, AST/path/name/pointer re-resolution,
digest-only equality, `ValueId`/`MirType`, physical input/Call/publication,
and raw or compatibility success cannot issue or complete A.

Fail-fast boundary: after Script-family sealing and before
`prepare_script_recipe()`/`OpenScriptPhysicalEntryV1`, parse/postpass/profile,
stable identity, forest/catalog/window, target/result, Recipe/Join, and
required-proof coverage must be complete. Deferred, authority loss, identity
drift, mixed candidate coverage, or a missing candidate product stops before
physical effects.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-ONLY-A-D0` is
this docs-only contract: fix source identity, catalog/result ownership,
temporary forest lifetime, AST-free output, and the A-to-B/C handoff before
any carrier or canonical consumer implementation.

This A design is parked at the input boundary. The parser/source identity
handoff is now closed; the next bounded design stop is
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-INPUT-D0`, which defines the
complete canonical semantic input envelope before A can be implemented.

Non-claims: no physical input/carrier, Call/publication/Return, canonical
production switch, raw/compat retirement, source-admission change, ABI,
backend, or performance claim.

## Exhaustive source-only disposition table

The source-only issuer must expose a finite state rather than an optional
candidate. Every state has one issuer, one pre-effect behavior, one terminal or
continuation, and one fallback policy. `None`, wildcard arms, and
`unwrap_or(default)` may not merge any two rows.

| state | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|
| `NotApplicable` | canonical family classifier proves the input is outside Script direct-static scope | no Script A effect and no source scan | caller-owned non-Script dispatch | never fabricate `NonCandidate` or route Script raw by absence |
| `CompatibilitySource` | parser/source handoff explicitly marks a compatibility cohort | preserve typed compatibility origin; no A observation or physical effect | existing compatibility owner or design stop | never become `SourceAuthorityUnavailable`, `NonCandidate`, or A success |
| `Deferred` | Script resolver/source admission reports `ResolveScriptForestOutcomeV1::Deferred` | preserve the deferred reason; no candidate observation or physical effect | explicit deferred owner or design stop | never become `NonCandidate`, `DirectStatic`, or raw success |
| `SourceAuthorityUnavailable` | A cannot acquire parser postpass, stable identity, profile, complete window, or resolver/catalog authority | typed design stop before Recipe/entry effects | `NoSafeSlice` until the missing issuer/identity is closed | no empty catalog, default identity, AST rescan, or raw fallback |
| `ObservationIncomplete` | A has a source identity but cannot observe the entire retained Script window/forest/catalog exactly once | typed design stop before any A product or Recipe/entry effect | `NoSafeSlice` until coverage is made total | never round to `NonCandidate`, `Deferred`, compatibility, or raw success |
| `NonCandidate` | A's complete retained-window observation proves every row is explicitly non-candidate and integrity is clean | no direct-static product or physical effect | explicit canonical non-candidate projection may select the existing raw recipe owner | only after complete observation; missing coverage is not absence |
| `DirectStaticSourceReady` | A co-seals one source identity with complete forest, target/result, Recipe/Join, operand-proof, and terminal rows | issue one move-only AST-free source package; no physical effect | future C disposition/B transport consumes it once | no second issuer, name lookup, or selected-normal copy |
| `IntegrityInvalid` | A verifier finds missing, duplicate, foreign, stale, mixed, or contradictory source/product rows | typed reject before Recipe/entry/child effects | terminal candidate/session discard | no retry, re-pair, `NonCandidate`, compatibility, or raw fallback |
| `Transported` | future A-to-C-to-B handoff consumes `DirectStaticSourceReady` exactly once | no replay or second source interpretation | detached canonical consumer terminal (future slice) | no clone, replay, or return to source/raw |

`NonCandidate` is valid only after complete source observation. `Deferred` and
`SourceAuthorityUnavailable` are not evidence of absence. If source identity
exists but the retained window cannot be observed completely, the outcome is
`ObservationIncomplete`, not `NonCandidate`. A complete observation with
missing/duplicate/foreign rows is `IntegrityInvalid`, not
`ObservationIncomplete`. `DirectStaticSourceReady` is an A source-package
state, not a physical permission in this D0. `Transported` belongs to the
future C-to-B lifecycle, not to A's issuer; no carrier is implemented here.

## Exhaustive transitions

The finite transition relation is:

```text
Script input
  -> NotApplicable | CompatibilitySource | Deferred | SourceAuthorityUnavailable | ObservationIncomplete
  -> [after complete observation] NonCandidate | DirectStaticSourceReady | IntegrityInvalid
DirectStaticSourceReady -> Transported | IntegrityInvalid   (future C/B only)
Transported              -> detached terminal only; no replay
```

The first line is exhaustive for source-family admission and source
observation readiness. The second line is exhaustive only after A has a
complete source-authority-backed observation. `ObservationIncomplete` is the
explicit partial/coverage state; it cannot be silently reclassified as
`SourceAuthorityUnavailable` after observation begins. There is no implicit
`Pending`, `Unknown`, or compatibility wildcard. If an additional state is
discovered, this card returns to design stop and the table is revised before
implementation evidence is accepted.

### A/C/B phase and owner reconciliation

The state names above are phase-qualified by owner:

```text
A source observation/package phase:
  NotApplicable | Deferred | SourceAuthorityUnavailable
  | ObservationIncomplete | NonCandidate | DirectStaticSourceReady
  | IntegrityInvalid

C-to-B transport phase (future only):
  DirectStaticSourceReady -> Transported -> detached terminal
```

`SourceAuthorityUnavailable` means the parser-backed source/window authority
cannot be acquired or verified before observation. `ObservationIncomplete`
means that authority exists but total retained-window coverage cannot be
issued; both stop at `NoSafeSlice`, but they are different witnesses.
`IntegrityInvalid` is only for a complete observation whose expected rows are
known and then fail duplicate/foreign/stale/contradictory validation.

The owner boundary is intentionally one-way:

```text
A = canonical source observation/package issuer; no physical permission
C = sole canonical disposition owner; consumes A's package once
B = typed transport only; it does not observe source or reissue meaning
```

Thus A's `DirectStaticSourceReady` is a complete source package, C owns the
future disposition decision, and B only transports that typed decision. No
state may be inferred from an absent optional payload.

Examples pin the neutral states:

```text
outside Script direct-static scope
  -> NotApplicable
inside scope + complete window + zero candidates
  -> NonCandidate
inside scope + source identity but incomplete coverage
  -> ObservationIncomplete
inside scope + no source/window authority
  -> SourceAuthorityUnavailable
```

## A-to-C-to-B ownership contract

```text
A  CanonicalScriptDirectStaticSourceOnlyIssuerV1
   borrows parser-backed sealed Script source once, validates the complete
   window, and co-seals existing semantic products into one AST-free source
   package. It is the sole A issuer, not a physical or transport owner.

C  CanonicalScriptDirectStaticDispositionV1 (future design/implementation)
   is the only canonical disposition owner. It consumes A's package and must
   not re-resolve the AST or issue a second target/result/Recipe/Join/
   physical-input fact.

B  CanonicalCoreSourcePlanCompileRequestV1 (future transport change)
   transports C's required typed Script decision: DirectStaticSourceReady,
   explicit NonCandidate, or terminal IntegrityInvalid. An absent optional
   payload may not silently select RawScriptBodyRecipeV1. B never becomes a
   second source or disposition issuer.

detached Script entry
   remains the existing future consumer; it owns Call/publication/Return only
   after a later carrier slice.
```

The current `normal_default_root_catalog_lifecycle` is selected-normal and
Builder-backed. It is evidence for existing product issuers, not the
canonical A caller. The canonical request currently carries only plan,
admission, and source receipt, then projects Script into
`RawScriptBodyRecipeV1`; this D0 does not change that route.

## Evidence and boundary

The source-only gap is real and must not be hidden by a transport field:

- `src/mir/compiler/normal_source_plan/product.rs:151-202` owns
  `SealedNormalScriptSourceV1`, but no direct-static semantic disposition.
- `src/mir/compiler/canonical_core_dispatch.rs:94-130` carries only plan,
  admission, and receipt; its Script path at `:413-424` and `:526-563`
  prepares `RawScriptBodyRecipeV1` directly.
- `src/mir/builder/normal_default_root_catalog_lifecycle.rs:456-665` issues
  the existing target/result/Recipe/Join/proof products only in the selected
  Builder lifecycle; it is not a canonical source-only issuer.
- Existing direct-static result/Recipe/Join/physical-input products brand
  themselves with temporary AST pointer identity. Pointer identity is not a
  stable canonical source identity across moving source products and cannot
  cross A-to-B.
- `src/mir/source_call_target/script_direct_static.rs` and the result/Recipe/
  Join/proof siblings may be borrowed as inputs only after A has one stable
  parser/source identity and one complete source window.
- The parser source handoff I0 already retains the one-shot postpass,
  profile, digest, and read/parse lineage through source planning; A is the
  next issuer boundary, not a second parser or resolver pass.

A may use a temporary `VerifiedScriptSemanticSourceV1<'source>` borrow while
issuing rows, but the canonical output must be owned and AST-free. If stable
identity or complete catalog/result ownership cannot be co-sealed without
`normal_default_root_catalog_lifecycle`, `comp_ctx`, pointer comparison, or
AST re-resolution, remain `NoSafeSlice`.

## Identity closure and remaining input gap (2026-08-21)

The parser/source identity gap described by the original A stop is closed by
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-IDENTITY-I0`. The source plan now
borrows and validates the parser-issued lineage, digest, canonical profile,
UTF-8 length, and one-read/one-parse receipt before classification; it does
not re-read or re-parse the source.

The remaining gap is semantic input ownership, not identity transport:

- `src/mir/compiler/canonical_core_dispatch.rs:526-546` reaches
  `prepare_script_recipe()` and `OpenScriptPhysicalEntryV1` without a complete
  source-only demand/window input;
- `src/mir/builder/normal_default_root_catalog_lifecycle.rs:456-665` can issue
  selected-normal target/result/Recipe/Join products, but those products are
  Builder-owned and retain temporary pointer identity;
- `src/mir/builder/normal_script_semantic_source.rs` retains the selected
  forest and direct-static siblings, but it is not a canonical source-only
  issuer;
- `src/mir/source_call_target/script_direct_static.rs` is an observation
  inventory, not a complete A input package.

Therefore the next card must name one complete input owner and lifetime before
`DirectStaticSourceReady` can become more than design vocabulary. No selected
Builder product may be copied into canonical A by pointer, path, name, ordinal,
or digest-only equality.

## Acceptance for this design stop

- the finite table above is accepted and every negative witness maps to one
  state only;
- A is the sole source observation/package issuer, C is the sole disposition
  owner, and B is transport-only; no A/C/B state is issued twice;
- parser/source identity has one named issuer and is not a path, filename,
  pointer, name, or digest-only join;
- A can name one retained Script window and one temporary forest lifetime;
- target, result, Recipe, Join, and required-proof inputs are co-sealed once;
- A's output contains no AST, `ValueId`, Builder ordinal, physical block, or
  guessed empty row;
- `CompatibilitySource`, `Deferred`, `SourceAuthorityUnavailable`,
  `ObservationIncomplete`,
  `NonCandidate`, and `IntegrityInvalid` remain distinct through the future
  request;
- a future carrier has one named consumer and one retirement/cutover edge;
- no code, fixture, source admission, physical Call, publication, production
  switch, raw retirement, or performance evidence is opened by this D0.

## NoSafeSlice conditions

Remain at this design stop if any of the following is true:

1. A can see only selected-normal Builder state and has no canonical caller;
2. source identity requires pointer, path, filename, ordinal, or name
   inference;
3. an incomplete/Deferred/authority-missing observation must be rounded to
   `NonCandidate` to keep the pipeline moving;
4. A would reparse/re-resolve the AST or issue a second semantic product;
5. B cannot receive a typed `DirectStaticSourceReady`/`NonCandidate`/
   `IntegrityInvalid` decision without an optional raw fallback;
6. the output must contain physical `ValueId`/block facts before the detached
   session exists;
7. a new source shape or source-admission rule is required, which would make
   this more than a source-only design stop; or
8. the future carrier has no canonical consumer and retirement edge.

Until all are closed, no carrier, canonical consumer, raw cutover, production
switch, or performance run is authorized.
