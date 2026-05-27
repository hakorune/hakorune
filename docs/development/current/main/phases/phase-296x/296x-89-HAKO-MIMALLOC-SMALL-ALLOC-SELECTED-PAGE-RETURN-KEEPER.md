---
Status: Current
Date: 2026-05-27
Scope: apply the selected small-alloc page-return keeper without widening allocator activation or winner claims.
Blocker: HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-RETURN-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-88-HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-89 Hako Mimalloc Small-Alloc Selected Page Return Keeper

## Purpose

Apply one narrow `.hako` keeper selected by row88:

```text
keeper=small_alloc_selected_page_return_reuse
keeper_kind=box_count
```

`HakoAllocObjectLifecyclePageQueue.selectPage()` already returns the selected
page. `HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1` currently
ignores that return value and performs another `pages.get(selected_index)` in a
caller-repeated hot method. Reuse the returned page instead.

## Required Output

```text
output_contract=hako-mimalloc-small-alloc-selected-page-return-keeper-v0
input_contract=hako-mimalloc-multi-method-source-mir-observation-v0
keeper
target_method
selected_page_return_reused=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not change release lookup, multi-page selection policy, provider activation,
process replacement, hooks, globals, or winner claims in this row.
