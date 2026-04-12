# 201x-90 Private-Carrier Overwritten Store Pruning SSOT

Status: SSOT

## Goal

- finish lane B2 from `phase199x`
- keep generic memory reasoning narrow before the observer/control lane starts

## Contract

- B2 owns only overwritten `Store { value, ptr }` pruning
- `ptr` must resolve to a definitely private carrier root
- first cut allows:
  - direct same-root carrier reuse
  - copy-only alias propagation
  - same-block overwrite only

## Blockers

- any intervening `Load` on the same private carrier family
- any carrier escape
- cross-block or loop-carried store reasoning
- forwarding or value replacement

## Next

1. C0 observer/control docs-only inventory
2. C1 `Debug` policy decision
3. C2 terminator/control cleanup

## Not Yet

- MemorySSA-style reasoning
- dead-store elimination on public/shared carriers
- store-to-load forwarding
