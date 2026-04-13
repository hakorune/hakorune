Status: LANDED
Phase: 274x

# 274x-90 ThinLTO First Cut SSOT

## Decision

- accepted

## Scope

- add the first narrow `ThinLTO` cut
- consume landed build-policy and callable/edge contracts
- keep `PGO` disabled

## Owners

- `ipo_build_policy`
- `ipo_callable_contract`
- `ipo_call_edge_contract`

## Contract

- `ThinLTO` is a build-time consumer, not a closure-meaning owner
- `ThinLTO` must read derived callable/edge facts, not raw closure split booleans
- the first cut stays narrow and conservative

## Out Of Scope

- `PGO`
- profile artifact generation / merge / use
- closure-specific link-time rewrites
