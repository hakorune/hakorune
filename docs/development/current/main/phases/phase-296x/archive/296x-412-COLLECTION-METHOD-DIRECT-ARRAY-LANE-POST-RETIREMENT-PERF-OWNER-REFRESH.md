---
Status: Landed
Date: 2026-05-30
Scope: refresh the post-retirement perf owner after the scoped retirement smoke and decide whether the lane can move to ArrayRepr design.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-RETIREMENT-PERF-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-411-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE.md
  - docs/development/current/main/phases/phase-296x/296x-410-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
  - docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md
  - tools/allocator/collection_method_call_direct_array_lane_post_retirement_perf_owner_refresh.py
  - tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_post_retirement_perf_owner_refresh_guard.sh
---

# 296x-412 Collection Method Direct Array Lane Post-Retirement Perf Owner Refresh

## Purpose

Refresh the hot-owner classification after the scoped retirement smoke.

The smoke from row411 proved the retirement slice is structurally valid. This
row checks whether the DirectArray runtime path now dominates enough to move
the lane toward `ArrayRepr` design rather than reopening helper micro-
optimization.

This row does not delete code. It only classifies the owner and selects the
next design row.

## Contract

```text
output_contract=collection-method-direct-array-lane-post-retirement-perf-owner-refresh-v0
input_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-semantic-smoke-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
selected_method=HakoAllocPageModel.acquire_usize/1
direct_array_backend_store_pct=...
direct_array_backend_load_pct=...
direct_array_backend_direct_op_pct=...
direct_array_backend_total_pct=...
legacy_field_helper_pct=...
legacy_array_helper_pct=...
arraybox_public_helper_pct=...
array_slot_backend_safe_pct=...
array_handle_cache_pct=...
arraybox_runtime_total_pct=...
legacy_hash_pct=...
legacy_helper_cache_total_pct=...
hako_method_pct=...
direct_array_dominates_legacy_helper_cache=0|1
array_repr_design_open=1
selected_boundary=array_repr_design_row
next_diagnostic=array_repr_design_row
selected_next=array_repr_design_row
selected_reason=direct_array_path_ready_for_arrayrepr_design_after_retirement_smoke
legacy_retirement_candidate_0=single_thread_exact_array_helper_backend
legacy_retirement_candidate_1=array_slot_handle_entry_cache
legacy_retirement_candidate_2=array_slot_public_helper_fast_lane
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Mini Task Board

Keep each item small enough for a mini worker. This row is code-backed perf
owner refresh, not a docs-only marker.
Do not open implementation. Treat each task below as independently runnable.
Do not bundle multiple files into one worker pass.

### DPF-001: Direct Array Perf Compare

Input:
- `tools/allocator/collection_method_call_direct_array_lane_post_retirement_perf_owner_refresh.py`

Output:
- short table of the direct-array lane perf split
- short list of likely miss points

Acceptance:
- the direct-array lane remains exact-only
- no implementation is proposed

### DPF-002: Legacy Helper/Cache Compare

Input:
- `tools/allocator/collection_method_call_direct_array_lane_post_retirement_perf_owner_refresh.py`

Output:
- short table of the legacy helper/cache split
- short list of likely miss points

Acceptance:
- the retirement decision stays explicit
- no implementation is proposed

### DPF-003: Tests And Route Assertions Review

Input:
- `src/llvm_py/tests/test_collection_method_call.py`
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`

Output:
- short note on the remaining assertions that still anchor the retirement split

Acceptance:
- the tests remain pinning the chosen owner split
- no implementation is proposed

### DPF-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row412 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_post_retirement_perf_owner_refresh_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The selected-method direct-array lane smoke is not enough by itself; this row keeps
the retirement decision explicit. The perf refresh will decide whether the
DirectArray path dominates the legacy helper/cache surface, and if so it will
open the `ArrayRepr` design row next.

Do not delete the legacy helper/cache paths in this row.

## Acceptance

- row411 is landed and its smoke surface remains intact
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
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_post_retirement_perf_owner_refresh_guard.sh
```
