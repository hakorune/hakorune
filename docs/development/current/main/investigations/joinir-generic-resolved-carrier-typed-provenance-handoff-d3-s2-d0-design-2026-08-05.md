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

Only after this card is accepted and the typed products are implemented may a
bounded neutral issuer be considered:

```text
resolver observation -> AST-free neutral snapshot -> private Builder adapter
```

No selector arm, production caller, Recipe/JoinSig/PHI/physicalizer, MIR/VM
route, Retry deletion, fallback deletion, or scheduler cutover is authorized
by this card. Caller census must be zero before any later production switch.

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
