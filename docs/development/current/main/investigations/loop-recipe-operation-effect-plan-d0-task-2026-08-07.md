# Loop Recipe Operation Effect Plan D0

Status: `DESIGN-STOP`
Date: 2026-08-07
Parent: `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Purpose

Define the smallest AST-free source/effect projection that may be consumed by
the common Loop physicalizer after the topology/After canary. This is a design
row only. It must close the identity gap before any operation MIR is emitted.

## Problem to solve

`VerifiedLoopBindingEffectRelationV1` currently carries a binding, role
ordinal, class, and source anchor, but not a `LoopItemKeyV1`. Repeated read or
write ordinals in nested loops therefore cannot identify one logical operation
without consulting source order or guessing from names. The physicalizer must
not perform that reconstruction.

## Required design

Issue one move-only, AST-free effect product whose every row contains:

```text
LoopItemKeyV1
exact source/effect anchor
recipe binding/value relation
resolver-issued BindingRefV1
operation kind and typed operands
owner / loop / block provenance
```

The product must be issued from the existing co-sealed source relations. It
may project or consume them, but it must not copy source truth, re-verify the
Recipe, or create a second operation owner. Nested loops must remain
unambiguous even when role ordinals repeat.

## Acceptance matrix

Accept only when all of these are explicit and mechanically checkable:

- every accepted operation maps to exactly one `LoopItemKeyV1`;
- the item key belongs to the existing Recipe block and loop topology;
- the source anchor and resolver binding have the same owner/brand;
- each operand/result value key is typed and exists in the same Recipe;
- duplicate item keys, foreign anchors, missing operands, wrong block/loop,
  and repeated-ordinal ambiguity reject as typed `NoSafeSlice`;
- no name lookup, source preorder rematch, profile label, or legacy route is
  consulted;
- the common physicalizer receives this effect product only after the
  topology/entry receipt boundary is already sealed;
- failure remains a whole unpublished function-session discard; no retry or
  fallback is introduced.

## Explicit non-goals

```text
operation MIR emission
Return / DraftSeal / module publication
production selector or caller switch
Generic relabeling
new Recipe kinds
same-session repair/retry
legacy scheduler/fallback deletion
```

## Deliverables

1. One SSOT schema and ownership statement for the item-keyed effect product.
2. Positive nested-loop fixture plus negative duplicate/foreign/missing cases.
3. A passive conversion API from existing co-sealed source relations.
4. Exact reference and owning README updates in the implementation commit
   that first consumes the product; no implementation claim before then.

## Next step

After this design is accepted, implement only the passive effect product and
its focused gates. Operation lowering remains closed until that receipt is
green and the current state selects the next implementation row.
