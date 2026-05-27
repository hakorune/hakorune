---
Status: Landed
Date: 2026-05-28
Scope: measure page-local ArrayBox dynamic weight before selecting the next keeper.
Blocker: PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-145-MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT.md
---

# 296x-146 Page Array Dynamic Weight Probe

## Purpose

Separate page-local ArrayBox runtime weight from compiler helper-copy pressure
after post-BoxShape correctness closeout. This row should decide whether the
next keeper should target compiler-side same-module helper call lowering,
allocator page-array surface, or benchmark reset/setup shape.

## Required Output

```text
output_contract=page-array-dynamic-weight-probe-v0
input_contract=mir-builder-post-boxshape-correctness-closeout-v0
operation_repeat
alloc_count
release_count
reset_count
page_acquire_array_get_weight
page_acquire_array_set_weight
page_release_array_get_weight
page_release_array_set_weight
reset_array_set_weight
dynamic_owner
selected_next
summary=ok
```

## Evidence

```text
output_contract=page-array-dynamic-weight-probe-v0
input_contract=mir-builder-post-boxshape-correctness-closeout-v0
operation_repeat=8192
alloc_count=524288
release_count=524288
reset_count=8192
page_capacity=64
page_acquire_array_get_weight=524288
page_acquire_array_set_weight=524288
page_release_array_get_weight=524288
page_release_array_set_weight=1048576
reset_array_get_weight=0
reset_array_set_weight=1572864
total_array_get_weight=1048576
total_array_set_weight=3145728
total_array_weight=4194304
reset_array_weight=1572864
alloc_release_array_weight=2621440
reset_array_weight_percent=37
alloc_release_array_weight_percent=62
dynamic_owner=allocator_page_array_surface
compiler_helper_copy_secondary=1
winner_claim=0
replacement_active=0
selected_next=page_array_keeper_selection
summary=ok
```

Interpretation:

```text
Page-local ArrayBox operations are dynamically large. Reset/setup is visible
but not dominant under the current proof workload: alloc+release page-array
weight is 62% of total page-array operations. The next row should choose a
page-array keeper before returning to same-module helper call lowering.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_page_array_dynamic_weight_probe_guard.sh
```
