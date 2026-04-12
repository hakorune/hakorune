# 187x-90: overwritten local field-set pruning SSOT

Status: SSOT
Date: 2026-04-12
Scope: extend the landed local-field DCE slice so a same-block earlier `FieldSet` disappears when a later reachable write to the same definitely non-escaping local root/field makes it observationally dead.

## Goal

- keep DCE effect-sensitive without widening into generic heap-memory reasoning
- reuse the existing local-root authority for definitely non-escaping local boxes
- remove only overwritten local writes that are provably dead before any reachable read/escape boundary

## Decision

- DCE may collect a same-block pending write map keyed by `(local_root, field)`
- when backward scan sees an earlier `FieldSet` for a key that already has a later reachable pending write, the earlier write may disappear
- a reachable `FieldGet` on the same `(local_root, field)` clears that pending overwrite witness
- a reachable escape use on the same `local_root` clears all pending overwrite witnesses for that root
- the surviving later write stays alive when a downstream read still observes that field
- generic `Store`, generic `Load`, `Debug`, and control-flow terminators stay outside this cut

## Acceptance

- same-block overwritten local `FieldSet` instructions disappear on definitely non-escaping local boxes
- a later read of the same field keeps the surviving final write alive
- local read/write behavior from `phase185x` and `phase186x` remains intact
- `tools/checks/dev_gate.sh quick` stays green after the release-artifact sync guard is satisfied

## Exit

- overwritten local field-set pruning is landed
- broader effect-sensitive / partial DCE remains separate backlog
