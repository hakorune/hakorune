---
Status: Landed
Date: 2026-05-30
Scope: refresh the perf owner after selected-method ArraySlot NativeDirect semantic smoke and decide whether legacy helper/cache retirement can open.
Blocker: ARRAY-SLOT-NATIVEDIRECT-POST-SEMANTIC-PERF-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-370-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-SEMANTIC-SMOKE.md
  - tools/allocator/array_slot_nativedirect_post_semantic_perf_owner_refresh.py
  - tools/checks/k2_wide_phase296x_array_slot_nativedirect_post_semantic_perf_owner_refresh_guard.sh
---

# 296x-371 ArraySlot NativeDirect Post-Semantic Perf Owner Refresh

## Purpose

Refresh the hot-owner classification after the selected-method ArraySlot
NativeDirect semantic smoke.

The semantic smoke from row370 proved the selected-method DirectArray path is
structurally valid. This row checks whether the DirectArray runtime path now
dominates the hot owner enough to open a legacy helper/cache retirement
selection.

This row does not delete legacy helper/cache code. It only decides whether the
retirement selection can open.

## Contract

```text
output_contract=array-slot-nativedirect-post-semantic-perf-owner-refresh-v0
input_contract=array-slot-nativedirect-selected-method-semantic-smoke-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
attribution_source=perf_callgraph
direct_array_backend_store_pct=...
direct_array_backend_load_pct=...
direct_array_backend_total_pct=...
legacy_field_helper_pct=...
legacy_array_helper_pct=...
legacy_hash_pct=...
legacy_helper_cache_total_pct=...
hako_method_pct=...
direct_array_dominates_legacy_helper_cache=0|1
legacy_helper_cache_retirement_open=0|1
selected_boundary=array_slot_nativedirect_legacy_helper_cache_retirement_selection
next_diagnostic=array_slot_nativedirect_legacy_helper_cache_retirement_selection
selected_next=array_slot_nativedirect_legacy_helper_cache_retirement_selection
selected_reason=direct_array_path_dominates_legacy_helper_cache_after_semantic_smoke
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

The selected-method DirectArray smoke is not enough by itself; this row keeps
the retirement decision explicit. The perf refresh confirmed DirectArray
dominance over the legacy helper/cache surface, so the next row opens legacy
helper/cache retirement selection.

Do not delete the legacy helper/cache paths in this row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_post_semantic_perf_owner_refresh_guard.sh
```
