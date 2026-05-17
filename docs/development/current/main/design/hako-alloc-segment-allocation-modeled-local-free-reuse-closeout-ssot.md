---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-128A segment allocation modeled local-free reuse closeout guard.
Related:
  - docs/development/current/main/phases/phase-293x/293x-632-MIMAP-126A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-ROUTE.md
---

# Hako Alloc Segment Allocation Modeled Local-Free Reuse Closeout SSOT

## Decision

`MIMAP-128A` is a guard-only closeout for the MIMAP-126A local-free reuse
route.

The closed seam is:

```text
released-span report
  -> local-free integration
  -> HakoAllocPageModel.releaseLocal(block_id)
  -> HakoAllocPageModel.acquire(size)
  -> local_free collection count increases
```

This closeout does not add allocator behavior. It freezes the owner/proof/docs
wiring and stop-line set before another allocator row is selected.

## Closed Rows

| Row | Status | Owner |
| --- | --- | --- |
| `MIMAP-126A` | landed | local-free reuse owner, proof app, guard |
| `MIMAP-128A` | landed by this closeout | manifest-backed closeout guard |

## Guard

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh
tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh
tools/checks/run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-reuse-closeout
```

## Stop Lines

The closeout must keep these inactive:

- new allocator behavior;
- direct page array mutation;
- raw pointer residence;
- segment-map pointer membership or lookup;
- arena backing allocation;
- atomic bitmap execution;
- page-source or OSVM execution;
- real thread scheduling or worker spawning;
- source-level concurrency feature changes;
- provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`;
- backend `.inc` app/name matcher.

## Next

```text
MIMAP-129A post-local-free-reuse-closeout row selection
```
