---
Status: Design stop — complete-observation prerequisite selected
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-ISSUER-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-input-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one canonical source-only A issuer before Script Recipe/entry
Classification: BoxCount (design only; implementation remains closed)
NextCard: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-observation-d0-2026-08-21.md
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-ISSUER-D0

## Six-line brief

Decision: Fix the single issuer and handoff needed to turn the accepted
canonical Script input contract into one AST-free source-only A package. This
D0 does not add a carrier, Recipe, Join, physical Call, or production caller.

Source authority + canonical issuer: the phase-qualified co-seal of parser
admission, `HandoffReady`, and the matching parser/profile/receipt identity is
the upstream authority. A new sibling
`CanonicalScriptDirectStaticSourceOnlyIssuerV1` is the only A semantic issuer;
its private readiness state is not a second public receipt.

Non-authority: `NormalSourcePlanClassifierV1`,
`CanonicalCoreSourcePlanCompileRequestV1`, `normal_default_root_catalog_lifecycle`,
`MirBuilder`/`comp_ctx`, selected-normal direct-static products,
`RawScriptBodyRecipeV1`, AST/pointer/path/name/ordinal joins, digest-only
equality, `ValueId`/`MirType`, and compatibility success.

Fail-fast boundary: after canonical Script family and identity validation,
before `prepare_script_recipe()`, `OpenScriptPhysicalEntryV1`, Builder install,
or child effects. Missing parser-backed handoff, incomplete window/catalog/
forest/terminal coverage, foreign rows, or identity drift stops here.

Smallest next slice: the accepted state matrix is closed; the new
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-D0` now fixes the
complete source/window/forest/target/proof/terminal observation boundary
before any issuer implementation is authorized.

Non-claims: no parser grammar/source admission change, A/C/B code, Recipe/Join,
physical Call/publication, Return/signature, canonical production switch,
compatibility/raw retirement, ABI/backend/performance, or selected-normal
cutover.

## Why a separate issuer is required

The current canonical path is:

```text
parser/source handoff
  -> source identity validation
  -> NormalSourcePlanClassifier
  -> SealedNormalScriptSourceV1
  -> prepare_script_recipe()
  -> OpenScriptPhysicalEntryV1
