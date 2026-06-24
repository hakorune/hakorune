---
Status: Landed
Date: 2026-05-30
Scope: select the first legacy helper/cache surface to retire now that the selected-method direct-array lane semantic smoke has been refreshed by row408.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-408-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-SEMANTIC-PERF-OWNER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
  - tools/allocator/collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection.py
  - tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_selection_guard.sh
---

# 296x-409 Collection Method Direct Array Lane Legacy Helper Cache Retirement Selection

## Purpose

Select the first legacy helper/cache surface to retire now that the direct-array
path has been refreshed after the selected-method semantic smoke.

This row does not delete code. It chooses the retirement target and keeps the
split between public ArrayBox semantics and DirectArray hot storage explicit
before any retirement implementation is opened.

## Contract

```text
output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-selection-v0
input_contract=collection-method-direct-array-lane-post-semantic-perf-owner-refresh-v0
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

## Mini Task Board

Keep each item small enough for a mini worker. This row is docs/report only.
Do not open implementation. Treat each task below as independently runnable.
Do not bundle multiple files into one worker pass.

### RDS-001: Direct Array Retirement Target

Input:
- `tools/allocator/collection_method_call_direct_array_lane_legacy_helper_cache_retirement_selection.py`

Output:
- short table of the direct-array dominance split
- first retirement target

Acceptance:
- the direct-array lane remains exact-only
- no implementation is proposed

### RDS-002: Split SSOT Boundary Review

Input:
- `docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md`

Output:
- short note on how the public ArrayBox vs DirectArray split stays explicit

Acceptance:
- the split stays visible before any retirement implementation
- no implementation is proposed

### RDS-003: Tests And Route Assertions Review

Input:
- `src/llvm_py/tests/test_collection_method_call.py`
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`

Output:
- short note on the remaining assertions that still anchor the retirement split

Acceptance:
- the tests remain pinning the chosen owner split
- no implementation is proposed

### RDS-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row409 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_selection_guard.sh` passes
- `bash tools/checks/k2_wide_phase296x_arraybox_public_semantics_and_directarray_split_ssot_guard.sh` passes
- `git diff --check` passes

## Decision

DirectArray dominance opens the legacy retirement lane, and the first target is
the exact-array helper backend. Before implementation, the split SSOT keeps
public ArrayBox semantics separate from the DirectArray hot storage owner.

## Acceptance

- row408 remains intact and its smoke surface remains available
- `collection_method_call.py` remains distinct from the boxcall / field sink /
  legacy bridge compatibility surfaces
- the dispatch layer stays thin
- the next selected row is docs-first
- the selected-method lane stays exact-only
- no public ArrayBox handle reinterpretation is introduced

## Forbidden

- no new DirectArray member
- no helper micro-optimization
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`
- no public ArrayBox handle reinterpretation

## Guard

```bash
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_selection_guard.sh
```
