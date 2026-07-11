---
Status: Landed
Date: 2026-05-25
Scope: thin-wrap the MIMAP-268A and MIMAP-272A segment arena backing modeled allocation plan and apply guard roots into impl-backed wrappers.
Related:
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_guard.sh
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_guard.sh
---

# 295x-156 MIMAP-268A and MIMAP-272A Segment Arena Backing Modeled Allocation Plan and Apply Thin-Wrap Batch

## Blocker

```text
MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-WORKLOAD-CONTRACT-295X-001
```

## Decision

Thin-wrap the MIMAP-268A segment arena backing modeled allocation plan guard root
and the MIMAP-272A segment arena backing modeled allocation apply guard root.
The batch keeps the validation semantics unchanged and moves both real shell
bodies into `tools/checks/impl/`.

Selected roots:

- `k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_guard.sh`
- `k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_guard.sh`

## Cleanup

- Keep both root scripts as thin wrappers that exec their impl bodies.
- Keep the allocation-plan and allocation-apply routes unchanged.
- Leave the current mimalloc comparison blocker unchanged.

## Result

The MIMAP-268A and MIMAP-272A guard roots are now easier to scan at the root level.

## Stop Line

This batch does not open real pointer residence, pointer lookup, arena
backing, segment-map mutation, atomic bitmap, OSVM, worker, provider, backend
matcher, or winner claims.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_plan_guard.sh
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_apply_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