```

`SealedNormalScriptSourceV1` retains source AST/statement sites and parser
lineage, but it does not own the complete Script demand window, declaration
and Brand views, import/config snapshot, resolver forest, target/result rows,
or terminal relation that A needs. Those pieces are currently assembled in
the selected Builder lifecycle. Reusing that lifecycle as canonical authority
would make Builder state a second source issuer and would pair temporary
pointer identity across phases.

The callpoint for a future issuer is the thin boundary immediately before
`prepare_script_recipe()` in `canonical_core_dispatch::compile_script`. The
748-line dispatch file and the 719-line selected lifecycle remain no-growth
owners; semantic issuance belongs in a new sibling module. If the parser
handoff cannot provide a required source-backed row, this card remains
`NoSafeSlice` and opens a parser-handoff prerequisite rather than inventing a
default or borrowing `comp_ctx`.

## Upstream identity boundary

`CanonicalSourceBacked` is not a standalone code state or issuer. In the
state-matrix P0 it is only a shorthand for the co-seal
`CanonicalScriptCohortAdmitted + HandoffReady + matching parser/profile/
receipt identity`. A lone admission, identity receipt, compatibility product,
or source-free product cannot enter A observation. The matrix card owns the
upstream and transport rows; this card owns only the A rows below.

## Exhaustive A issuer state table

The upstream parser and transport states are owned by the companion
`SCRIPT-DIRECT-STATIC-SOURCE-A-STATE-MATRIX-P0` card. `A.NotApplicable`,
`A.CompatibilitySource`, and `A.Deferred` are preserved upstream labels, not
new A issuances. After `HandoffConsumed`, only the observation rows below are
reachable by the future sibling issuer:

| state | phase | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|---|
| `A.NotApplicable` | A ingress | upstream parser/transport owner; preserved, not issued by A | no A observation or physical effect | caller-owned non-direct-static source owner | never fabricate an A result or enter raw Script |
| `A.CompatibilitySource` | A ingress | upstream typed compatibility owner; preserved, not issued by A | no A observation | compatibility owner or parked stop | never become A success |
| `A.Deferred` | A ingress | upstream deferred owner; preserved, not issued by A | no partial A observation | deferred owner or `NoSafeSlice` | never become empty input or raw success |
| `A.SourceAuthorityUnavailable` | A ingress | co-sealed parser admission/handoff/identity is absent or foreign | stop before package/Recipe/entry/effects | `NoSafeSlice` until authority exists | no default identity, AST rescan, or Builder fallback |
| `A.ObservationIncomplete` | A observation | authority exists but retained window, forest, catalog, proof, or terminal coverage is missing/gapped | stop before package/Recipe/entry/child effects | `NoSafeSlice` until coverage is total | never round to `A.CompleteNoDirectStaticRows` or compatibility |
| `A.CompleteNoDirectStaticRows` | A private observation witness | complete integrity-clean observation proves every retained row is outside direct-static scope | no direct-static package or physical effect; C may classify later | future C owner | missing coverage is not absence; no raw fallback |
| `A.InputAuthorityReady` | A internal | the sole A issuer co-seals all required source-bound inputs | private readiness only; no physical effect | `A.DirectStaticSourceReady` or `A.IntegrityInvalid` | no second issuer/public receipt/selected copy |
| `A.DirectStaticSourceReady` | A terminal | the sole A issuer co-seals target/result/sites/proof/terminal and issues one AST-free package | move-only package; no physical effect | future C consumes once | no name lookup, retry, or re-pairing |
| `A.IntegrityInvalid` | A verification | complete observation finds present foreign, duplicate, stale, mixed, or contradictory rows | reject before Recipe/entry/child effects | terminal candidate/session discard | no retry, re-pair, `A.CompleteNoDirectStaticRows`, compatibility, or raw fallback |

The boundary is exact: missing expected rows or coverage gaps are
`A.ObservationIncomplete`; a present row with invalid identity or conflicting
membership is `A.IntegrityInvalid`; only complete clean zero-row observation
is the private `A.CompleteNoDirectStaticRows` witness. Public
`C.NonCandidate` belongs to the future C disposition owner and is not issued
by A. `A.InputAuthorityReady` is private and `A.DirectStaticSourceReady` is
the only public A package. Future C/B states are not re-used as A states.

## Issuer input and candidate boundary

The single issuer must validate/co-seal, in one source identity and one
move-only source envelope:

```text
parser invocation brand + parser lineage/profile/digest/read-parse receipt
matching source-plan identity and retained HandoffReady carrier
complete retained Script ProgramBody window and coverage
declaration facts + Brand catalog + canonical import/config snapshot
one resolver forest for that exact window
target inventory with explicit noncandidate reasons
result representation and ordered receiver/argument/result sites
required-callee-argument proof
final Sequence/root Return terminal relation
```

`DirectStaticSourceReady` requires all of the following:

```text
parser-backed canonical ScriptRoot
complete window and resolver forest
resolved semantic disposition
Qualified(ProvenUnbound) receiver
typeop/reserved route = Ordinary
canonical static target + declaration/arity/result exact match
ordered source sites and terminal relation exact
complete target/result/proof cardinality
```

Representation such as `ExactI64` remains a source fact in this package;
physical eligibility is a later consumer decision. `FunctionOwnerIdV1` may
validate a resolver forest internally but is invocation-local and cannot be
the source identity. No AST, pointer, `ValueId`, MIR block, Builder ordinal,
physical instruction, or Recipe key may cross the A package boundary.

## A/C/B ownership

```text
A = source-observation/package phase
  one parser-backed observation, one private readiness, one
  DirectStaticSourceReady package

