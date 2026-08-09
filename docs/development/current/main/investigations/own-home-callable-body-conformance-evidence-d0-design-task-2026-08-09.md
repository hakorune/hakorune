---
Status: D0 accepted; bounded Call+Return I0 open
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-conformance-catalog-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-CONFORMANCE-EVIDENCE-D0

## Decision

Keep three authorities separate:

```text
VerifiedResolvedBodyShapeInventoryV1
  = neutral syntax/resolver shape and source coverage

VerifiedHomeAbiV1
  = declared receiver/parameter/result Home demands

VerifiedQueryBodyHomeFlowEvidenceV1
  = body-level proof that this bounded body performs no Home
    consume/create/end/escape
```

Do not widen `BodyEffectKindV1` with a semantic `NoHome` value. Do not use
`VerifiedHomeAbiV1` as body proof, and do not use MIR `ownership_ssa` as a
resolver source. The existing `body_shape` effect list is partial for general
control/effect vocabulary, so `effects().is_empty()` is not a universal
absence proof.

The bounded exact `return me` evidence I0 is now landed in
`query_body_conformance_evidence.rs`. That I0 is deliberately unchanged by
this card: it proves only the exact lexical receiver-read/ordinary-return
cohort and its Query Home no-transfer relation. It does not open general body
conformance.

## General body execution evidence D0 (accepted design)

General conformance needs four source-level axes and one relational aggregate.
The four axes are private receipts or borrowed views; the only new public
semantic owner is the non-`Clone`, body-root-scoped execution-evidence
aggregate. This section authorizes design only, not implementation.

```text
private BodyCoverageReceiptV1
  = exact body root/owner/provenance and complete statement/expression/
    relation coverage, including nested-owner boundaries

private BodyEffectEvidenceV1
  = source-anchored neutral effect events (write, allocation, call, IO,
    await, and the accepted effect vocabulary), with unknown/opaque rows
    rejected rather than silently omitted

private BodyControlEvidenceV1
  = source-anchored Return/Break/Continue/QMark/Throw/non-local-control,
    branch/loop transfer events, and resolved-exit target consistency

private BodyHomeFlowEvidenceV1
  = Home-flow events/state (consume/create/share/end/escape or explicit
    no-transfer) from a language Home-flow issuer; it does not reissue the
    declared Home ABI or use ownership SSA as source authority

VerifiedBodyExecutionEvidenceV1
  = one atomic co-seal of the four receipts for the same declaration, owner,
    body root, parser provenance, resolver brand, and complete coverage
```

The existing `VerifiedResolvedBodyShapeInventoryV1` is usable as a structural
coverage input, `VerifiedResolvedFunctionV1` as binding/assignment/exit
identity input, and `VerifiedSemanticOwnerForestV1` as nested-owner/upvar
isolation input. None of them currently proves the complete general axes by
itself.

The current code gaps were intentional design blockers.  The worker audit has
now accepted this D0 boundary.  They remain implementation limits for the
bounded next slice:

* `BodyEffectKindV1` is partial and does not record Print/IO or every opaque
  expression class; an empty effect vector is therefore not a no-effect
  proof.
* Return/Break/Continue are not yet co-sealed with a complete QMark/Throw/
  non-local-control and branch/loop-transfer inventory; Await belongs to the
  effect/suspension axis.
* `VerifiedHomeAbiV1` is declaration-only. A body Home event/state issuer for
  consume/create/share/end/escape is not yet available.

Do not add a semantic `NoHome` member to `BodyEffectKindV1`, copy partial
shadow vectors into a verified receipt, or derive source Home/effect meaning
from MIR `EffectMask`, `FunctionSignature`, or ownership SSA. If an axis
cannot be issued and fully covered, the development state is `NoSafeSlice`.
The first general implementation slice, after this D0 is accepted, must stay
resolver-only and must not open target, Recipe, Builder, MIR, publication, or
fallback.

## Authority table and co-seal contract

The four general receipts do not replace existing authorities and do not form
four independent semantic guesses.

