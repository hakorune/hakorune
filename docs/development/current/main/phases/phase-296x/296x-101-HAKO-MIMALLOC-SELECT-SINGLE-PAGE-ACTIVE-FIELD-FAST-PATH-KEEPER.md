---
Status: Current
Date: 2026-05-27
Scope: add an active-page field fast path inside the single-page select route.
Blocker: HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-100-HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-SOURCE-MIR-REFRESH.md
---

# 296x-101 Hako Mimalloc Select Single-Page Active Field Fast Path Keeper

## Purpose

Apply one BoxCount keeper selected by row100:

```text
keeper=select_single_page_active_field_fast_path
keeper_kind=box_count
```

The current workload always hits the single-page active path. Add a narrow
active-page fast path before the generic lifecycle method-call checks, using
the already-cached first page object and preserving the existing generic
fallback for retired/decommitted/unavailable pages.

## Required Output

```text
output_contract=hako-mimalloc-select-single-page-active-field-fast-path-keeper-v0
input_contract=hako-mimalloc-post-select-first-page-cache-source-mir-refresh-v0
keeper=select_single_page_active_field_fast_path
target_method=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
active_field_fast_path_used=1
generic_lifecycle_fallback_preserved=1
proof_summary=ok
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

Do not remove the generic retired/decommitted fallback path. Do not change
multi-page selection policy, provider activation, replacement, hooks, globals,
or winner claims in this row.
