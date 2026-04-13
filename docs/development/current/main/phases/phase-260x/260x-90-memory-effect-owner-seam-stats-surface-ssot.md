# Memory-Effect Owner Seam and Stats Surface SSOT

Status: SSOT
Date: 2026-04-13
Owner: `phase-260x`

## Purpose

- lift the landed private-carrier memory cleanup out of the simplification bundle
- make the memory-effect layer a first-class top-level owner seam
- give the optimizer a dedicated stats bucket for memory-effect work

## Decisions

### Owner seam

- `memory_effect::apply(module)` is the current owner for landed memory-sensitive `Load` / `Store` cleanup.
- `semantic_simplification::apply(module)` keeps pure DCE / CSE / CFG simplification only.
- shared analysis helpers may remain reusable, but the owner and scheduling live at the top-level `memory-effect layer`.

### Stats surface

- `OptimizationStats.memory_effect_optimizations` counts the instructions removed by the memory-effect owner pass.
- the optimizer merges that bucket alongside the other top-level pass buckets.

### Scope

- in scope:
  - dead private-carrier `Load` pruning
  - same-block private-carrier store-to-load forwarding
  - same-block private-carrier redundant load elimination
  - overwritten private-carrier `Store` pruning
- out of scope for this phase:
  - broader cross-block DSE widening
  - hoist / sink legality

## Acceptance

- optimizer schedule has a dedicated memory-effect stage after semantic simplification
- the stats bucket is observable in optimizer totals and display output
- direct pass tests cover at least one `Load` pruning and one overwritten `Store` pruning case
- direct pass tests cover a same-block store-to-load forwarding case
- direct pass tests cover a same-block redundant load elimination case
- direct pass tests cover the immediate-successor overwritten-store widening case

## Next Cut

- `M3` overwritten-store / DSE widening beyond the landed same-block cut
