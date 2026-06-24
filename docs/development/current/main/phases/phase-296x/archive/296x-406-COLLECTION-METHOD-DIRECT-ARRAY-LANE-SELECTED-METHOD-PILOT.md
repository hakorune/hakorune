---
Status: Landed
Date: 2026-05-30
Scope: implement the first selected-method direct-array lane pilot in collection_method_call.py after row405 froze the guard surface.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-SELECTED-METHOD-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-405-COLLECTION-METHOD-DIRECT-ARRAY-LANE-GUARD-SURFACE.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/utils/resolver_helpers.py
  - src/llvm_py/tests/test_collection_method_call.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
---

# 296x-406 Collection Method Direct Array Lane Selected-Method Pilot

## Purpose

Implement the first helper-free selected-method direct-array lane pilot.

The selected-method lane remains exact-only and only authorises
`HakoAllocPageModel.acquire_usize/1` when the receiver carries the explicit
`ArrayRepr::DirectI64` fact. Public ArrayBox handles remain on the fallback
path, and the compatibility surfaces stay secondary.

## Contract

```text
output_contract=collection-method-direct-array-lane-selected-method-pilot-v0
input_contract=collection-method-direct-array-lane-guard-surface-v0
workload_id=representative-object-lifecycle-small-block-v0
shared_route_order_surface=collection_method_call.py
direct_array_lane_surface=_lower_direct_array_nativedirect_call
array_fallback_surface=_lower_array_collection_method_call
map_fallback_surface=_lower_map_collection_method_call
compatibility_surface_boxcall=boxcall_runtime_data.py
compatibility_surface_field_sink=field_access.py
compatibility_surface_legacy=mir_call_legacy.py
tests_surface_anchor=test_runtime_data_dispatch_policy.py|test_collection_method_call.py
runtime_data_dispatch_thin_consumer=1
direct_array_lane_exact_only=1
public_arraybox_runtime_surface_secondary=1
compatibility_surfaces_secondary=1
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_array_i64_exact
selected_method_only=1
receiver_origin_fact=resolver.arrayrepr_facts
receiver_origin_fact_value=ArrayRepr::DirectI64
receiver_origin_fact_required=1
receiver_origin_must_be_direct_array=1
public_arraybox_handle_as_direct_buffer_allowed=0
default_backend_emission=0
generic_arraybox_rewrite_allowed=0
runtime_helper_semantics_changes_allowed=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
direct_array_data_offset_bytes=32
direct_array_element_size_bytes=8
direct_array_get_lowering=direct_i64_load_with_oob_zero
direct_array_set_lowering=direct_i64_store_with_append_len_update_and_oob_zero
append_policy_preserved_in_direct_path=1
oob_policy_preserved_in_direct_path=1
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
direct_array_helper_route_reuse_allowed=0
legacy_retirement_now=0
legacy_retirement_policy=defer_until_direct_array_semantic_smoke_and_perf_owner_refresh
python_unit_smoke=ok
selected_method_get_smoke=ok
selected_method_set_smoke=ok
non_selected_method_helper_smoke=ok
selected_method_direct_load_store_open=1
generic_direct_load_store_open=0
native_direct_open=selected_method_only
selected_next=collection_method_call_direct_array_lane_semantic_smoke
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Mini Task Board

Keep each item small enough for a mini worker. This row is still docs/report
only. Do not open implementation. Treat each task below as independently
runnable. Do not bundle multiple files into one worker pass.

### DLP-001: Direct Array Pilot Compare

Input:
- `src/llvm_py/instructions/mir_call/collection_method_call.py`
- `src/llvm_py/utils/resolver_helpers.py`

Output:
- short table of the selected-method direct-array pilot
- short list of likely miss points

Acceptance:
- the direct-array lane remains exact-only
- no implementation is proposed

### DLP-002: Array Fallback Compare

Input:
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the ArrayBox fallback branch
- short list of likely miss points

Acceptance:
- the fallback branch stays fallback-only
- no implementation is proposed

### DLP-003: Tests And Route Assertions Review

Input:
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short note on the remaining assertions that still anchor the direct-array split

Acceptance:
- the tests remain pinning the chosen owner split
- no implementation is proposed

### DLP-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row406 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_selected_method_pilot_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The exact-only direct-array lane selected-method pilot is the next implementation
row after the guard surface:

```text
selected_next=collection_method_call_direct_array_lane_semantic_smoke
selected_reason=the exact-only direct-array lane now needs the selected-method pilot implementation before the semantic smoke row can measure the direct path
```

## Acceptance

- row405 is landed and its guard surface remains intact
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
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_selected_method_pilot_guard.sh
```
