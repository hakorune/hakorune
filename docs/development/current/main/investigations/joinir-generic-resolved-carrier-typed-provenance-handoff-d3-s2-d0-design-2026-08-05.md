Status: accepted authority realignment; D1/P0/P1 closed docs-static-only; P2 selected
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md
Supersedes: the next-row ambiguity after D3-S1-S2
Decision: accepted D1 authority map; production remains closed
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-HANDOFF-DESIGN0-D3-S2-D0`
ParentCurrentCard: `docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md`
Exception: this compact design card is allowed because D3-S1-S2 closed the prior bounded child and the current frontier changed to a typed handoff boundary; it does not authorize production code.

# Typed provenance handoff design stop

## Purpose

D3-S1-S2 already co-seals one parsed natural-Both source, resolver
`BindingRef` obligations, and fresh V0/V1 plan observations. Its `j`, final,
and PHI label matches are corroboration only. The next boundary is therefore
not another label-backed test. It is the contract for carrying source identity
to a future neutral handoff without minting a second semantic authority.

## Authority chain

```text
parsed source
-> one FunctionSemanticResolverSessionV1
-> VerifiedResolvedFunctionV1
-> resolver-issued loop forest + exact BindingRefV1 relations
-> P0 source-site totality census
-> P1 existing source-projector/bridge closeout
-> P2 AST-free neutral carrier facts
-> P3 family disjointness
-> one Generic Recipe producer
     -> Recipe-owned LoopBindingKeyV1 + BindingRef relation
-> one non-Clone canonical plan
-> policy selector
-> canonical resolved lowerer after a separate I0
```

The resolver owns source sites, strict-ancestor relations, and `BindingRefV1`.
The current Builder-local facts extractor remains observation-only. A neutral
facts issuer may be considered only after P0/P1; it may not mint source
identity, retain `CanonicalLoopFacts`, infer provenance from names/ValueIds,
or issue Recipe keys. `LoopBindingKeyV1` is Recipe-local identity and its sole
issuer is the Recipe producer. Binding SSA owns only physical
`BindingRefV1 -> ValueId/PHI` reaching definitions.

## Candidate products to specify before implementation

1. `ResolvedCarrierObservationV1`: resolver-issued role rows containing source
   function/forest identity, exact source sites, strict-ancestor relation, and
   the relevant `BindingRefV1` identities. No diagnostic names, route IDs,
   physical `ValueId`, or AST clone is allowed.
2. `VerifiedGenericResolvedCarrierFactsV1`: neutral, AST-free facts and
   mode-neutral carrier classification after P0/P1. Its owner is the neutral
   `mir::loop_structural_facts` layer; it contains no Recipe key or policy.
3. `VerifiedGenericRecipeProductV1`: one Recipe producer co-seals Recipe,
   JoinSig, source-effect claims, and the sole `BindingRefV1 ->
   LoopBindingKeyV1` relation while issuing Recipe-local keys.
4. `VerifiedResolvedCarrierCanonicalPlanV1`: one opaque, non-forgeable,
   non-`Clone` value that co-seals facts, eligibility, Recipe product, and all
   route-affecting inputs. Its linear ownership is the invocation seal; there
   is no independent `PreflightSeedV1` or `InvocationSealV1` product.
5. A future selector consumes only that canonical plan. It does not accept a
   four-field bundle or independently pairable facts/capability/seed values.

`LoopRouteContext` remains a fragment owner only: it observes the loop
condition/body and produces CorePlan/ValueId facts. Its `fn_body` is a capture
hint, not a full FunctionDeclaration return/ABI/Home lowerer. The existing
return descent and draft finalizer remain the only return/termination owners.

## Fail-fast reject matrix

Before any Builder effect, return typed `UnresolvedStop` for:

```text
missing / foreign / ambiguous BindingRef
source, forest, frame, or canonical-plan brand mismatch
AST-bearing or ValueId-bearing neutral snapshot
missing or mismatched facts/eligibility/Recipe-key relation
planner-required V0 suppression without a typed policy receipt
Unavailable / Ambiguous / NoRecursive / unstable facts
incomplete source matrix or winner/result/Home/PHI parity
natural no-debt-to-different-winner witness not observed
```

None of these may become `LegacyPreserveExistingSchedule`, fallback, retry,
V1 precedence, or V0 suppression by implication.

## Deferred evidence rows

The following remain `NotYetObserved` or `UnresolvedStop` and must not be
fabricated with DTO mutation or failure injection:

```text
natural V0-only / Neither / duplicate-write / Program-wrapper rows
mode cross-product gaps
natural V0 post-effect debt followed by a different V1 winner
full function post-loop return/result ABI parity
Home-bearing payload and finalization meaning
```

A future cfg(test)-only row may observe full scalar return projection only if
the parsed function body actually reaches the return, V0/V1 builders are fresh
and isolated, and the return/outer-PHI relation is source-backed. Scalar
evidence must not claim Home semantics. A natural debt receipt is required for
the no-debt/different-winner check; synthetic debt is evaluator-only.

## First implementation slice after this design

The first bounded child is deliberately narrower than the final handoff
products. It is the cfg(test)-only observation task
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-OBSERVATION0-D3-S2-S0`.
It consumed the existing resolver-issued forest/frame and exact
`BindingRefV1` source relations into a private non-Clone witness. It does not
publish a production snapshot, assign a Generic `LoopBindingKeyV1`, or issue
an opaque selection input. The old statement that key and seed remain separate
design authorities is historical; D1 fixes Recipe as the sole key issuer and
folds invocation pairing into the future canonical plan.

