---
Status: Design stop — complete-observation owner required
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-issuer-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: HandoffReady -> complete source observation before A package
Classification: BoxCount (new source-observation contract; no implementation)
NextCard: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-SOURCE-ENVELOPE-I0
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-D0

## Six-line brief

Decision: Define the one source-only observation owner that can prove a
canonical Script window is complete before the A issuer may emit a package;
do not treat the parser carrier or an empty target catalog as that proof.

Source authority + canonical issuer: one move-only source envelope co-seals
the parser cohort admission, parser source rows and invocation brand,
source-plan identity, `HandoffReady` carrier, and retained
`SealedNormalScriptSourceV1` lineage/receipt. A future
`CanonicalScriptDirectStaticSourceOnlyIssuerV1` is the sole issuer of the
phase-qualified A observation.

Non-authority: parser syntax rows alone, duplicated source identity/digest/
profile/count fields, `NormalSourcePlanClassifierV1`, an empty callable/target
catalog, selected Builder/`comp_ctx`, AST/pointer/name/ordinal/digest pairing,
`Recipe`/`Join`, `ValueId`/`MirType`, and the existing Script recipe are not
complete-observation authorities.

Fail-fast boundary: consume `HandoffReady` exactly once at `compile_script()`
before `discard_before_a_consumer()`, `prepare_script_recipe()`, physical
entry, Builder state, or child effects. Verify the source-brand/plan relation
in that same envelope. Missing window/forest/catalog/proof/terminal coverage
is `A.ObservationIncomplete`; present foreign/duplicate/stale/contradictory
rows are `A.IntegrityInvalid`.

Smallest next slice: open the transport-only child
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-SOURCE-ENVELOPE-I0` to co-seal
the source envelope and close the parser/compiler state boundary; only after
that child is green may
`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-OBSERVATION-I0` implement the
complete-observation issuer. Static target admission, if needed, is a separate
D0 and cannot be smuggled into either I0.

Non-claims: no A package implementation in this D0, no new Script source
admission, no static target acceptance, no public `C.NonCandidate` issuance,
no Recipe/Join/physical Call, publication, Return/signature,
compatibility/raw retirement, production switch, ABI/backend, or performance
claim.

## Why the issuer D0 needs this child boundary

The parser-to-compiler carrier is now transported linearly, but it carries
only parser rows, source identity/profile/read-parse evidence, and complete
import/config syntax. The source-plan identity and parser invocation brand are
not yet co-sealed with the carrier at the A boundary as one
non-reconstructible source envelope. It also does not contain a resolver
forest, complete retained Script window, target/result rows, required-argument
proof, or final terminal relation. The current `compile_script()` therefore
still closes the carrier at `DiscardedBeforeA` and then enters the existing
Script recipe. That is the correct no-consumer behavior, not evidence that A
can observe a zero set.

The current canonical Script admission is effectively a pure Script family
(top-level Box/function/import surfaces are not admitted). Consequently an
empty static declaration catalog cannot prove
`A.CompleteNoDirectStaticRows` for a source that contains a static call. A
target outside the catalog is an observation gap until a source-owned
inventory explains the full window.

## Carrier integrity and legacy-route boundary

The current parser/frontdoor carrier and `CanonicalCoreSourcePlanCompileRequestV1`
still expose source identity, profile, receipt, count, parser invocation brand,
and plan identity through separate projections. This D0 therefore requires a
future source-envelope co-seal before A observation; matching duplicated
primitive fields at `compile_script()` is not sufficient. The existing
parser/frontdoor `HandoffConsumed` spelling is an internal embedding/forwarding
sentinel in some owners, not proof that an A consumer has consumed the carrier.
Only a future compiler-owned A transition may issue the phase-qualified
`compiler.HandoffConsumed` edge.

The child I0 must also remove the remaining type-level ambiguity: the parser
product currently uses a `HandoffConsumed` spelling for its internal move into
the parallel callable handoff, while the compiler transport reserves the same
spelling for a future named A consumer. A guard exception is not enough. The
parser-internal move must have a private embedding marker (for example,
`MovedToParallelHandoff`), and only the compiler A-consumer owner may issue
`compiler.HandoffConsumed`.

The old route is the named
`SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001` owner. Before A exists,
`HandoffReady -> DiscardedBeforeA` is the explicit no-A terminal and the old
Recipe may continue under its current owner. Once A observation opens,
`HandoffReady` must branch either to the named A consumer or to an explicit
no-A terminal; `ObservationIncomplete` and `IntegrityInvalid` may never fall
back into the old Recipe or raw Script route.

## Phase-qualified observation table

| state | sole issuer / authority | pre-effect behavior | terminal / continuation | fallback |
|---|---|---|---|---|
| `A.SourceAuthorityUnavailable` | parser/carrier co-seal | stop before observation | `NoSafeSlice` | no default identity/rescan |
| `A.ObservationIncomplete` | observation issuer with identity but missing window/forest/catalog/proof/terminal coverage | stop before Recipe/entry/effects | design stop or separate admission D0 | never `A.CompleteNoDirectStaticRows`/raw |
| `A.CompleteNoDirectStaticRows` | same issuer after complete clean zero-row observation; private witness | no direct-static package/effect | future C owner may issue public `C.NonCandidate` | only after total coverage |
| `A.InputAuthorityReady` | same issuer, private co-seal checkpoint | no physical effect | `A.DirectStaticSourceReady` or invalid | no public second receipt |
| `A.DirectStaticSourceReady` | same issuer after complete rows/proof/terminal | move one A package | future C consumer | no name lookup/retry |
| `A.IntegrityInvalid` | same issuer with present foreign/duplicate/stale/contradictory rows | reject before Recipe/entry/effects | discard terminal | no repair/re-pair/fallback |

The decisive boundary is:

```text
missing expected row / window gap / unavailable observer -> ObservationIncomplete
present row with foreign, duplicate, stale, or contradictory identity
  -> IntegrityInvalid
