# Generic Memory DCE Observer/Owner Contract SSOT

Status: SSOT
Date: 2026-04-12
Owner: `phase-199x`

## Purpose

- fix the first generic-memory contract before widening DCE beyond local `FieldGet` / `FieldSet`
- keep `Load` / `Store` ownership separate from local-field lane A and observer/control lane C

## Vocabulary

### GenericMemoryOp

- `Load { dst, ptr }`
- `Store { value, ptr }`

This lane does **not** own:

- `FieldGet` / `FieldSet`
- `Debug`
- terminators / control anchors
- future `store.array.str` / `store.map.value` pilot rows

### PrivateCarrierRoot

The first lane-B cuts may only reason about pointer carriers that are:

1. rooted at `RefNew { dst, box_val }`
2. rooted on a definitely non-escaping local box
3. propagated only through copy-only aliases on the first cut

Out of scope for the first cuts:

- same-root phi carriers
- mixed public/private pointer families
- alias-heavy pointer graphs

### Memory Observer

Memory observation for lane B is owned by the carrier side (`ptr`), not by the stored value side.

Observer classes:

1. `Load { ptr }` on the same private carrier family
2. `Store { ptr }` on the same private carrier family
3. any escape of the carrier itself across the existing boundary vocabulary

Non-goal:

- `Store.value` liveness is **not** memory-observer ownership
- it stays on the existing SSA liveness / escape vocabulary side

## Lane Split

- lane A owns local object field reasoning:
  - `FieldGet`
  - `FieldSet`
- lane B owns generic pointer-carrier memory reasoning:
  - `Load`
  - `Store`
- lane C owns observer/control semantics:
  - `Debug`
  - terminators
  - control anchors

No phase may mix A/B/C widening in one cut.

## First Narrow Cuts

### B1 Dead Load Pruning

Allowed only when:

- `Load { dst, ptr }`
- `dst` is dead after normal SSA liveness
- `ptr` resolves to a `PrivateCarrierRoot`
- no carrier escape is observed

Not allowed yet:

- phi-carried pointer roots
- public/shared pointer carriers
- alias-heavy reasoning

### B2 Overwritten Store Pruning

Allowed only when:

- earlier `Store { value, ptr }` and later `Store { value, ptr }` target the same `PrivateCarrierRoot`
- no intervening `Load` on that carrier family
- no intervening carrier escape

Not allowed yet:

- store-to-load forwarding
- dead-store elimination on public/shared carriers
- MemorySSA-style whole-function reasoning

## Guardrails

- `Load` remains a READ effect and `Store` remains a WRITE effect
- lane B must not infer object-field semantics from pointer-carrier traffic
- lane B must not widen `FieldGet` / `FieldSet` heuristics by another name
- `Debug` policy stays in lane C
