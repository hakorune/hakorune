# Phase 184x: dead local field-get read pruning

- Status: Landed
- Purpose: widen DCE with a first effect-sensitive read-removal slice by removing dead `FieldGet` reads on definitely non-escaping local boxes.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for dead local `FieldGet` reads
- Non-goals:
  - no `Load` pruning
  - no `Debug` stripping
  - no control-flow terminator cleanup
  - no broader effect-sensitive DCE widening

## Decision

- `FieldGet { dst: _, base, .. }` may be removed when `base` resolves to a definitely non-escaping local box root and the result is otherwise unused.
- the dead read's operand chain may disappear too when nothing else keeps it live.
- `Load` stays outside this cut.
- `Debug` stays outside this cut because it still carries debug effect in MIR metadata.
- terminators (`Return` / `Branch` / `Jump`) stay outside this cut.

## Acceptance

- dead `FieldGet` reads on non-escaping local boxes disappear when otherwise unused.
- the dead alias chain feeding such a field read disappears too.
- reachable edge-arg and return-driven liveness stays unchanged.
- `Load` and `Debug` instructions remain in place.
- `tools/checks/dev_gate.sh quick` stays green.

## Exit

- dead local `FieldGet` read pruning is landed.
- broader effect-sensitive / partial DCE remains separate backlog.
