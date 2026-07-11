# Phase 176x: reachable-only DCE first cut

- Status: Landed
- Purpose: land the first cross-block/partial DCE slice by ignoring uses that exist only in blocks unreachable from `entry`, while keeping the current pure-instruction DCE contract otherwise unchanged.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for reachable-only liveness marking
- Non-goals:
  - no unreachable block deletion
  - no partial side-effect DCE
  - no new effect vocabulary
  - no string return-carrier cleanup
  - no generic escape widening

## Decision Now

- treat this as the first `cross-block` DCE cut, not as a generic CFG cleanup phase
- semantic authority stays:
  - `MirInstruction::effects()`
  - current generic `used_values()`
  - reachability from `entry`
- this cut is allowed to:
  - ignore terminator / edge-arg / instruction uses in unreachable blocks
  - remove pure defs whose only consumers live in unreachable blocks
- this cut is not allowed to:
  - delete non-pure unreachable instructions
  - reorder across effects
  - reinterpret effect classes

## Acceptance

- if a pure value is used only by unreachable blocks, DCE may remove that def
- if an unreachable block contains pure instructions whose results are not used by reachable blocks, DCE may remove those pure instructions too
- reachable edge-arg use-sites must still keep their carrier values alive
- focused unit guards live in `src/mir/passes/dce.rs`
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- reachability-aware liveness marking is landed as the first DCE widening slice
- broader `partial` / effect-sensitive DCE remains separate backlog
