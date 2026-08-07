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

The logical Recipe already carries `LoopItemKeyV1` and a typed
`LoopOperationV1`. The existing `VerifiedLoopBindingEffectRelationV1` is a
different product: it records binding read/write/carrier effects and is not an
operation-source map. The callable test producer also already has an exact
item/site/operation relation, but it is profile-local and its operation view
duplicates the Recipe operation vocabulary. Generic G0 currently keeps the
source sites in its structural facts and drops them after issuing the Core.

The design gap is therefore the neutral projection that combines existing
logical item/operation truth with one profile-issued exact source anchor. It
must not be repaired by adding item keys to the binding-effect product or by
matching repeated ordinals after the fact.

## Required design

Issue one move-only, AST-free operation/effect product whose every row contains:

```text
LoopItemKeyV1
exact source/effect anchor
owner/source brand
checked loop/block placement witness
```

An operation may have no source binding (for example a literal), so a
resolver-issued `BindingRefV1` is an optional row witness, not a required
field. When present, it must have the same owner/source brand as the anchor.
The row must not copy Recipe operands; consumers read those through the
single `LoopItemKeyV1` reference.

The row does not copy `LoopOperationV1` or its operands. The sealed Recipe is
the sole logical operation owner; the item key is the typed reference used to
read and validate that operation exactly once.

The product must be issued from the existing co-sealed Core plus profile
source relations. It moves the Core as its unique logical owner and carries
the profile evidence beside it; it must not copy source truth, re-verify the
Recipe, or create a second operation owner. The binding-effect product remains
a separate table inside that Core. Nested loops must remain unambiguous even
when role ordinals repeat.

The join must use the existing Core's sealed effect rows. If the current Core
API does not expose the already-sealed anchor/class needed for the join, add a
non-authority accessor or one consuming join helper at that Core boundary.
Do not create a second effect catalog or copy the rows into the operation
product.

Coverage is keyed by Recipe operations, not by all Core effect rows. Every
`LoopRecipeItemV1::Operation` item must have exactly one source-evidence row;
`ReadBinding`/`WriteBinding` items additionally point to their exact sealed
Core effect row when one exists. Literal/compare/binary operation items need
no binding-effect row. Structural carrier rows and callable Tail/After reads
remain owned by their existing continuation/tail products, and their explicit
non-consumption is not a silent drop.

The operation product must be issued before the P0 topology-only
`into_physical_boundary` path is called. That path intentionally drops source
anchors and is not an operation-D0 entry point. Operation D0 must provide a
separate consuming conversion (for example `into_operation_effect_product`)
so the Core is never duplicated and the anchors are never reconstructed after
they have been dropped.

## Candidate comparison

```text
extend VerifiedLoopBindingEffectRelationV1 with LoopItemKey
  reject: it changes a binding-lifetime product into an operation map

reuse callable VerifiedLoopOperationSourceRelationV1 directly
  reject: it is profile-local and duplicates the Recipe operation enum;
  Generic G0 has no equivalent product at the same boundary

issue one neutral operation/effect product from Recipe + profile adapter
  accept candidate: Recipe owns item/operation truth, the adapter owns the
  exact source anchor, and the neutral product only co-seals their relation
```

The recommended product owns the existing Core and one evidence ledger, not a
second Recipe or a copied operation table:

```text
VerifiedLoopOperationEffectProductV1 {
    core: VerifiedLoopCoreProductV1       // moved, sole logical owner
    source_evidence: [
        VerifiedLoopOperationSourceEvidenceV1 {
            item: LoopItemKeyV1
            source_anchor: exact AST-free source anchor
            source_loop: owner-branded loop source site
            source_binding: optional view into core binding relations
            placement: checked view from core Recipe membership
        }
    ]
}
```

`placement` is a checked witness derived from existing Recipe item
membership, not a second topology table. The product is non-`Clone` and is
issued once by a profile adapter while moving the co-sealed Core; the common
physicalizer may consume it but must not rebuild it from `role.ordinal` or
source-expression order. The public name may be finalized as
`VerifiedLoopOperationEffectProductV1` during D0, but there must be exactly one
product owner and no parallel relation/table with the same meaning. An
evidence row's operation kind, operands, binding class, and effect relation
are always read by its `item` from `core`; they are not copied into the row.

`DerivedCarrierEntry` remains in the separate binding-effect product because
it has no `LoopRecipeItemV1::Operation` row. It must not be fabricated as an
operation just to fill the common product.

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
- the common physicalizer receives this operation/effect product only after the
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
3. A passive conversion API from existing co-sealed source relations, with
   the Generic G0 producer retaining/issuing its exact item-keyed source
   evidence before the structural facts are consumed.
4. Exact reference and owning README updates in the implementation commit
   that first consumes the product; no implementation claim before then.

## Ordered implementation ladder after this design stop

The following order is part of this task's exit contract. Do not skip ahead or
open operation MIR before the preceding receipt is green.

```text
0. D0 decision (this row)
   freeze row fields, source-anchor brand, optional BindingRef rule,
   placement-witness rule, Core move ownership, the pre-physical-boundary
   conversion point, and typed reject cases.

1. passive profile evidence
   callable adapter and Generic G0 producer retain/issue item-keyed exact
   source anchors before structural facts are consumed; no MIR or selector.
   The evidence is issued before Core/After is reduced to the P0 physical
   boundary, and no later source-order reconstruction is permitted.

2. neutral product conversion
   issue VerifiedLoopOperationEffectProductV1 once from the co-sealed Core +
   profile evidence; consume/validate Recipe membership exactly once; add positive
   nested-loop and duplicate/foreign/missing/wrong-placement gates. Join to
   sealed Core effect rows through an accessor/helper only; no copied effect
   catalog.

3. operation physicalization canary
   only after steps 0-2 are green; keep operation emission test-only and
   caller-zero, with existing session discard and DraftSeal ownership.

4. profile parity and production cutover
   later row only: callable/G0 parity, selector switch, retry/fallback
   retirement, then legacy route deletion.
```

Every implementation step must remain below the repository's 800-line source
limit. The step that first consumes the neutral product must update the exact
landed behavior references in the same commit (the loop recipe contract,
Generic stage matrix, loop-recipe-contract README, owning lowering README,
and the current-state/workstream receipt). It must not update a reference page
to claim physical, production, backend, or retirement behavior before that
behavior is actually landed.

## Next step

After this design is accepted, implement only the passive effect product and
its focused gates. Operation lowering remains closed until that receipt is
green and the current state selects the next implementation row.
