---
Status: active — Builder compatibility migration is the next bounded slice
Date: 2026-08-08
Parent: `callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`
Reference: `docs/reference/language/callable-contracts.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R5

## Goal

Remove the remaining Builder-side assumptions that Box methods are an
unordered AST map. Keep the ordered AST inventory and explicit compatibility
projections as the only inputs, then delete old map helpers when their callers
reach zero.

## Authority

```text
parser AST:
  BoxMethodInventoryV1 is the only ordered source carrier

compatibility projection:
  named lookup/order is allowed only at an audited legacy edge

resolver/source seal:
  not opened by R5

Builder/MIR:
  consumes an explicit projection and cannot reconstruct source order
```

## Implementation order

1. Inventory all remaining `HashMap`/name-sort method projections and classify
   each caller as durable, compatibility-only, or retire candidate.
2. Replace one named Builder edge with an explicit inventory projection; add a
   focused positive/negative gate and keep the old edge out of that caller.
3. Repeat only for the next named caller after the fast gate is green. Do not
   widen the AST contract or add a fallback.
4. Delete an old helper only after caller-zero evidence is recorded.
5. Update the owner README, affected language/reference receipt, this card,
   task map, and `CURRENT_STATE.toml` in the same implementation commit.

## Stop lines

```text
no resolver capability
no Hako parser parity claim
no CallableContract issuer
no source order from HashMap/name sorting
no compatibility-row promotion
no Builder retry/fallback
no broad cleanup mixed with a single caller migration
```

## Acceptance

```text
each migrated caller names its inventory projection
compatibility projection remains visibly compatibility-only
caller-zero guard proves retired helper has no production callers
focused tests cover order/provenance preservation and malformed input
all touched source files remain below 800 lines
reference and README receipt land with the implementation
```