This observation child is closed: four focused tests pass, production
caller/import is zero, and no artifact was produced. The D3-S2 design stop
remains open for the neutral snapshot, branded logical-key relation, and
opaque selection-input ownership.

The former direct-chain sketch below is superseded by D1 and retained only as
history:

```text
resolver observation -> AST-free neutral snapshot -> private Builder adapter
```

## S3 repeat-audit — closed cfg(test)-only

The bounded child was selected as
`JOINIR-GENERIC-RESOLVED-CARRIER-PROVENANCE-REPEAT-AUDIT0-D3-S2-S3`.
It is a `cfg(test)`-only observer over one private, non-`Clone` pair of
complete D3-S2-S2 provenance products from fresh resolver sessions A and B.
The pair is the sole input; loose forest/frame/role/AST/facts arguments are
not accepted.

The observer records only source-topology and outer/inner-site equality, typed
role and strict-ancestor equality, distinct resolver brands, and the fact that
raw frame coordinates may compare equal without establishing identity. The
resolver-issued brand remains the sole owner/issuer. `DirectAccum` frame
semantics remain unchanged, and no Generic snapshot/key/seed, selector,
eligibility, winner, `InvocationSeal`, Builder/MIR/Recipe/PHI, Return/Home/debt,
or production authority is introduced.

Typed mismatches reject before effects with no fallback or retry: reused/equal
brands, function/source mismatch, site/topology mismatch, role or binding
mismatch, strict-ancestor mismatch, frame-coordinate mismatch, missing or
detached products, and mixed/foreign brands. Equal raw frame coordinates with
distinct brands are a positive observation, not a reject.

The execution receipt is
`joinir-generic-resolved-carrier-provenance-repeat-audit-d3-s2-s3-task-2026-08-05.md`.
The focused provenance suite is green at 12/12 and the row remains
`cfg(test)`-only. If a future implementation needs a second issuer, loose
components, or any production/Generic selection meaning, stop and reopen this
design card; the current frontier returns here.

## Post-S3 next-row audit — NoSafeSlice

The worker premise audit checked the three plausible next rows and selected no
new execution row. Scalar full-function Return projection lacks a sealed
Return/outer-PHI sole owner and fresh isolated V0/V1 fixture. Natural
post-effect debt followed by a different winner lacks a real source-backed
debt receipt; synthetic mutation is forbidden. Generic snapshot,
`LoopBindingKey`, preflight seed, and `InvocationSeal` still lack sole-owner
contracts. Therefore the current S3 product remains the last evidence and the
frontier stays at this design stop.

Until a new design seal names the missing owner and natural fixture, do not
add an issuer, input/output product, reject enum, Builder/MIR/Recipe/PHI,
Return/ABI/Home/debt meaning, Generic key/seed/selector, fallback, retry, or
production caller. This is a NoSafeSlice disposition, not permission to infer
one of those semantics from existing labels or `ValueId`s.

## D1 authority realignment — accepted

`JOINIR-GENERIC-RESOLVED-CARRIER-AUTHORITY-REALIGNMENT0-D3-S2-D1`
supersedes the earlier D3-S2-D0 draft in these exact respects:

