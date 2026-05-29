---
Status: Landed
Date: 2026-05-29
Scope: refresh the hot owner after fact-driven supported-storage DirectSlot NativeDirect lowering.
Blocker: POST-DIRECT-SLOT-SUPPORTED-STORAGE-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-344-DIRECT-SLOT-SUPPORTED-STORAGE-NATIVEDIRECT-IMPLEMENTATION.md
  - tools/allocator/direct_slot_post_supported_storage_owner_refresh.py
---

# 296x-345 Post DirectSlot Supported-Storage Owner Refresh

## Purpose

Classify the remaining exact-EXE owner after row344 removed legacy
typed-object field helper calls from the supported DirectSlot storage surface.

The hot owner moved to ArrayBox single-thread exact storage. In particular,
`HashMap` lookup inside the diagnostic Array slot backend dominates together
with `single_thread_store_i64`.

## Contract

```text
output_contract=direct-slot-post-supported-storage-owner-refresh-v0
input_contract=direct-slot-supported-storage-nativedirect-implementation-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
field_helper_pct=0.00
array_store_pct=38.21
array_load_pct=10.67
array_hash_pct=39.55
array_direct_op_pct=4.17
array_slot_helper_pct=2.92
array_total_pct=95.52
hako_method_pct=0.00
selected_boundary=array_single_thread_exact_handle_cache
next_diagnostic=array_single_thread_exact_handle_cache
selected_reason=array_single_thread_hash_lookup_dominates_after_direct_slot_supported_storage
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The next row may optimize only the `single_thread_exact` Array slot backend
lookup shape. It must not change public `ArrayBox` semantics or the default
`safe_rwlock` backend.

```text
selected_owner=array_slot_backend_single_thread_exact_handle_cache
allowed_file=crates/nyash_kernel/src/plugin/array_slot_backend.rs
default_backend_semantics_change=0
public_arraybox_storage_change=0
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_direct_slot_supported_storage_owner_refresh_guard.sh
```
