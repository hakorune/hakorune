# 227x-90 Semantic Simplification Owner Seam SSOT

Status: SSOT

## Decision

- `semantic_simplification` is now the optimizer-facing owner seam for the landed simplification passes
- this first cut bundles:
  - `DCE`
  - `CSE`
- this cut is BoxShape only
  - no new simplification behavior
  - no `SimplifyCFG`
  - no `SCCP`
  - no jump-threading

## Why

- the roadmap treats `semantic simplification bundle` as one layer
- keeping DCE and CSE directly wired in `optimizer.rs` would repeat the same family-specific coupling problem that `phase225x` / `phase226x` already removed from the placement/effect lane
- the next simplification slices need one stable owner seam before any structural CFG work is added

## In Scope

- add a top-level `semantic_simplification` pass entry
- move optimizer wiring for landed DCE/CSE under that entry
- lock focused tests for DCE and CSE behavior through the bundle

## Out of Scope

- `SimplifyCFG`
- `SCCP`
- jump-threading
- memory-effect widening

## Acceptance

1. optimizer calls one simplification owner seam instead of direct DCE/CSE wiring
2. existing DCE and CSE counters still flow through `OptimizationStats`
3. focused bundle tests stay green
4. `tools/checks/dev_gate.sh quick` stays green
