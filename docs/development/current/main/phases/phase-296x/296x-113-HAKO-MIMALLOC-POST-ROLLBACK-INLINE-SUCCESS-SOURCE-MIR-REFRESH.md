---
Status: Current
Date: 2026-05-27
Scope: refresh hot-owner selection after rolling back the inline success result keeper.
Blocker: HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-112-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT.md
---

# 296x-113 Hako Mimalloc Post Rollback Inline Success Source/MIR Refresh

## Purpose

Refresh source/MIR and hot-owner rank after row112. The next selection should
avoid both measured non-keepers:

```text
select_single_page_active_field_fast_path
small_alloc_inline_success_result_fast_path
```

## Required Output

```text
output_contract=hako-mimalloc-post-rollback-inline-success-source-mir-refresh-v0
input_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0
method_count
active_method_count
rejected_keeper_count=2
selected_owner
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row. Keep provider activation,
replacement, hooks, globals, and winner claims closed.
