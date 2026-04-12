# Phase 183x: pure no-dst call pruning

- Status: Landed
- Purpose: widen the generic no-dst pure cleanup slice beyond `Safepoint` by removing pure `Call` instructions that produce no destination and are otherwise unused.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for pure no-dst `Call`
- Non-goals:
  - no `Debug` stripping
  - no control-flow terminator cleanup
  - no unreachable block deletion
  - no broader effect-sensitive DCE widening

## Decision

- pure `Call` instructions with `dst: None` may be removed when `effects().is_pure()`
- operand values of such calls are no longer treated as liveness roots on the DCE side unless another reachable use keeps them live
- `Safepoint` remains removable through the same generic no-dst pure cleanup helper
- `Debug` stays outside this cut because it still carries debug effect in MIR metadata
- terminators (`Return` / `Branch` / `Jump`) stay outside this cut

## Acceptance

- pure no-dst `Call` instructions disappear when they are otherwise unused
- the dead operand chain feeding such a call disappears too
- reachable edge-arg and return-driven liveness stays unchanged
- `Debug` instructions remain in place
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- pure no-dst `Call` pruning is landed
- broader effect-sensitive / partial DCE remains separate backlog
