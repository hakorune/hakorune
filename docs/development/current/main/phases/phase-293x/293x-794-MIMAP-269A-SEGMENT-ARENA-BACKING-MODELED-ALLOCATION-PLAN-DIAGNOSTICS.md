# 293x-794 MIMAP-269A Segment Arena Backing Modeled Allocation Plan Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add observer-only diagnostics for the MIMAP-268A modeled allocation-plan
inventory.

## Context

MIMAP-268A records scalar/model allocation-plan facts from accepted source
accounting reports. The next row should summarize the accepted plan, reject
categories, and inactive substrate flags without recording new allocation-plan
rows.

## Scope

- Observe MIMAP-268A modeled allocation-plan inventory counters.
- Publish accepted/rejected summary counts.
- Publish reject-category seen flags for:
  - missing source accounting
  - rejected source accounting
  - invalid plan token
  - invalid plan geometry
  - closed substrate requirement
- Mirror last-report plan facts for diagnostics.
- Keep the row observer-only.

## Stop Lines

- No new allocation-plan inventory rows.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added observer-only allocation-plan diagnostics owner and report.
- Added proof app, L2 guard, proof manifest row, check index entry, module
  export, memory README entry, and diagnostics SSOT.
- Verified accepted/rejected inventory summary counters and missing /
  rejected / invalid-token / invalid-geometry / closed-substrate seen flags.
- Verified the observer mirrors the latest allocation-plan report facts
  without recording new allocation-plan rows.
- Kept real pointer residence, pointer-derived lookup, real arena backing,
  segment-map mutation, atomic bitmap execution, OSVM/page-source execution,
  worker/provider activation, and backend matchers inactive.

## Selected Next Row

`MIMAP-270A` segment arena backing modeled allocation plan closeout pack.
