# Phase 187x: overwritten local field-set pruning

- Status: Landed
- Purpose: widen effect-sensitive DCE within reachable blocks by removing an earlier local `FieldSet` when the same definitely non-escaping local root/field is overwritten before any reachable read or escape use.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for same-block overwritten local field writes
- Non-goals:
  - no generic `Store` pruning
  - no generic `Load` pruning
  - no `Debug` stripping
  - no terminator cleanup
  - no cross-root alias write folding

## Decision

- a reachable earlier `FieldSet` may disappear when a later reachable `FieldSet` writes the same field on the same definitely non-escaping local root before any reachable `FieldGet` or escape use of that root
- the later surviving write remains intact when a downstream read still needs that field value
- the cut stays same-block and local-root only; generic store/load memory DCE remains out of scope
- `Store`, `Load`, `Debug`, and terminators stay outside this cut

## Acceptance

- an overwritten earlier local `FieldSet` disappears when a later same-root/same-field write dominates the final observable field value inside the block
- a later same-root/same-field read keeps the final write alive
- unrelated generic memory instructions remain untouched
- `tools/checks/dev_gate.sh quick` stays green after perf release artifacts are synced

## Exit

- same-block overwritten local field-set pruning is landed
- the remaining DCE backlog is broader effect-sensitive / partial widening beyond the local field slices
