---
Status: Design closed; next bounded BoxShape row
Date: 2026-08-08
Decision: accepted architecture; `NoSafeSlice` remains only until the typed schema/target issuer is implemented
Scope: M8 LoopV0 scan ingress; one profile-neutral Recipe vocabulary, no physical activation
---

# LOOP-RECIPE-TYPED-CALL-VALUE-D0

## Current Capsule

- **Current decision:** S6C scans cannot issue a truthful portable Recipe with
  the current numeric-only `LoopRecipeV1` vocabulary.
- **Current implementation status:** the typed boundary is accepted, but no
  schema, instance-target issuer, observer, producer, Builder, or physicalizer
  change is landed for S6C.
- **Next ordered task:** first perform the behavior-neutral source split in
  `LOOP-RECIPE-OPERATION-SHAPE-SPLIT-R0`; then implement the typed schema and
  instance-call target as separate BoxCount rows.
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
these properties. The schema-version decision is explicit: do not silently
widen the pre-production numeric V1 wire. The typed cohort will use an
explicit `LoopRecipeV2` artifact/schema version.

1. **Typed values:** preserve existing `I64`, `Bool`, and `Unit`, and add only
   an explicitly named logical `Text` value domain for forward `ScanWithInit`.
   `Text` does not imply pointer representation, ownership, GC, or ABI. A
   source-bound value/Home contract supplies those facts. `Handle`, `Any`,
   `Opaque`, Array, collection, and nominal Box values are not in this cohort.
2. **Typed call leaf:** add one Recipe-local call slot carrying an optional
   receiver `ValueKey`, ordered argument `ValueKey`s, and an optional result
   `ValueKey`. The Recipe never contains a method/Box name, MIR callee,
   resolver capability, `ValueId`, `BasicBlockId`, or runtime lookup string.
   The source-bound call relation owns the resolver-issued target, exact
   receiver/parameter/result contract, Home relation, effects, suspension and
   control disposition, ABI profile, and exact source expression site.
3. **Allowed first profile:** calls must be exact, non-suspending, non-control,
   and admitted by the existing callable/type contract. Missing signature,
   unsupported effect, ownership mismatch, or unknown result class freezes
   before Recipe/Core publication.
4. **Typed comparison:** add a fixed `TextEq(Text, Text) -> Bool` operation.
   Do not import or reuse the If-only direct-call schema. Numeric comparison
   remains the existing operation family; cross-domain coercion is rejected.
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

## Accepted source-bound target boundary

The current `ResolvedCallableRefV1` is free-static only. It is not sufficient
for `subject.length()` or `subject.substring(...)`. Before a scan observer can
be implemented, the resolver must issue one opaque instance-method target
capability that co-seals target identity, receiver/parameter/result types,
Home relations, effects, suspension/control, ABI profile, and source site.
Facts may retain the semantic role, `BindingRef`, and exact site; the producer
may mint the local call-slot and value keys; no layer may recover the target
from a method or Box name.

Home is not a `LoopValueClass`. Unknown Home relation, result class, effect, or
target identity is a pre-Recipe failure, not an opaque value fallback.

`return i` is `LoopRecipeItemV1::Exit(Return { value })` when the Loop algebra
owns that exit. The outer `return -1` remains callable Tail/Completion. ABI,
Tail, and Completion never enter Loop Facts or the neutral Recipe.

Disposition precedence is fixed:

```text
Rejected (identity / foreign / duplicate / forged)
  > Unresolved (missing target / signature / Home / effect / site / coverage)
  > Declined (fully observed but wrong family shape/effect)
  > Candidate (exact complete contract)
```

`NoSafeSlice` is a development state while the contract or issuer is absent;
it is not a fifth source disposition.

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
- explicit V2 schema boundary, logical `Text`, local Call slot, `TextEq`,
  source-bound instance target, Home/effect ownership, fail-fast precedence,
  and Loop/Tail split are fixed;
- the 781-line demand and 725-line verifier modules are scheduled for the
  behavior-neutral `LOOP-RECIPE-OPERATION-SHAPE-SPLIT-R0` before schema growth;
- one-family implementation order is fixed (`ScanWithInit` first);
- non-claims forbid AST reuse, opaque fallback, route-specific operation
  kinds, Builder/MIR/physicalization, selector, retry, and production;
- the later implementation row explicitly updates
  `docs/reference/mir/loop-recipe-contract.md` and affected module READMEs in
  the same commit as the landed schema/observer/producer and tests.

## Ordered successor rows

```text
LOOP-RECIPE-OPERATION-SHAPE-SPLIT-R0
  -> typed schema V2 + local call slot/TextEq
  -> resolver instance-call target contract
  -> source-bound call relation/verifier
  -> S6C ScanWithInit Facts/producer
  -> physical canary
  -> production switch and callers-zero retirement
```

The first row is BoxShape-only and may use a short refactor series. Schema,
instance-target, and ScanWithInit are separate BoxCount rows. Every landed
typed schema/observer/producer row updates the reference contract and affected
module READMEs in the same commit; legacy scan facts/builders are deleted only
after production parity and callers-zero evidence.

## Stop

Do not implement S6C if the proposed call/value contract still requires a
method-name lookup, an opaque result, If-specific schema reuse, guessed item
counts, AST reconstruction, or a route-local adapter. Return to this design
boundary and close the missing authority first.
