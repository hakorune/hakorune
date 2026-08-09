---
Status: bounded I0 landed; general evidence remains at design stop
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

## General body execution evidence D0 (current design stop)

General conformance needs four source-level axes and one relational aggregate.
The names below are proposed boundaries; this section authorizes design only,
not their implementation.

```text
VerifiedBodyCoverageReceiptV1
  = exact body root/owner/provenance and complete statement/expression/
    relation coverage, including nested-owner boundaries

VerifiedBodyEffectEvidenceV1
  = source-anchored neutral effect events (write, allocation, call, IO,
    await, and the accepted effect vocabulary), with unknown/opaque rows
    rejected rather than silently omitted

VerifiedBodyControlEvidenceV1
  = source-anchored Return/Break/Continue/QMark/Throw/Await/non-local-control
    events and resolved-exit target consistency

VerifiedBodyHomeFlowEvidenceV1
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

The current code gaps are intentional design blockers:

* `BodyEffectKindV1` is partial and does not record Print/IO or every opaque
  expression class; an empty effect vector is therefore not a no-effect
  proof.
* Return/Break/Continue are not yet co-sealed with a complete QMark/Throw/
  Await/non-local-control inventory.
* `VerifiedHomeAbiV1` is declaration-only. A body Home event/state issuer for
  consume/create/share/end/escape is not yet available.

Do not add a semantic `NoHome` member to `BodyEffectKindV1`, copy partial
shadow vectors into a verified receipt, or derive source Home/effect meaning
from MIR `EffectMask`, `FunctionSignature`, or ownership SSA. If an axis
cannot be issued and fully covered, the development state is `NoSafeSlice`.
The first general implementation slice, after this D0 is accepted, must stay
resolver-only and must not open target, Recipe, Builder, MIR, publication, or
fallback.

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
