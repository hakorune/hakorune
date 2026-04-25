---
Status: Landed
Date: 2026-04-25
Scope: Decide the remaining constructor/birth carrier split after the compatibility contract is explicit.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-188-remaining-inc-mirror-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-216-receiver-surface-fallback-sunset-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-248-runtime-data-get-route-prune-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-249-constructor-birth-compatibility-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-smoke-index.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/abi-export-inventory.md
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/retained-boundary-and-birth-placement-ssot.md
---

# 291x-250 Constructor-Birth Carrier Design Card

## Goal

Make the remaining constructor/birth follow-up explicit without touching the
compatibility row yet.

`291x-249` already fixed the current truth:

```text
NewBox(ArrayBox|MapBox)
  -> explicit birth() initializer marker
  -> generic-method `birth` compatibility row
  -> dedicated constructor/birth carrier, if ownership is split further
```

This card exists to decide the next carrier shape before any prune attempt.

## Boundary

- Do not prune the generic-method `birth` compatibility row in this card.
- Do not change `nyash.array.birth_h` / `nyash.map.birth_h` ABI exports in
  this card.
- Do not collapse constructor intent and birth compatibility into one hidden
  fallback.
- Do not add new classifier growth or new fallback behavior.
- Do not reopen receiver-surface prune work here.

## Analysis

The remaining seam is not a dead mirror. It is the boundary between three
already-documented truths:

1. `NewBox` still emits an explicit birth initializer marker in the MIR
   builder.
2. The generic-method `birth` row still exists as transitional compatibility
   glue.
3. The concrete `birth_h` exports already exist as mainline substrate rows in
   the ABI inventory.

The open question is ownership shape:

- a dedicated constructor/birth carrier for ArrayBox and MapBox
- a metadata path that lets the current carrier remain thin
- or a deferred hold if neither can be proven without widening the surface

## Decision

Keep the current `birth` compatibility row pinned until one carrier shape is
chosen and documented.

The next step should be a design card or metadata-path card that answers:

- where the constructor intent lives
- which layer owns the compatibility row
- what fixture or smoke pins the chosen boundary

## Next Work

The safest follow-up is a narrow carrier-design card that selects one owner
shape and records the deletion condition for the transitional `birth` row.

## Acceptance

- The card makes the carrier choice explicit before any code change.
- The card names the owner split and the removal condition for the temporary
  `birth` compatibility row.
- The card stays docs-only and does not overlap the existing prune or review
  cards.
