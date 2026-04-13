Status: ACTIVE
Phase: 273x

# 273x-90 IPO Callable / Edge Contract Owner Seam SSOT

## Decision

- accepted

## Scope

- add one owner seam for IPO callable-node facts
- add one owner seam for IPO call-edge facts
- keep build behavior unchanged in this phase

## Owners

- `ipo_callable_contract`
- `ipo_call_edge_contract`

## Contract

- `closure_split_contract` remains the owner for closure meaning
- IPO consumes derived facts through a separate layer:
  - callable facts:
    - thin surface
    - env surface
    - addr-taken / escape / effect-facing facts
  - edge facts:
    - direct thin / direct thick / indirect call shape
    - future hotness overlay slot
- `ThinLTO` and `PGO` must consume these contracts, not raw closure facts

## Out Of Scope

- enabling `ThinLTO`
- enabling `PGO`
- profile artifact generation / use
