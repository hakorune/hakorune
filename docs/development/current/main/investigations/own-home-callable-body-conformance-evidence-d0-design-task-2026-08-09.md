---
Status: accepted bounded design; I0 not opened
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

## Bounded evidence product

The future private, non-`Clone` aggregate is:

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

`VerifiedQueryBodyHomeFlowEvidenceV1` is a sibling body-level receipt owned by
the resolver/Home-flow boundary. `home_abi.rs` remains declaration-only and
`home_relation.rs` remains passive vocabulary. The evidence issuer consumes
the exact body owner/facts row and the already sealed `VerifiedHomeAbiV1`; it
does not mutate or reissue either authority.

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

## I0 order after this design stop

```text
1. issue bounded safety/coverage receipt from existing owner-tree shape
2. issue bounded VerifiedQueryBodyHomeFlowEvidenceV1
3. atomically co-seal VerifiedQueryBodyConformanceEvidenceV1
4. run CALLABLE-CONTRACT-CONFORMANCE-I0 over exact selected rows
5. stop before target / Recipe / Builder / MIR
```

The first I0 must remain a private resolver receipt. General CFG Home Flow,
field/state authority, ownership SSA, physical ABI, target, Recipe/CallSlot,
publication, fallback, and production remain closed and require later design
rows.

## Required closeout

Before implementation, the I0 card must name the exact issuer methods,
same-brand/full-coverage checks, and real parser/resolver negative fixtures.
The implementation slice must update `docs/reference/language/`, the resolved
semantics README, the task map, and `CURRENT_STATE.toml` together. No code or
fixture is authorized by this D0 alone.
