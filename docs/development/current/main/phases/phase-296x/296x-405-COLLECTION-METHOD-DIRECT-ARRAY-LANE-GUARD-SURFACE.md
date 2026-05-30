---
Status: Landed
Date: 2026-05-30
Scope: define the direct-array lane guard surface before any selected-method implementation opens.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-404-COLLECTION-METHOD-DIRECT-ARRAY-LANE-OWNER-SELECTION.md
  - docs/development/current/main/investigations/phase296x-404-dalo001-direct-array-lane-compare.md
  - docs/development/current/main/investigations/phase296x-404-dalo002-array-fallback-compare.md
  - docs/development/current/main/investigations/phase296x-404-dalo003-compatibility-surfaces-compare.md
  - docs/development/current/main/investigations/phase296x-404-dalo004-tests-and-route-assertions-review.md
  - docs/development/current/main/investigations/phase296x-404-dalo005-next-owner-selection.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/utils/resolver_helpers.py
  - src/llvm_py/instructions/boxcall_runtime_data.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/mir_call_legacy.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - src/llvm_py/tests/test_collection_method_call.py
---

# 296x-405 Collection Method Direct Array Lane Guard Surface

## Purpose

Row404 selected the exact-only direct-array lane as the remaining
highest-leverage owner. Freeze the guard surface that authorizes the first
selected-method implementation attempt while keeping compatibility surfaces and
fallbacks secondary.

## Contract

```text
output_contract=collection-method-direct-array-lane-guard-surface-v0
input_contract=collection-method-direct-array-lane-owner-selection-v0
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
selected_representation=NativeDirect
selected_storage_substrate=DirectArrayI64BufferV0
receiver_origin_fact=resolver.direct_array_i64_ids
receiver_origin_fact_required=1
receiver_origin_must_be_direct_array=1
selected_method_only=1
public_arraybox_handle_as_direct_buffer_allowed=0
default_backend_emission=0
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
direct_array_helper_route_reuse_allowed=0
arraybox_items_rwlock_exposure=0
array_slot_cache_vec_exposure=0
by_name_hako_alloc_special_case=0
fallback_boundary=explicit_public_arraybox_snapshot_handle
fallback_boundary_required=1
planned_erased_helper_ops=2
planned_added_helper_ops=0
planned_net_helper_delta=2
planned_net_helper_delta_positive=1
selected_next=collection_method_call_direct_array_lane_selected_method_pilot
selected_reason=the exact-only direct-array lane now needs a guard surface that can authorize one selected-method pilot without reopening the shared route order or the compatibility fallback surfaces
implementation_open=0
optimization_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
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

### DLG-001: Direct Array Lane Compare

Input:
- [DALO-001 direct array lane compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo001-direct-array-lane-compare.md>)
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the exact-only direct-array lane
- short list of likely miss points

Acceptance:
- the direct-array lane remains exact-only
- no implementation is proposed

### DLG-002: Fallback Compare

Input:
- [DALO-002 array fallback compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo002-array-fallback-compare.md>)
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the ArrayBox and MapBox fallback branches
- short list of likely miss points

Acceptance:
- the fallback branches stay fallback-only
- no implementation is proposed

### DLG-003: Compatibility Surfaces Compare

Input:
- [DALO-003 compatibility surfaces compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo003-compatibility-surfaces-compare.md>)
- `src/llvm_py/instructions/boxcall_runtime_data.py`
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call_legacy.py`

Output:
- short table of the compatibility surfaces
- short list of likely miss points

Acceptance:
- the compatibility surfaces stay secondary
- no implementation is proposed

### DLG-004: Tests And Route Assertions Review

Input:
- [DALO-004 tests and route assertions review](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo004-tests-and-route-assertions-review.md>)
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short note on the remaining assertions that still anchor the split

Acceptance:
- the tests remain pinning the chosen owner split
- no implementation is proposed

### DLG-005: Next Owner Selection

Input:
- DLG-001 through DLG-004 outputs

Output:
- choose exactly one next owner

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### DLG-006: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row405 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_guard_surface_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The exact-only direct-array lane needs a guard surface before the first selected
method implementation pilot:

```text
selected_next=collection_method_call_direct_array_lane_selected_method_pilot
selected_reason=the exact-only direct-array lane now needs a guard surface that can authorize one selected-method pilot without reopening the shared route order or the compatibility fallback surfaces
```

## Acceptance

- row404 is landed and its direct-array lane selection remains intact
- `collection_method_call.py` remains distinct from the boxcall / field sink / legacy bridge compatibility surfaces
- the dispatch layer stays thin
- the next selected row is docs-first
- no implementation is opened

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
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_guard_surface_guard.sh
```
