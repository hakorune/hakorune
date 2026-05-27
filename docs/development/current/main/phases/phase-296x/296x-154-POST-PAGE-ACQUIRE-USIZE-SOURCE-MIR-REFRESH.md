---
Status: Current
Date: 2026-05-28
Scope: refresh source/MIR after the small-alloc acquire_usize fast path measurement.
Blocker: POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-153-POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT.md
---

# 296x-154 Post Page Acquire Usize Source/MIR Refresh

## Purpose

Refresh source/MIR after measuring the acquire_usize fast path keeper, then
select the next owner without mixing page-model keepers and compiler helper-copy
work in the same row.

## Required Output

```text
output_contract=post-page-acquire-usize-source-mir-refresh-v0
input_contract=post-page-acquire-usize-fast-path-measurement-v0
active_owner
selected_next
summary=ok
```