C = canonical-disposition phase
  consumes A's package once and is the sole owner of the direct-static
  semantic disposition; C does not re-observe parser source

B = typed-transport phase
  carries C's typed decision only; B does not observe source or reissue A/C
  meaning
```

These are phase roles, not competing issuers. The upstream matrix defines the
co-seal that permits A ingress; no standalone `CanonicalSourceBacked` state is
reissued. A missing or unconfirmed source/window/config authority is
`A.SourceAuthorityUnavailable` or `A.ObservationIncomplete` before complete
observation. Only after the retained window and all required rows are
complete may a present duplicate, foreign, stale, or contradictory row become
`A.IntegrityInvalid`. C/B transport names belong to their own phase and are
not A states. A/C/B may not collapse any state into an empty candidate or
compatibility success.

The issuer must not call selected `normal_default_root_catalog_lifecycle`,
install into `comp_ctx`, or borrow selected Builder products as canonical
facts. A future implementation may reuse source-neutral validation kernels
only after their authority and identity are explicit; reuse of a Builder
receipt by pointer/name is not a kernel.

## Acceptance for this design stop

- one named sibling issuer and one parser-backed phase-qualified source
  authority are fixed;
- the companion state-matrix P0 maps every upstream and transport state, with
  no standalone `CanonicalSourceBacked` alias or reissuer;
- the observation D0 distinguishes complete zero-row observation from missing
  target/window/forest/proof coverage and identifies source-admission expansion
  as a separate design decision;
- parser invocation brand, source-plan identity, and HandoffReady carrier are
  co-sealed before observation; duplicated receipt/digest/profile fields cannot
  be re-paired after the boundary;
- the canonical callpoint is before `prepare_script_recipe()` and remains
  thin; the 748/719-line existing owners receive no semantic growth;
- every A state above and every upstream/transport state in the companion
  matrix has one owner, pre-effect behavior, continuation, and fallback
  policy; missing and present-invalid rows remain distinct;
- complete zero-candidate observation (`A.CompleteNoDirectStaticRows`),
  incomplete coverage, and terminal drift map to three distinct states;
- public `C.NonCandidate` is issued only by future C after consuming the
  private A zero-row witness; A does not reissue C dispositions;
- the issuer input list has one lifetime and no pointer/name/digest-only join;
- `InputAuthorityReady` is private and `DirectStaticSourceReady` is the only
  public A package;
- A package contains no AST, `ValueId`, MIR/Builder physical fact, or Recipe
  key, and C/B do not reissue source meaning;
- no code, fixture, source admission, fallback, physical, production, or
  performance change is opened by this D0.

## NoSafeSlice conditions

Remain at this D0 if any condition holds:

1. canonical input still requires selected Builder/`comp_ctx` state;
2. parser handoff cannot provide complete window, declarations/Brand/imports,
   resolver forest, target/result rows, proof, or terminal relation;
3. any required row can only be paired by pointer, path, name, ordinal,
   filename, or digest equality;
4. `NonCandidate` would mean missing/empty observation rather than complete
   explicit coverage;
5. A/C/B issue the same source meaning more than once;
6. `InputAuthorityReady` must become a second public semantic receipt;
7. A implementation needs AST rescan, second resolver pass, raw fallback,
   source admission expansion, or production switch;
8. the issuer would require semantic growth in a source already at the
   760-line design trigger or the 800-line hard stop.
9. the companion phase-qualified state matrix is not accepted, or any
   actual enum variant lacks an owner/transition row;
10. the complete-observation D0 cannot name a source owner for every required
    window/forest/catalog/proof/terminal input;
11. plan, parser invocation brand, and HandoffReady are only matched by
    duplicated receipt/digest/profile fields rather than one source envelope;
12. A is required to issue public `C.NonCandidate`, or any upstream
    `NotApplicable`/`CompatibilitySource`/`Deferred` state is reissued by A.

Until these are closed, the next row remains design-only and no canonical
physical consumer or production claim is allowed.
