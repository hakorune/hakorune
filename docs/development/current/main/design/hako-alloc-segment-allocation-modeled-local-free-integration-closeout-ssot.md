---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-121A segment allocation modeled local-free integration closeout guard.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-integration-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout-ssot.md
---

# Hako Alloc Segment Allocation Modeled Local-Free Integration Closeout SSOT

## Decision

`MIMAP-121A` is a guard-only closeout for the MIMAP-119A local-free integration
route.

The closed seam is:

```text
released-span report
  -> local-free candidate ledger
  -> local-free apply-plan ledger
  -> local-free page apply route
  -> explicit HakoAllocPageModel.releaseLocal(block_id)
```

This closeout does not add allocator behavior. It freezes the owner/proof/docs
wiring and stop-line set before another allocator row is selected.

## Closed Rows

| Row | Status | Owner |
| --- | --- | --- |
| `MIMAP-119A` | landed | local-free integration owner, proof app, guard, SSOT |
| `MIMAP-121A` | landed by this closeout | manifest-backed closeout guard |

## Guard

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_closeout_guard.sh
tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_closeout_guard.sh
tools/checks/run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-integration-closeout
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
MIMAP-122A post-local-free-integration-closeout row selection
```
