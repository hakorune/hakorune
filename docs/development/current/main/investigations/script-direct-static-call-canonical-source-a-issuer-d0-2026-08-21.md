---
Status: Accepted design stop — parser handoff D0 selected
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-ISSUER-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-input-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one canonical source-only A issuer before Script Recipe/entry
Classification: BoxCount (design only; implementation remains closed)
NextCard: docs/development/current/main/investigations/script-direct-static-call-canonical-source-parser-input-handoff-d0-2026-08-21.md
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-ISSUER-D0

## Six-line brief

Decision: Fix the single issuer and handoff needed to turn the accepted
canonical Script input contract into one AST-free source-only A package. This
D0 does not add a carrier, Recipe, Join, physical Call, or production caller.

Source authority + canonical issuer: parser-backed
`SealedNormalScriptSourceV1`, its `CanonicalParserSourceHandoffV1` lineage,
and one source-backed configuration/import snapshot are the authority. A new
sibling `CanonicalScriptDirectStaticSourceOnlyIssuerV1` is the only semantic
issuer; its private readiness state is not a second public receipt.

Non-authority: `NormalSourcePlanClassifierV1`,
`CanonicalCoreSourcePlanCompileRequestV1`, `normal_default_root_catalog_lifecycle`,
`MirBuilder`/`comp_ctx`, selected-normal direct-static products,
`RawScriptBodyRecipeV1`, AST/pointer/path/name/ordinal joins, digest-only
equality, `ValueId`/`MirType`, and compatibility success.

Fail-fast boundary: after canonical Script family and identity validation,
before `prepare_script_recipe()`, `OpenScriptPhysicalEntryV1`, Builder install,
or child effects. Missing parser-backed handoff, incomplete window/catalog/
forest/terminal coverage, foreign rows, or identity drift stops here.

Smallest next slice: docs-only issuer and handoff design. Name the source
inputs, one lifetime, phase-qualified states, exact candidate boundary, and
the sibling placement below the 760/800-line limits. No implementation is
authorized until this D0 is accepted.

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

`CanonicalSourceBacked` is an upstream identity-I0 state issued by
`CanonicalParserSourceHandoffV1`; it is not an A disposition and is never
reissued by A. Only that state may enter A observation. `ASTOnly`, typed
compatibility, and source-free upstream states remain outside A and map to
the explicit ingress rows below; no AST scan or default conversion may turn
them into `CanonicalSourceBacked`.

## Exhaustive issuer state table

| state | phase | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|---|
| `CanonicalSourceBacked` | upstream handoff | `CanonicalParserSourceHandoffV1` issues the parser-backed identity-I0 disposition | pass the exact lineage/receipt to A; no A or physical effect yet | A observation may begin | A never reissues it; other upstream states cannot default into it |
| `NotApplicable` | ingress | canonical family/profile classifier proves non-Script or outside direct-static scope | no A observation or physical effect | caller-owned non-Script or non-direct-static source owner | never fabricate `NonCandidate` or enter raw Script by absence |
| `CompatibilitySource` | ingress | parser handoff marks a compatibility cohort | preserve typed compatibility origin; do not issue A | explicit compatibility owner or parked stop | never become authority loss, `NonCandidate`, or A success |
| `Deferred` | ingress | resolver/source admission returns `Deferred` | preserve the reason; no partial A observation | deferred owner or `NoSafeSlice` | never become empty input, `NonCandidate`, or raw success |
| `SourceAuthorityUnavailable` | ingress | parser lineage, source receipt, config/import snapshot, or canonical source is absent/foreign before observation | typed stop before Recipe/entry/effects | `NoSafeSlice` until issuer input exists | no default identity, AST rescan, or Builder fallback |
| `ObservationIncomplete` | A observation | the single A issuer has authority but cannot observe the complete retained window, forest, catalogs, or terminal rows once | typed stop before A package/Recipe/entry/effects | `NoSafeSlice` until coverage is total | never round to `NonCandidate` or compatibility |
| `NonCandidate` | A observation | the single A issuer completes integrity-clean observation and every row is explicitly outside direct-static scope | no direct-static package or physical effect | canonical non-direct-static source owner | missing coverage is not absence; no raw fallback |
| `InputAuthorityReady` | A internal | the same A issuer has co-sealed every required source-bound input | private readiness only; no public semantic receipt or physical effect | continue inside A toward `DirectStaticSourceReady` | no second issuer, public carrier, or selected-normal copy |
| `DirectStaticSourceReady` | A terminal | the single A issuer co-seals target/result/sites/terminal/proof and issues one AST-free package | move-only package; no physical effect | future C consumes exactly once | no name lookup, retry, or re-pairing |
| `IntegrityInvalid` | A verification | complete observation finds duplicate, foreign, stale, mixed, missing, or contradictory rows | typed reject before Recipe/entry/child effects | terminal candidate/session discard | no retry, re-pair, `NonCandidate`, compatibility, or raw fallback |
| `Transported` | C-to-B future phase | B consumes `DirectStaticSourceReady` once | no replay or second source interpretation | detached canonical consumer terminal | no clone, return to A, or raw path |

`CanonicalSourceBacked` is an upstream admission state, not an A state.
`InputAuthorityReady` is private A-internal readiness, not a second issuer.
`Transported` is a future B state, not an A disposition. A complete zero-row
observation is `NonCandidate`; a row found but failing final Sequence/root
Return terminal validation is `IntegrityInvalid`, never `NonCandidate`.

## Issuer input and candidate boundary

The single issuer must validate/co-seal, in one source identity:

```text
parser lineage/profile/digest/read-parse receipt
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

These are phase roles, not competing issuers. `CanonicalSourceBacked` is the
upstream identity-I0 state consumed by A; it is not an A disposition. A
missing or unconfirmed source/window/config authority is
`SourceAuthorityUnavailable` or `ObservationIncomplete` before complete
observation. Only after the retained window and all required rows are
complete may a duplicate, foreign, stale, or cardinality mismatch become
`IntegrityInvalid`. `Transported` belongs to the future C-to-B lifecycle and
is not an A state. A/C/B may not collapse any of these states into an empty
candidate or compatibility success.

The issuer must not call selected `normal_default_root_catalog_lifecycle`,
install into `comp_ctx`, or borrow selected Builder products as canonical
facts. A future implementation may reuse source-neutral validation kernels
only after their authority and identity are explicit; reuse of a Builder
receipt by pointer/name is not a kernel.

## Acceptance for this design stop

- one named sibling issuer and one parser-backed source authority are fixed;
- the upstream `CanonicalSourceBacked` identity state is consumed, not
  reissued, and all other upstream states map to explicit ingress outcomes;
- the canonical callpoint is before `prepare_script_recipe()` and remains
  thin; the 748/719-line existing owners receive no semantic growth;
- all ten states above have one owner, pre-effect behavior, continuation, and
  fallback policy, including neutral `NonCandidate` and phase-only
  `Transported`;
- complete zero-candidate observation, incomplete coverage, and terminal drift
  map to three distinct states;
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

Until these are closed, the next row remains design-only and no canonical
physical consumer or production claim is allowed.
