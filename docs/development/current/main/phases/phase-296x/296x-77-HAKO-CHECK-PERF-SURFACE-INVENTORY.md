---
Status: Landed
Date: 2026-05-27
Scope: inventory object lifecycle facade perf surfaces and select the first keeper candidate.
Blocker: HAKO-CHECK-PERF-SURFACE-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-76-HAKO-CHECK-PERF-SURFACE-CONTRACT.md
  - tools/hako_check/README.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
---

# 296x-77 hako_check Perf Surface Inventory

## Purpose

Apply the `hako_check perf-surface` contract to
`lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako` and inventory the
two currently selected parity hot methods:

```text
objectLifecycleSmallAlloc
objectLifecycleReleaseBlock
```

## Expected Candidate

```text
output_contract=hako-check-perf-surface-inventory-v0
input_contract=hako-check-perf-surface-contract-v0
target_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
target_box=HakoAllocObjectLifecycleFacade
target_method=objectLifecycleReleaseBlock
linear_search_candidate=1
suggested_next=release_known_page_fast_path
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

This row selects one keeper candidate. It does not apply the keeper.

## Landed Evidence

```text
output_contract=hako-check-perf-surface-inventory-v0
input_contract=hako-check-perf-surface-contract-v0
target_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
target_box=HakoAllocObjectLifecycleFacade
target_method=objectLifecycleReleaseBlock
linear_search_candidate=1
suggested_next=release_known_page_fast_path
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_check_perf_surface_inventory_guard.sh
```
