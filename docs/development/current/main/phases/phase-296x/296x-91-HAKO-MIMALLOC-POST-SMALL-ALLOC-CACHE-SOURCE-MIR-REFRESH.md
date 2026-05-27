---
Status: Current
Date: 2026-05-27
Scope: refresh source/MIR observation after the small-alloc selected-page cache keeper before selecting another keeper.
Blocker: HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-90-HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-91 Hako Mimalloc Post Small-Alloc Cache Source/MIR Refresh

## Purpose

Refresh source/MIR observation after row89 removed the caller-repeated
`pages.get(selected_index)` from `objectLifecycleSmallAlloc/1`.

The next selection must be based on current source/MIR shape, not the pre-row89
array access evidence.

## Required Output

```text
output_contract=hako-mimalloc-post-small-alloc-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-small-alloc-cache-keeper-measurement-v0
method_count
confirmed_source_mir_risk_count
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement the next keeper in this refresh row.
