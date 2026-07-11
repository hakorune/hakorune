# Phase 181x SSOT: Safepoint no-op DCE

Scope: land the first generic no-dst pure cleanup slice by removing `Safepoint` no-op instructions, while keeping `Debug` and terminators outside the current DCE widening.

## Decision

- `Safepoint` is a no-op today and may be removed when it is otherwise unused.
- `Debug` remains outside the generic no-dst cleanup path because it still carries debug effect.
- control-flow terminators (`Return` / `Branch` / `Jump`) stay outside this cut.
- the current DCE widening is still a local pass concern; no backend/runtime policy move is introduced here.

## Contracts

- `Safepoint` removal must not change reachable value liveness for any other instruction kind.
- the existing `KeepAlive` pruning behavior remains unchanged.
- the existing reachable-only DCE behavior remains unchanged.

## Exit Condition

- `Safepoint` is removed from the live instruction stream by DCE when present as a removable no-op.