| Authority | Owns | Must not own |
| --- | --- | --- |
| `VerifiedResolvedBodyShapeInventoryV1` | neutral statement/expression/relation shape and source coverage input | effect absence, Home meaning, contract/profile, Recipe key |
| `VerifiedResolvedFunctionV1` | `BindingRef`, assignment/direct-call sites, resolved exits, region identity | Home state, MIR `ValueId`/block, public contract |
| `VerifiedSemanticOwnerForestV1` | root/child/upvar and nested-owner isolation | parent-body interpretation of child facts |
| private `BodyCoverageReceiptV1` view | existing body-owner/carrier/shape/forest identity and coverage agreement | a second source-membership truth |
| private `BodyEffectEvidenceV1` view/issuer | complete source-anchored effect rows and effect coverage | Home ABI, Query/Pure selection, physical effect mask |
| private `BodyControlEvidenceV1` view/issuer | complete source-anchored control/exit rows and target consistency using resolved-exit/region authority | contract selection, CFG reconstruction by guess |
| private `BodyHomeFlowEvidenceV1` issuer | language Home events/state and explicit closed no-transfer result | declared Home ABI reissue, ownership SSA, MIR cleanup |
| `VerifiedBodyExecutionEvidenceV1` | same declaration/owner/root/provenance relation across all four receipts | any new effect, control, Home, or ABI meaning |

The aggregate is therefore relational only. It may say that the four receipts
describe the same body and complete source domain; each axis issuer must have
already proved its own vocabulary and coverage.

## Event vocabulary and coverage rules

The D0 vocabulary is source-level and neutral:

```text
effect axis:
  Write, Allocation, Call, IO, Await, FailurePropagation

control axis:
  Return, Break, Continue, QMark, Throw, NonLocalControl, branch/loop transfer

Home axis:
  Consume, Create, Share, End, Escape, Forward
```

`Await` belongs to the effect/suspension axis, not the control-exit axis.
`NoHome` is not an effect event. A no-transfer result is issued only after the
Home event domain is complete and contains no transfer rows.

For every selected body root, the future issuers must prove:

```text
every statement/expression/relation source site is covered exactly once
every effect/control/Home event points to one covered source site
unknown/opaque/unclassified sites are rejected, not omitted
resolved exit targets agree with the source owner/region
nested callable bodies and upvars are separate owner domains
no duplicate/missing/foreign event or coverage row
```

The current implementation cannot satisfy all rows: Print/IO is not yet
recorded as a neutral effect, QMark/Throw/non-local control and branch/loop
transfer are not fully co-sealed, and no language Home event issuer exists.
Those cases remain `NoSafeSlice` until their issuer and full-coverage receipt
are designed and landed. The existing owner/carrier/body-shape coverage is a
relational receipt only; it must not become a second source-membership truth.
Once an issuer exists, an opaque or unclassified site in an otherwise admitted
cohort is `Unresolved`; it is not converted into an empty effect/control/Home
set.

## Minimal next implementation slice

The D0 is accepted.  The next open row is the separate bounded implementation
card:

```text
docs/development/current/main/investigations/own-home-callable-body-effect-control-i0-implementation-task-2026-08-09.md
```

It is intentionally smaller than general execution evidence and proceeds in
this order:

```text
1. issue one private `BodyEffectControlCoverageReceiptV1` from the existing
   `ResolvedFunctionBodyShapeProductV1` only;
2. admit only exact root-direct `return me.invoke()` with one Call effect,
   one ExplicitReturn exit, and exact source relations;
3. keep unsupported/opaque/foreign shapes at private `NoSafeSlice` or
   `Rejected`, without adding a public semantic owner or widening the effect
   vocabulary;
4. leave the landed exact `return me` evidence/conformance I0 unchanged;
5. stop before Home-flow, general four-axis co-seal, conformance catalog,
   target, Recipe, Builder, MIR, publication, or fallback.
```

The first implementation slice must not add a Query-specific port, infer
contract behavior from method names, copy `Shadow*` vectors as verified facts,
or use a test-only constructor. Coverage must borrow the existing
body-owner/carrier/shape/forest identity; it is not a second source truth.
Home readiness additionally requires the Home-model SSOT's CFG-complete,
admitted-grammar, and ownership-changing witness conditions. A positive
fixture is admissible only when all four axes are available; otherwise the
correct result is development `NoSafeSlice`.

## General disposition matrix

