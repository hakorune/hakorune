---
Status: Landed
Date: 2026-05-30
Scope: smoke the selected-method direct-array lane after the pilot implementation landed in collection_method_call.py.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-SEMANTIC-SMOKE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-406-COLLECTION-METHOD-DIRECT-ARRAY-LANE-SELECTED-METHOD-PILOT.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/utils/resolver_helpers.py
  - src/llvm_py/tests/test_collection_method_call.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
---

# 296x-407 Collection Method Direct Array Lane Semantic Smoke

## Purpose

Smoke the selected-method direct-array lane after row406 landed the pilot.

This row verifies the Python lowering route and the Rust DirectArrayI64 runtime
substrate together, while keeping perf refresh and legacy helper/cache
retirement closed for now.

## Contract

```text
output_contract=collection-method-direct-array-lane-semantic-smoke-v0
input_contract=collection-method-direct-array-lane-selected-method-pilot-v0
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
python_lowering_smoke=ok
rust_direct_array_substrate_smoke=ok
direct_array_get_set_lowering_smoke=ok
direct_array_append_oob_storage_smoke=ok
direct_array_materialization_snapshot_smoke=ok
public_arraybox_handle_separation_smoke=ok
default_public_arraybox_helper_path_preserved=1
silent_fallback_allowed=0
helper_load_writeback_substitution_allowed=0
direct_array_helper_route_reuse_allowed=0
legacy_retirement_now=0
legacy_retirement_policy=defer_until_post_semantic_perf_owner_refresh
selected_next=collection_method_call_direct_array_lane_post_semantic_perf_owner_refresh
implementation_open=0
optimization_open=0
perf_measurement_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Commands

```bash
PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_collection_method_call src.llvm_py.tests.test_runtime_data_dispatch_policy
HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact cargo test -p nyash_kernel direct_array_i64 --lib -- --nocapture
```

## Decision

The selected-method direct-array lane is structurally smoke-clean. The next row
is a perf owner refresh, not legacy deletion. Helper/cache retirement still
needs direct evidence that the DirectArray path has taken over the hot owner.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_semantic_smoke_guard.sh
```
