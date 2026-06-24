---
Status: Landed
Date: 2026-05-29
Scope: refresh the hot owner after the Array single-thread exact handle cache keeper.
Blocker: POST-ARRAY-HANDLE-CACHE-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-346-ARRAY-SINGLE-THREAD-EXACT-HANDLE-CACHE.md
  - tools/allocator/array_post_handle_cache_owner_refresh.py
---

# 296x-347 Post Array Handle Cache Owner Refresh

## Purpose

Classify the remaining owner after row346 removed `HashMap` lookup from the
diagnostic Array single-thread exact backend.

The hash owner is gone. The remaining large owner is the Array slot helper
boundary itself: `single_thread_store_i64`, `single_thread_load_encoded_i64`,
and the exported Array slot helper call surface. The next row should design an
ArraySlot NativeDirect guard surface instead of adding another runtime helper
micro-optimization.

## Contract

```text
output_contract=array-post-handle-cache-owner-refresh-v0
input_contract=array-single-thread-exact-handle-cache-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
array_store_pct=45.00
array_load_pct=12.87
array_hash_pct=0.00
array_slot_helper_pct=6.11
array_total_pct=63.98
hako_method_pct=35.71
selected_boundary=array_slot_nativedirect_guard_surface
next_diagnostic=array_slot_nativedirect_guard_surface
selected_reason=array_helper_call_boundary_dominates_after_hash_removed
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The next row should define a guard surface for ArraySlot NativeDirect lowering.
It must stay fact-driven and keep public `ArrayBox` semantics intact.

```text
selected_owner=array_slot_nativedirect_guard_surface
rejected=runtime_helper_internal_fast_lane_repeat
rejected=public_arraybox_storage_change
rejected=hako_source_workaround
required=positive_net_helper_delta
required=exact_i64_array_slot_storage_facts
required=silent_fallback_forbidden
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_array_handle_cache_owner_refresh_guard.sh
```
