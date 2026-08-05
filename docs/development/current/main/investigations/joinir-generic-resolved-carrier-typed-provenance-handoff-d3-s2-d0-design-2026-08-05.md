Status: accepted design stop — no implementation
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md
Supersedes: the next-row ambiguity after D3-S1-S2
Decision: accepted for design consultation only
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
-> AST-free GenericCarrierFactsSnapshotV1
-> one sealed preflight/selection input
-> private Builder adapter (only after this design is accepted)
```

The resolver owns source sites, strict-ancestor relations, and `BindingRefV1`.
The current Builder-local facts extractor remains observation-only. A neutral
issuer may publish an AST-free snapshot, but it may not mint source identity,
retain `CanonicalLoopFacts`, or infer provenance from names/ValueIds.

## Candidate products to specify before implementation

1. `ResolvedCarrierObservationV1`: resolver-issued role rows containing source
   function/forest identity, exact source sites, strict-ancestor relation, and
   the relevant `BindingRefV1` identities. No diagnostic names, route IDs,
   physical `ValueId`, or AST clone is allowed.
2. `GenericCarrierFactsSnapshotV1`: neutral, AST-free facts and mode-neutral
   carrier classification. Its owner is the neutral `mir::loop_structural_facts`
   layer, not the selector or Builder.
3. `LoopBindingKeyV1` relation: a typed, one-way projection from resolver
   `BindingRefV1` to a logical loop binding key. The Builder-owned Binding SSA
   and physical `ValueId`/PHI relation are downstream products; labels cannot
   stand in for this relation.
4. `InvocationSealV1` / preflight seed: non-`Clone`, one-request identity that
   prevents independently pairing facts, capability, route order, or stale
   Builder state.
5. `VerifiedResolvedCarrierSelectionInputV1`: opaque, non-forgeable,
   non-`Clone` wrapper consumed as one unit by a future selector/adapter.

`LoopRouteContext` remains a fragment owner only: it observes the loop
condition/body and produces CorePlan/ValueId facts. Its `fn_body` is a capture
hint, not a full FunctionDeclaration return/ABI/Home lowerer. The existing
return descent and draft finalizer remain the only return/termination owners.

## Fail-fast reject matrix

Before any Builder effect, return typed `UnresolvedStop` for:

```text
missing / foreign / ambiguous BindingRef
source, forest, or frame mismatch
AST-bearing or ValueId-bearing neutral snapshot
missing or mismatched InvocationSeal/preflight seed
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
It consumes the existing resolver-issued forest/frame and exact
`BindingRefV1` source relations into a private non-Clone witness. It does not
publish a production snapshot, assign a Generic `LoopBindingKeyV1`, or issue
an opaque selection input. Those three authorities remain separate design
work because Generic key assignment and seed ownership are not yet defined.

This observation child is now closed: four focused tests pass, production
caller/import is zero, and no artifact was produced. The D3-S2 design stop
remains open for the neutral snapshot, branded logical-key relation, and
opaque selection-input ownership.

Only after this observation child and a follow-up design seal may a bounded
neutral issuer be considered:

```text
resolver observation -> AST-free neutral snapshot -> private Builder adapter
```

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

## Acceptance

Design acceptance requires the parent D3 card, D3-S1 disposition matrix,
Generic post-effect/stage references, resolved-semantics README, current state,
10-Now mirror, and active workstream to point to this card. A later
implementation closeout must update language/reference documents in the same
commit, with focused evidence, caller census, artifact manifest, fail-fast
boundary, and every touched source/check file below 800 lines.

```text
typed provenance owner = 1
neutral snapshot owner = 1
selector/issuer caller  = 0 until a later I0
label/ValueId inference = 0
synthetic debt witness  = 0
production cutover      = 0
```
