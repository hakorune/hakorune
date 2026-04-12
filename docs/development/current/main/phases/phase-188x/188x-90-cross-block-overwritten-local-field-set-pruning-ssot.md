# 188x-90: cross-block overwritten local field-set pruning SSOT

Status: SSOT
Date: 2026-04-12
Scope: extend the landed local overwritten-write DCE slice across one reachable linear CFG edge without widening into loops, merges, or generic heap-memory reasoning.

## Goal

- keep DCE structural and local-root based
- allow one-step partial DCE across split basic blocks after the same-block overwrite cut
- avoid unsound loop-carried or merge-carried overwrite folding

## Decision

- DCE may propagate pending overwritten-write witnesses only across a reachable linear edge
- a linear edge means:
  - source block has exactly one reachable successor
  - successor block has exactly one reachable predecessor
  - dominator query confirms the successor is not a backedge header for that source
- the propagated witness is still `(local_root, field)`
- a `FieldGet` on that field clears the pending witness
- any reachable escape use on that local root clears all pending witnesses for that root
- generic `Store`, generic `Load`, `Debug`, terminators, loop backedges, and merge edges stay outside this cut

## Acceptance

- overwritten local `FieldSet` instructions disappear across one reachable linear edge
- a successor-block read before the overwriting write keeps the predecessor write alive
- same-block overwrite behavior from `phase187x` stays intact
- loop backedges and merge edges remain intact
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- linear-edge overwritten local field-set pruning is landed
- broader effect-sensitive / partial DCE remains separate backlog
