---
Status: Current
Date: 2026-05-27
Scope: refresh source/MIR observation after the small-alloc direct select keeper measurement.
Blocker: HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-107-HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-KEEPER-MEASUREMENT.md
---

# 296x-108 Hako Mimalloc Post Small-Alloc Direct Select Source/MIR Refresh

## Purpose

Refresh source/MIR and hot-owner rank after row107. The next selection should
account for the accepted small-alloc direct select keeper and the row101
rejected field shortcut.

## Required Output

```text
output_contract=hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh-v0
input_contract=hako-mimalloc-post-small-alloc-direct-select-keeper-measurement-v0
method_count
active_method_count
rejected_keeper=select_single_page_active_field_fast_path
accepted_keeper=small_alloc_direct_single_page_select_fast_path
selected_owner
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row. Keep provider activation,
replacement, hooks, globals, and winner claims closed.
