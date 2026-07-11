# Phase 188x: cross-block overwritten local field-set pruning

- Status: Landed
- Purpose: widen effect-sensitive DCE across one linear reachable edge by removing an earlier local `FieldSet` when a later same-root/same-field write in the unique successor block overwrites it before any reachable read or escape use.
- Scope:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/phases/README.md`
  - `docs/development/current/main/phases/phase-163x/README.md`
  - `src/mir/passes/dce.rs`
  - focused DCE unit contracts for linear-edge overwritten local field writes
- Non-goals:
  - no generic `Store` pruning
  - no generic `Load` pruning
  - no `Debug` stripping
  - no terminator cleanup
  - no loop backedge overwrite folding
  - no merge/multi-predecessor overwrite reasoning

## Decision

- an earlier `FieldSet` may disappear across a block boundary only when the CFG edge is a reachable linear edge:
  - predecessor has one reachable successor
  - successor has one reachable predecessor
  - successor does not dominate predecessor
- the overwrite witness is still keyed by the same definitely non-escaping local root and field
- a reachable `FieldGet` on that field or any reachable escape use of that root clears the overwrite witness
- loop backedges, merges, `Store`, `Load`, `Debug`, and terminators stay outside this cut

## Acceptance

- a same-root/same-field local write in block `A` disappears when block `B` is the unique linear reachable successor and overwrites that field before any reachable read/escape
- a successor-block read before the overwrite keeps the predecessor write alive
- loop backedges and merge edges remain untouched
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- cross-block overwritten local field-set pruning is landed
- the remaining DCE backlog is the broader effect-sensitive / partial widening beyond the local field slices and linear-edge overwrite case