complete clean observation with zero direct-static rows -> private CompleteNoDirectStaticRows
complete rows + proof + terminal -> DirectStaticSourceReady
```

`A.CompleteNoDirectStaticRows` is not a synonym for “this pure Script
currently has no declared static target.” It requires an issuer-owned census
of every retained source site and an explicit reason for each noncandidate row.
The public `C.NonCandidate` disposition belongs to the future C owner and may
be issued only after it consumes this private witness.

## Required co-sealed observation inputs

The future issuer must receive one source-bound lifetime containing all of:

```text
parser invocation brand + parser profile/digest/read-parse lineage
matching source-plan identity and retained HandoffReady carrier
retained ProgramBody window and total coverage
declaration, Brand, import, and config views
resolver forest for the same source identity
target inventory with explicit noncandidate reasons
result representation and ordered receiver/argument/result sites
required-callee-argument proof
FinalSequence / RootReturn terminal relation
```

Existing products may be borrowed as validation kernels only when the issuer
revalidates their source identity and emits its own AST-free rows. Pointer,
name, ordinal, path, digest, Builder state, or `ValueId` equality cannot pair
rows after the source boundary.

## Source admission split

This D0 does not decide whether a Script containing a static declaration or a
static call is admitted. If complete observation proves that the current pure
Script family cannot represent that shape, open a separate
`SCRIPT-DIRECT-STATIC-CANONICAL-SOURCE-ADMISSION-D0` with its own source
authority and accepted-shape decision. The observation I0 may then consume
that source product; it must not manufacture one from an empty catalog.

## Acceptance and NoSafeSlice

Accept this D0 only when:

- one named source-only observation issuer and one HandoffReady consumer are
  fixed;
- parser invocation brand, source-plan identity, and HandoffReady are co-sealed
  before observation rather than paired from duplicate receipt fields;
- all six A observation states above have an owner, pre-effect rule, terminal/continuation,
  and fallback policy;
- complete zero-row observation is a private A witness, while public
  `C.NonCandidate` remains owned by future C; missing coverage and present-invalid
  rows are distinct;
- the carrier remains parser/source evidence only and is never treated as a
  semantic A package;
- source admission expansion, if necessary, is a separate D0;
- no code, fixture, Recipe, Join, physical, fallback, production, or
  performance change is opened.

Remain at `NoSafeSlice` if any required window/forest/catalog/proof/terminal
row has no source owner, if plan/rows/brand/HandoffReady are paired only by
duplicated receipt or digest fields, if pure Script cannot issue complete
zero-row proof, if a static target is paired by name/ordinal/digest, if A is
asked to issue public `C.NonCandidate`, or if A would need to fall back to the
old Script recipe after an observation error.

## External review reconciliation

The selected-normal direct-static bridge and its claim-ingress fail-fast are
already Keeper rows; this D0 does not reopen them. The parser/frontdoor carrier
I0 is also complete and remains transport-only. The five parallel direct-static
`Option` products are a later `SCRIPT-DIRECT-STATIC-SEMANTIC-PACKAGE-COSEAL-D0`
cleanup row in the parked MirBuilder map, and the 819-line `builder.rs` barrel
is covered by the existing `MIRBUILDER-ROOT-TEST-TAIL-SPLIT-P0` row after
production cutover. Neither cleanup concern is a source-observation authority
or a reason to widen this A row. The existing classification guard remains
green at its current P1 scope; its all-phase/transition coverage expansion is
parked as `ROUTING-CLASSIFICATION-COMPLETENESS-GUARD-P2`, not an A blocker.
Earlier parser-input/carrier D0 prose that uses a bare `Transported` or treats
`HandoffConsumed` as a single phase-free state is superseded by the accepted
phase-qualified matrix and is not an issuer authority.

## Child task selected before A observation

`SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-SOURCE-ENVELOPE-I0` is the only
next implementation candidate. It is a BoxShape transport/integrity slice,
not a new semantic source product: the envelope aggregates already-issued
parser/frontdoor evidence and source-plan relation evidence, then moves once
into the current no-A boundary. The parked parallel `Option` package cleanup,
the 819-line `builder.rs` barrel split, and the all-phase routing guard
expansion remain separate tasks and must not be folded into this child.
