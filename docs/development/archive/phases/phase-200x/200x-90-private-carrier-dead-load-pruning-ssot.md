# 200x-90 Private-Carrier Dead Load Pruning SSOT

Status: SSOT

## Goal

- finish lane B1 from `phase199x`
- add the first code slice for generic memory DCE without widening beyond private carriers

## Contract

- B1 owns only dead `Load { dst, ptr }` pruning
- `ptr` must resolve to a definitely private carrier root
- first private carrier roots are:
  - `RefNew { dst, box_val }`
  - `box_val` is a definitely non-escaping local box
  - alias propagation is `Copy`-only on this cut

## Blockers

- same-carrier `Store`
- carrier escape through `Call`, `Return`, `Throw`, or other non-copy uses
- `Debug` and observer/control cleanup
- phi-carried or mixed-root carrier aliases

## Next

1. B2 overwritten `Store` pruning on the same private carriers
2. C0 observer/control docs-only inventory

## Not Yet

- store-to-load forwarding
- dead-store elimination
- MemorySSA-style reasoning
- generic public/private carrier merging
