# 293x-793 MIMAP-268A Segment Arena Backing Modeled Allocation Plan Inventory

Status: selected current
Date: 2026-05-19

## Decision

Add a scalar/model allocation-plan inventory after the segment arena backing
source-accounting ReportFields pilot.

## Context

MIMAP-264A records source-accounting facts from an accepted modeled source
bridge. MIMAP-265A observes diagnostics, MIMAP-266A closes that family, and
HAKO-ALLOC-REPORT-RECORD-004 keeps the source-accounting diagnostic report
construction cleaner without changing the returned report box.

The next narrow allocator row should consume an accepted source-accounting
report and publish a modeled arena-backing allocation plan before real arena
allocation opens.

## Scope

- Observe accepted
  `HakoAllocSegmentArenaBackingModeledSourceAccountingReport` values.
- Publish a model-only allocation plan with:
  - plan token
  - planned backing bytes
  - planned committed bytes
  - source capacity / committed / uncommitted facts
  - remaining bytes after the plan
  - closed-substrate requirement flags
- Reject missing/rejected source accounting, invalid plan tokens, invalid
  backing geometry, and any closed-substrate requirement.
- Keep this row inventory-only.

## Stop Lines

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
