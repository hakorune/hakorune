---
Status: Current
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
