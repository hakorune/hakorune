# Phase 181x: safepoint no-op DCE

- Status: Landed
- Purpose: land the first generic no-dst pure cleanup slice by removing `Safepoint` no-op instructions while keeping `Debug`, terminators, and broader partial DCE separate.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for `Safepoint`
- Non-goals:
  - no `Debug` stripping
  - no unreachable block deletion
  - no generic partial DCE widening
  - no control-flow terminator cleanup

## Decision Now

- `phase177x` is landed, and the next DCE slice is the first no-dst pure cleanup
- `phase181x` is landed; `Safepoint` no-op instructions were the first generic no-dst pure cleanup slice
- `Debug` stays outside this cut because it still carries debug effect in MIR metadata
- terminators / `Return` / `Branch` / `Jump` stay outside this cut

## Acceptance

- `Safepoint` no-op instructions disappear when they are reachable and otherwise unused
- `Debug` instructions remain in place
- reachable edge-arg and return-driven liveness stays unchanged
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- the first generic no-dst pure cleanup slice is landed
- broader no-dst / effect-sensitive DCE remains separate backlog
