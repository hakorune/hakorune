# 293x-798 MIMAP-273A Segment Arena Backing Modeled Allocation Apply Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add observer-only diagnostics for the MIMAP-272A modeled allocation-apply
inventory.

## Context

MIMAP-272A records scalar/model allocation-apply facts from accepted modeled
allocation-plan reports. The next row should summarize the accepted apply,
reject categories, and inactive substrate flags without recording new
allocation-apply rows.

## Scope

- Observe MIMAP-272A modeled allocation-apply inventory counters.
- Publish accepted/rejected summary counts.
- Publish reject-category seen flags for:
  - missing allocation plan
  - rejected allocation plan
  - invalid apply token
  - invalid apply geometry
  - closed substrate requirement
- Mirror last-report apply facts for diagnostics.
- Keep the row observer-only.

## Stop Lines

- No new allocation-apply inventory rows.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

- Added observer-only allocation-apply diagnostics owner and report.
- Added proof app, L2 guard, proof manifest row, check index entry, module
  export, memory README entry, and diagnostics SSOT.
- Verified accepted/rejected inventory summary counters and missing /
  rejected / invalid-token / invalid-geometry / closed-substrate seen flags.
- Verified the observer mirrors the latest allocation-apply report facts
  without recording new allocation-apply rows.
- Kept real pointer residence, pointer-derived lookup, real arena backing,
  segment-map mutation, atomic bitmap execution, OSVM/page-source execution,
  worker/provider activation, and backend matchers inactive.

## Selected Next Row

`MIMAP-274A` segment arena backing modeled allocation apply closeout pack.
