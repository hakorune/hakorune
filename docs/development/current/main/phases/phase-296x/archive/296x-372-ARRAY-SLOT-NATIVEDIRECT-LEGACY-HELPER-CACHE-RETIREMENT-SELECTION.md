---
Status: Landed
Date: 2026-05-30
Scope: select the first legacy helper/cache retirement target after DirectArray dominance was confirmed by row371.
Blocker: ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-371-ARRAY-SLOT-NATIVEDIRECT-POST-SEMANTIC-PERF-OWNER-REFRESH.md
  - tools/allocator/array_slot_nativedirect_legacy_helper_cache_retirement_selection.py
  - tools/checks/k2_wide_phase296x_array_slot_nativedirect_legacy_helper_cache_retirement_selection_guard.sh
---

# 296x-372 ArraySlot NativeDirect Legacy Helper Cache Retirement Selection

## Purpose

Select the first legacy helper/cache surface to retire now that row371 has
confirmed the DirectArray path dominates the hot owner.

This row does not delete code. It chooses the retirement target and opens the
next implementation row only after direct dominance is confirmed.

## Contract

```text
output_contract=array-slot-nativedirect-legacy-helper-cache-retirement-selection-v0
input_contract=array-slot-nativedirect-post-semantic-perf-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
attribution_source=perf_callgraph
direct_array_backend_store_pct=...
direct_array_backend_load_pct=...
direct_array_backend_direct_op_pct=...
direct_array_backend_total_pct=...
legacy_field_helper_pct=...
legacy_array_helper_pct=...
legacy_hash_pct=...
legacy_helper_cache_total_pct=...
hako_method_pct=...
direct_array_dominates_legacy_helper_cache=0|1
legacy_helper_cache_retirement_open=0|1
selected_retirement_candidate=single_thread_exact_array_helper_backend|array_slot_handle_entry_cache|array_slot_public_helper_fast_lane
selected_retirement_reason=direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke
selected_boundary=arraybox_public_semantics_and_directarray_split_ssot
next_diagnostic=arraybox_public_semantics_and_directarray_split_ssot
selected_next=arraybox_public_semantics_and_directarray_split_ssot
legacy_retirement_candidate_0=single_thread_exact_array_helper_backend
legacy_retirement_candidate_1=array_slot_handle_entry_cache
legacy_retirement_candidate_2=array_slot_public_helper_fast_lane
legacy_retirement_now=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

DirectArray dominance opens the legacy retirement lane, and the first target is
the exact-array helper backend. Before implementation, the next row freezes the
ArrayBox public-semantics versus DirectArray hot-storage split so the retirement
does not accidentally remove public ArrayBox behavior.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_legacy_helper_cache_retirement_selection_guard.sh
```
