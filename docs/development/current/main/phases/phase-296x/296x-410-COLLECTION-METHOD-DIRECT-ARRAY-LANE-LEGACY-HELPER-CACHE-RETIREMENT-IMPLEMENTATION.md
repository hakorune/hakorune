---
Status: Landed
Date: 2026-05-30
Scope: define the scoped implementation boundary for retiring only single_thread_exact_array_helper_backend after the DirectArray split SSOT and retirement selection are both landed.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-409-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
  - docs/development/current/main/phases/phase-296x/296x-411-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SEMANTIC-SMOKE.md
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_implementation_guard.sh
---

# 296x-410 Collection Method Direct Array Lane Legacy Helper Cache Retirement Implementation

## Purpose

Define the exact implementation boundary for the first legacy helper/cache
retirement slice.

This row does not delete ArrayBox public semantics, handle-entry cache, or
public helper fast lanes. It only narrows the implementation scope so the next
row can smoke the obsolete exact-array helper backend replacement without
widening the lane.

## Contract

```text
output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-implementation-v0
input_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-selection-v0
selected_method=HakoAllocPageModel.acquire_usize/1
implementation_scope=single_thread_exact_array_helper_backend
selected_retirement_target=single_thread_exact_array_helper_backend
selected_backend=direct_array_i64_exact
scoped_helper_backend_retirement_open=1
handle_entry_cache_retirement_deferred=1
public_helper_fast_lane_retirement_deferred=1
public_arraybox_semantics_preserved=1
public_arraybox_behavior_deletion=0
handle_entry_cache_deletion=0
public_helper_abi_removal=0
directarray_helper_route_fail_fast_until_scoped_replaced=1
direct_array_materialization_route_required=1
selected_boundary=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_implementation
next_diagnostic=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_implementation
selected_next=collection_method_call_direct_array_lane_legacy_helper_cache_retirement_semantic_smoke
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The implementation boundary is intentionally narrow:

- the only scoped retirement target is `single_thread_exact_array_helper_backend`
- `array_slot_handle_entry_cache` stays deferred
- public helper fast lanes stay deferred
- the DirectArray helper route remains fail-fast until the scoped retirement
  path is replaced

This row is the scoped retirement slice. The next row performs the semantic
smoke before any perf owner refresh.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_implementation_guard.sh
```
