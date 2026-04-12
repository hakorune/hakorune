# Phase 186x: same-root phi local field pruning

- Status: Landed
- Purpose: widen effect-sensitive DCE across cross-block same-root phi carriers by removing dead local `FieldGet` / `FieldSet` operations that still resolve to the same definitely non-escaping local box.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for same-root phi local field reads/writes
- Non-goals:
  - no generic `Store` pruning
  - no generic `Load` pruning
  - no `Debug` stripping
  - no terminator cleanup
  - no generic multi-root phi relaxation

## Decision

- a multi-input `Phi` may stay inside the local-field DCE slice when it resolves to the same definitely non-escaping local box root under generic `phi_query`
- dead `FieldGet` reads through that same-root phi may disappear when otherwise unused
- dead `FieldSet` writes through that same-root phi may disappear when otherwise unobserved
- mixed-root phi merges stay outside this cut
- `Store`, `Load`, `Debug`, and terminators stay outside this cut

## Acceptance

- dead local `FieldGet` / `FieldSet` operations disappear even when the local box flows through a same-root multi-input phi
- the phi carrier and dead copy chain feeding it disappear too when nothing else keeps them live
- mixed-root phi merges stay untouched
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- same-root phi local field pruning is landed
- broader effect-sensitive / partial DCE remains separate backlog