```text
resolver/neutral BindingRef -> LoopBindingKey projection = rejected
LoopBindingKeyV1 issuer                              = Recipe producer only
independent PreflightSeedV1                         = rejected target
independent InvocationSealV1                        = rejected target
four-field selection input                         = rejected target
one linear invocation owner                        = non-Clone canonical plan
neutral snapshot issuer before P0/P1               = 0
```

Resolver remains the sole source-site/forest/brand/`BindingRefV1` authority.
Neutral structural facts may classify only source-backed rows after P0/P1.
Policy remains in the selector. Binding SSA remains the sole physical
`BindingRefV1 -> ValueId/PHI` owner and does not mint Recipe keys. Return
coverage/emission remain with the existing completion, return statement, and
draft-finalizer owners. `LoopRouteContext` remains fragment-only.

## Ordered D3-S2 task ladder

```text
D3-S2-P0  SOURCE-SITE-TOTALITY-CENSUS0
  -> all Generic facts fields/producer arms classified against source sites

D3-S2-P1  SOURCE-PROJECTION0
  -> close/rebundle existing projector/bridge evidence; no new source issuer

D3-S2-P2  FACTS-SNAPSHOT0
  -> neutral non-Clone AST-free facts; no Recipe key/selector

D3-S2-P3  DISJOINTNESS0
  -> exact Generic/NestedPredicate/DirectAccum/A+ partition

D3-S2-S4  RECIPE-BINDING0
  -> Recipe producer issues keys and co-seals BindingRef relation

D3-S2-S5  CANONICAL-PLAN0
  -> one non-Clone plan owns facts/eligibility/Recipe/route inputs

D3-S2-I0  SELECTION-CUTOVER0
  -> selector consumes only the plan; exact natural-Both class; no fallback

D3-S2-R0  LEGACY-RETIRE0
  -> parity/caller census/reference closeout and selected old-edge deletion
```

P0 is closed by the static matrix
`joinir-generic-resolved-carrier-source-site-totality-census-d3-s2-p0-matrix-2026-08-05.tsv`.
It proves that current Generic facts fields are AST/name/derived products, not
canonical source-site/owner/`BindingRefV1` facts. The selected execution row is
now `JOINIR-GENERIC-RESOLVED-CARRIER-FACTS-SNAPSHOT0-D3-S2-P2`. P1 closed by
packaging the existing projector/source-bridge evidence in
`joinir-generic-resolved-carrier-source-projection-d3-s2-p1-matrix-2026-08-05.tsv`;
the witness remains cfg(test)/inspection-only and adds no production authority.

The scalar Return/outer-PHI authority map is a D1-dependent docs-only sidecar,
`JOINIR-GENERIC-RESOLVED-CARRIER-SCALAR-RETURN-AUTHORITY-MAP-DESIGN0-S4-D0`.
Its later fixture/projection rows require D1 plus P0 and a natural typed
full-function fixture. Natural debt-to-different-winner remains NoSafeSlice
until one unpublished candidate naturally produces V0 typed debt followed by
V1 success on the same witness. Synthetic DTO or fresh-builder substitution
does not qualify.

No selector arm, production caller, Recipe/JoinSig/PHI/physicalizer, MIR/VM
route, Retry deletion, fallback deletion, or scheduler cutover is authorized
by this card. Caller census must be zero before any later production switch.

## Consultation result: owner-brand premise must be repaired first

The independent premise audit found a gap in the current S0 witness. The
existing `VerifiedResolvedLoopSourceV1`, forest, and
`LoopExecutionFrameKeyV1` carry `FunctionOriginV1`/source coordinates, but not
the resolver-issued `FunctionOwnerIdV1` or an invocation/compilation brand.
Two resolver sessions can therefore have the same coordinate and equal
ownerless frame shape. A forest from session A can be paired with roles and a
frame from session B without the current S0 checks rejecting it. The S0 result
is still useful as same-product observation evidence, but it is not an
owner-branded cross-invocation capability.

The selected next child is therefore a design-gated premise repair, not a
Generic snapshot or key implementation:

```text
JOINIR-GENERIC-RESOLVED-CARRIER-CROSS-SESSION-BRAND-AUDIT0-D3-S2-S1
```

Its execution brief is
`joinir-generic-resolved-carrier-cross-session-brand-audit-d3-s2-s1-task-2026-08-05.md`.
The worker premise audit accepted this as a cfg(test)-only slice: it may
repair the handoff witness and adversarially verify cross-session rejection,
but it may not connect a production issuer or selector.