```text
NoSafeSlice:
  required issuer/event vocabulary/coverage authority is absent, or the
  source class is unsupported/opaque for the current cohort

Unresolved:
  the accepted traversal exists, but a required source/resolver capability
  is opaque or incomplete for this body

Declined:
  all four axes are fully observed and coherent, but the declared contract
  disallows the observed effect/control/Home behavior

Rejected:
  foreign owner/brand/root, duplicate/missing/extra relation, inconsistent
  region target, ABI mismatch, or parent/child body leakage

Candidate:
  all four axes are complete, same-root, coherent, and contract-compatible
```

Required negative coverage includes foreign parser/resolver/body-root or
nested-child leakage; missing/duplicate/opaque Print/IO, call, write,
allocation, await, or other effect rows; missing or wrong branch/loop/exit
targets; omitted QMark/Throw/non-local control; foreign Home ABI/flow brand,
double consume, use-after-consume, invalid Maybe join/backedge, duplicate
share, and unknown Home state; and any co-seal cardinality/identity mismatch.

`NoSafeSlice` is a development state outside the source disposition enum. The
source precedence remains `Rejected > Unresolved > Declined > Candidate`.

## Bounded evidence product (landed I0)

The landed private, non-`Clone` bounded aggregate is:

```text
VerifiedQueryBodyConformanceEvidenceV1
  ├─ exact owner/declaration identity
  ├─ complete bounded body-shape coverage receipt
  ├─ no-prohibited-effect/control receipt for this cohort
  └─ VerifiedQueryBodyHomeFlowEvidenceV1
```

It is issued from the existing resolver owner-tree products, not a second AST
traversal. It may retain the positive `return me` facts, but it issues no
Query/Home/signature/ABI meaning and never infers a public contract from the
body. The first cohort is deliberately structural:

```text
one Return(value)
one receiver Me BindingRef
one ReturnValue relation
no other statement/expression/relation
no recorded prohibited effect/control
receiver Home demand = Handle
result Home relation = Trivial
Home transfer = 0
```

This is a narrow no-transfer receipt, not general Home Flow. It does not cover
fields, indexes, writes, calls, loops, branches, captures, nested owners,
Home-bearing parameters/results, QMark, Throw, Await, or opaque syntax.

## Owner and implementation boundary

For the bounded I0, the current evidence issuer emits this narrow no-transfer
receipt while borrowing the already sealed `VerifiedHomeAbiV1`; it does not
claim to be the general Home-flow issuer. `home_abi.rs` remains
declaration-only and `home_relation.rs` remains passive vocabulary. The future
general Home-flow sibling must have its own language event/state authority and
must not mutate or reissue the declaration ABI.

The safety side may use the same owner-tree shape inventory, but must issue an
explicit bounded coverage/absence receipt rather than treating an empty
effect vector as complete for all future syntax. If the traversal cannot
prove totality for a shape, the issuer returns development `NoSafeSlice`.

## Failure matrix

```text
issuer or complete coverage not landed -> NoSafeSlice (development)
opaque/unsupported/incomplete traversal -> Unresolved
fully observed shape outside exact cohort -> Declined
foreign owner/brand/site/Home ABI       -> Rejected
duplicate/missing body relation         -> Rejected
exact bounded no-transfer evidence      -> Candidate input
```

Nested callable owners are never folded into the parent evidence. Empty
selected Query catalogs are upstream `NoQueryDeclaration`/`NoSafeSlice`, not a
default evidence row. Non-Query rows remain unselected and receive no default
evidence.

## Bounded I0 closeout and next order

```text
1. bounded safety/coverage receipt                 [landed]
2. bounded Query Home no-transfer receipt          [landed]
3. bounded evidence co-seal                        [landed]
4. bounded CALLABLE-CONTRACT-CONFORMANCE-I0        [landed]
5. stop before target / Recipe / Builder / MIR     [enforced]
```

The next row is this card's general evidence D0 closeout. It must first fix the
four receipt authorities, event vocabulary, complete-coverage rules, and the
negative matrix. General CFG Home Flow, field/state authority, ownership SSA,
physical ABI, target, Recipe/CallSlot, publication, fallback, and production
remain closed.

## Required closeout

Before any general implementation, the D0 closeout must name the canonical
issuer for each axis, the same-brand/full-coverage checks, the source
authority for Print/IO and unsupported control, the Home event authority, and
real parser/resolver negative fixtures. The later implementation slice must
update `docs/reference/language/`, the resolved semantics README, the task map,
and `CURRENT_STATE.toml` together. No broad code or fixture is authorized by
this design stop.
