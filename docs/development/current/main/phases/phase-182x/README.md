# Phase 182x: unreachable block pruning

- Status: Landed
- Purpose: land the next CFG-cleanup slice by deleting blocks unreachable from `entry` after the current DCE liveness cuts have already stabilized.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/function.rs`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for unreachable block pruning
- Non-goals:
  - no Debug stripping
  - no new effect vocabulary
  - no generic no-dst pure cleanup widening
  - no control-flow terminator semantics change
  - no string return-carrier cleanup

## Decision Now

- `phase181x` is landed, and the next DCE/CFG cleanup slice is unreachable block pruning
- this cut is structural: remove dead blocks that cannot be reached from `entry`
- keep `Debug` and effect-sensitive cleanup separate from this block-level cleanup
- do not treat this as a new effect semantics phase

## Acceptance

- blocks unreachable from `entry` are removed from the live function block map
- reachable blocks remain intact
- CFG predecessor caches are refreshed after pruning
- existing reachable-only liveness and `Safepoint` / `KeepAlive` behavior stays unchanged
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- unreachable block pruning is landed as the next CFG-cleanup slice
- broader effect-sensitive / no-dst DCE remains separate backlog
