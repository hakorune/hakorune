# Phase 185x: dead local field-set write pruning

- Status: Landed
- Purpose: widen DCE with a first effect-sensitive write-removal slice by removing dead `FieldSet` writes on definitely non-escaping local boxes.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for dead local `FieldSet` writes
- Non-goals:
  - no `Store` pruning
  - no `Load` pruning
  - no `Debug` stripping
  - no control-flow terminator cleanup
  - no broader effect-sensitive DCE widening

## Decision

- `FieldSet { base: _, value, .. }` may be removed when `base` resolves to a definitely non-escaping local box root and that root has no reachable `FieldGet` observers.
- the dead write's operand chain may disappear too when nothing else keeps it live.
- `Store` stays outside this cut.
- `Load` stays outside this cut.
- `Debug` stays outside this cut because it still carries debug effect in MIR metadata.
- terminators (`Return` / `Branch` / `Jump`) stay outside this cut.

## Acceptance

- dead `FieldSet` writes on non-escaping local boxes disappear when otherwise unobserved.
- the dead alias chain feeding such a field write disappears too.
- live `FieldGet` observers keep the corresponding field write in place.
- `Store` and `Load` instructions remain in place.
- `tools/checks/dev_gate.sh quick` stays green.

## Exit

- dead local `FieldSet` write pruning is landed.
- broader effect-sensitive / partial DCE remains separate backlog.
