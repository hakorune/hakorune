# Phase 184x SSOT: dead local field-get read pruning

Scope: land the next effect-sensitive DCE slice by removing dead `FieldGet` reads on definitely non-escaping local boxes, while keeping `Load`, `Debug`, control-flow terminators, and broader effect-sensitive DCE separate.

## Decision

- `FieldGet { dst: _, base, .. }` is removable when `base` resolves to a definitely non-escaping local box root and the result is otherwise unused.
- operands feeding such a dead read are not liveness roots unless another reachable use keeps them live.
- `Load` stays outside this cut.
- `Debug` remains outside this cut because it still carries debug effect.
- control-flow terminators (`Return` / `Branch` / `Jump`) stay outside this cut.

## Contracts

- removing a dead local field-get read must not change reachable liveness for any other instruction kind.
- the existing `KeepAlive` pruning behavior remains unchanged.
- the existing reachability pruning behavior remains unchanged.

## Exit Condition

- dead local `FieldGet` reads are removed by DCE when present as otherwise-unused no-ops.
