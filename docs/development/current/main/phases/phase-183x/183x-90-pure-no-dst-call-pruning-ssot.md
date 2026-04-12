# Phase 183x SSOT: pure no-dst call pruning

Scope: land the next generic no-dst pure cleanup slice by removing pure `Call` instructions with no destination, while keeping `Debug`, control-flow terminators, and broader effect-sensitive DCE separate.

## Decision

- `Call { dst: None, effects: PURE }` is removable when otherwise unused.
- operands of such calls do not anchor liveness unless another reachable use keeps them live.
- `Safepoint` stays removable through the same generic no-dst pure cleanup helper.
- `Debug` remains outside this cut because it still carries debug effect.
- control-flow terminators (`Return` / `Branch` / `Jump`) stay outside this cut.

## Contracts

- removing a pure no-dst call must not change reachable liveness for any other instruction kind.
- the existing `KeepAlive` pruning behavior remains unchanged.
- the existing reachability pruning behavior remains unchanged.

## Exit Condition

- pure no-dst `Call` instructions are removed by DCE when present as otherwise-unused no-ops.
