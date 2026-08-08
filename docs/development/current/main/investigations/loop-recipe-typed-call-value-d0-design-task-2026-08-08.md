---
Status: Design stop
Date: 2026-08-08
Decision: `NoSafeSlice` — close the neutral typed call/value prerequisite before S6C implementation
Scope: M8 LoopV0 scan ingress; one profile-neutral Recipe vocabulary, no physical activation
---

# LOOP-RECIPE-TYPED-CALL-VALUE-D0

## Current Capsule

- **Current decision:** S6C scans cannot issue a truthful portable Recipe with
  the current numeric-only `LoopRecipeV1` vocabulary.
- **Current implementation status:** no schema, observer, producer, Builder,
  or physicalizer change is landed for S6C.
- **Next ordered task:** design and accept one neutral typed call/value
  contract, then implement only forward `ScanWithInit`.
- **Production stop line:** no scan selector, physical route, fallback, or
  production caller is opened by this design row.
- **Retirement finish line:** after a real S6C implementation and parity,
  update the reference contract in that same implementation commit; legacy
  scan facts/builders remain until an explicit cutover row deletes them.

## Change

Keep S6C as a design stop and split the work into two bounded rows:

1. this prerequisite: a profile-neutral typed call/value vocabulary and its
   source/effect ownership;
2. a later `ScanWithInit` source observer/producer using that vocabulary.

Do not combine `ScanWithInit`, `SplitScan`, `CharMap`, `ArrayJoin`, and
`BoolPredicateScan` into one Facts union or one implementation row.

## Evidence and boundary

The existing `LoopOperationV1` admits only `ReadBinding`, `ConstI64`,
`BinaryI64`, `CompareI64`, and `WriteBinding`; `LoopValueClassV1` admits only
`I64`, `Bool`, and `Unit`. The forward scan fixture
`apps/tests/scan_with_init_ok_min.hako` requires typed string values and calls
for `length` and `substring`, a typed string equality, a loop return, and a
callable tail return. The old scan builders reconstruct AST and therefore are
not portable source authority.

This is a BoxShape/schema gap, not a source `Unresolved` outcome. Exact Recipe
counts are intentionally not published until the vocabulary is accepted.

## Contract

The prerequisite must define one profile-neutral operation/value family with
these properties:

1. **Typed values:** preserve existing numeric values and add the smallest
   explicitly named text value domain needed by forward `ScanWithInit`.
   Array/collection values are a later, separate design; do not smuggle them
   in as `Opaque`.
2. **Typed call leaf:** one neutral call operation carries a resolver-issued,
   source-bound callable target, optional receiver, ordered argument values,
   result value/class, and the sealed effect/Home contract. A method name or
   runtime lookup string is never the authority.
3. **Allowed first profile:** calls must be exact, non-suspending, non-control,
   and admitted by the existing callable/type contract. Missing signature,
   unsupported effect, ownership mismatch, or unknown result class freezes
   before Recipe/Core publication.
4. **Typed comparison:** the vocabulary must represent text equality without
   importing the If-only direct-call schema. Numeric comparison remains the
   existing operation family; cross-domain coercion is rejected.
5. **Input ownership:** string parameters, the character parameter, and the
   initialized index are explicit input-source relations. Do not create a
   scan-specific second input owner; extend the existing typed input relation
   family only after its parameter/initializer distinction is sealed.
6. **Control/tail boundary:** `return i` is a logical loop `Return` exit when
   the Loop algebra owns it. The outer `return -1` remains callable Tail and
   completion evidence. Neither tail nor ABI is absorbed into Loop Facts.
7. **Source/effect evidence:** each call, typed comparison, input, read/write,
   and exit has an exact resolver source site, owner/frame brand, and effect
   relation. Facts retain semantic roles and `BindingRef`; producers alone
   mint Recipe keys. No AST reread or synthetic AST is allowed.

The exact enum/wire names and normalized item/value counts are part of the
typed-call D0 acceptance. They must not be guessed in the S6C implementation
card. If the contract cannot be made neutral and reusable, keep S6C parked.

## First implementation slice after D0

Only the forward `ScanWithInit` fixture is eligible. Its semantic roles are:

```text
inputs: string subject, text needle, initialized i64 index
condition: read i; subject.length(); i < length
body: read i; compute i+1; subject.substring(...); text equality;
      conditional Return(i)
step: read i; add one; write i
tail: callable Return(-1), outside the Loop Recipe
```

This role list is not a Recipe golden and supplies no physical IDs. The later
families are separate rows: `SplitScan` needs explicit-else joins and mutable
start/collection effects; `BoolPredicateScan` needs predicate-call contracts;
`CharMap` and `ArrayJoin` need their own text/collection operation contracts.

## Disposition matrix

| Outcome | Boundary |
| --- | --- |
| `Candidate` | Exact resolver call targets, typed values/effects, source coverage, control/tail split, and ownership all seal together. |
| `Declined` | Fully observed forward source, but wrong step/operator/direction, extra effect, nested shape, or unsupported family policy. |
| `Unresolved` | Source site, callable signature, result class, Home/effect relation, or complete coverage is unavailable. |
| `Rejected` | Foreign owner/frame, duplicate role, mismatched target, forged capability, or incoherent source relation. |

`NoSafeSlice` remains the development state while this vocabulary is absent;
it is not a fifth source disposition.

## Done for this design row

- one tracked design Decision and task order are accepted;
- current numeric-only schema limitation is recorded with the exact fixture;
- typed call/value ownership, fail-fast boundary, and Loop/Tail split are fixed;
- one-family implementation order is fixed (`ScanWithInit` first);
- non-claims forbid AST reuse, opaque fallback, route-specific operation
  kinds, Builder/MIR/physicalization, selector, retry, and production;
- the later implementation row explicitly updates
  `docs/reference/mir/loop-recipe-contract.md` and affected module READMEs in
  the same commit as the landed schema/observer/producer and tests.

## Stop

Do not implement S6C if the proposed call/value contract still requires a
method-name lookup, an opaque result, If-specific schema reuse, guessed item
counts, AST reconstruction, or a route-local adapter. Return to this design
boundary and close the missing authority first.
