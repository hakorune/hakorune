# 293x-797 MIMAP-272A Segment Arena Backing Modeled Allocation Apply Inventory

Status: selected current
Date: 2026-05-19

## Decision

Consume an accepted MIMAP-268A allocation-plan report and record a model-only
allocation apply fact before real arena backing allocation opens.

## Context

MIMAP-268A records allocation-plan facts. MIMAP-269A observes diagnostics and
MIMAP-270A closed out the family. The next durable row should prove that an
accepted plan can be applied into scalar/model state without allocating real
arena backing.

## Scope

- Add a scalar/model allocation-apply inventory owner.
- Consume accepted allocation-plan reports only.
- Publish applied token, source/plan identity, allocated backing bytes,
  allocated committed bytes, and remaining source bytes.
- Reject missing/rejected plan reports, invalid apply token, invalid apply
  geometry, and closed substrate requirements.
- Keep this row L2 daily unless it introduces a new backend route shape.

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
