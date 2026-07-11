# Phase 177x: redundant keepalive pruning

- Status: Landed
- Purpose: land the first effect-sensitive DCE slice by pruning `KeepAlive { values }` instructions that no longer contribute any new liveness beyond already-reachable semantic uses.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for redundant `KeepAlive`
- Non-goals:
  - no `Debug` stripping
  - no `Safepoint` stripping
  - no unreachable block deletion
  - no generic no-dst pure instruction cleanup
  - no string return-carrier cleanup

## Decision Now

- treat this as the first effect-sensitive DCE cut, not as generic pure no-dst cleanup
- semantic authority stays:
  - `MirInstruction::effects()`
  - current reachable-only liveness marking from `phase-176x`
  - `KeepAlive` as a liveness-only instruction with no runtime effect
- this cut is allowed to:
  - remove `KeepAlive` only when every listed value is already live for other reachable reasons
- this cut is not allowed to:
  - remove a `KeepAlive` that is the only reason some value stays live
  - reinterpret `Debug` / `Safepoint`
  - widen into full partial side-effect DCE

## Acceptance

- redundant `KeepAlive` instructions disappear
- values kept alive only by a non-redundant `KeepAlive` still remain alive
- reachable edge-arg and return-driven liveness stays unchanged
- focused unit guards live in `src/mir/passes/dce.rs`
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- redundant `KeepAlive` pruning is landed as the first effect-sensitive DCE slice
- broader effect-sensitive / no-dst cleanup remains separate backlog