The S1 child is now closed. Its five focused provenance tests pass, including
the adversarial `forest_A + roles_B + frame_B` witness. The private wrappers
attach the resolver owner to the handoff-only forest/frame pair while leaving
the ownerless structural frame and all DirectAccum consumers unchanged. The
implementation has no production caller/import or artifact. The next row is
not selected here; a new design seal is required before a passive provenance
product may be opened.

Recommended shape (to avoid changing existing DirectAccum repeat semantics):

```text
existing structural LoopExecutionFrameKeyV1
+ non-Clone resolver-issued owner/issuer brand
-> Generic handoff-only branded source/frame capability
```

The brand must co-seal forest root, frame, and every `BindingRefV1` role. A
mandatory adversarial witness is `forest_A + roles_B + frame_B` for two fresh
sessions resolving the same source; it must reject with an owner-brand or
invocation mismatch. `LoopRecipeSourceOwnerV1`, `LoopRecipeSourceBindingV1`,
`NormalizedBindingKeyV1`, route IDs, names, AST paths, and physical `ValueId`
are not brands and remain non-authoritative for this boundary.

Only after that premise is accepted may the following passive product be
selected inside this same D3-S2 card:

```text
JOINIR-GENERIC-RESOLVED-CARRIER-PROVENANCE-PRODUCT0-D3-S2-S2
```

It would move the S0 observation into a source-only, private non-Clone
`resolved_semantics` product. It would still publish no
`GenericCarrierFactsSnapshotV1`, `LoopBindingKeyV1`, preflight seed,
`InvocationSeal`, opaque selection input, selector, Builder/MIR/Recipe/PHI,
Return/ABI/Home/debt semantics, or production caller. If the brand cannot be
sealed without introducing one of those owners, execution returns to this
design stop.

## S2 candidate contract — design only

The smallest next product is a private, non-`Clone` AST-free provenance
product owned by the neutral `mir::resolved_semantics` boundary. Its producer
consumes one S1-branded handoff as a single value; it does not accept loose
forest, frame, role, or facts references:

```text
S1 branded handoff
  -> ResolvedCarrierProvenanceProductV1
```

The product may retain only the resolver-owned function/source kind, outer and
inner source sites, parent membership, exact `BindingRefV1` role rows and
strict-ancestor relation, and the S1 owner/frame brand. It must not retain AST,
`CanonicalLoopFacts`, labels, route IDs, plan digests, `ValueId`, `BasicBlockId`,
PHI, Return/ABI/Home/debt facts, mode/seed flags, or Generic logical keys.
The product has no public constructor or detachable `parts()` accessor; a
future consumer must consume it as one sealed value.

The product issuer rejects before any Builder effect when the handoff is
missing, foreign, ambiguous, mixed-brand, duplicate-role, incomplete forest,
shadowed/foreign `BindingRefV1`, or frame/source identity is inconsistent.
No typed `Option<Capability>`, legacy fallback, retry, or selector policy may
represent these failures.

This candidate is now selected as the bounded execution row
`JOINIR-GENERIC-RESOLVED-CARRIER-PROVENANCE-PRODUCT0-D3-S2-S2`. The existing
resolved-semantics README remains the owner/issuer boundary, and the execution
must add a cfg(test)-only positive/mixed/negative matrix before any later
design row is opened. Generic snapshot/key/seed, opaque selection input, and
every production caller remain closed.

The S2 child is now closed as cfg(test)-only evidence. Its private factory
consumes one co-sealed resolver handoff, rejects the typed mismatch matrix
before effects, and publishes no production capability or selector input.
The frontier returns to this design stop for the next ownership decision.

## Acceptance

Design acceptance requires the parent D3 card, D3-S1 disposition matrix,
Generic post-effect/stage references, resolved-semantics README, current state,
10-Now mirror, and active workstream to point to this card. A later
implementation closeout must update language/reference documents in the same
commit, with focused evidence, caller census, artifact manifest, fail-fast
boundary, and every touched source/check file below 800 lines.

```text
typed provenance owner                    = 1
neutral snapshot owner before P0/P1      = 0
LoopBindingKey issuer                    = Recipe producer only
independent seed / InvocationSeal target = 0
selector/issuer caller                    = 0 until a later I0
label/ValueId inference                   = 0
synthetic debt witness                    = 0
production cutover                        = 0
```
