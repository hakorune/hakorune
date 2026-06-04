---
Status: Current
Date: 2026-05-31
Scope: select the next mimalloc source-level target after the owner refresh without reopening the direct-path lane.
Blocker: MIMALLOC-SOURCE-LEVEL-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-414-MIMALLOC-SOURCE-LEVEL-OWNER-REFRESH.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-route-taxonomy-ssot.md
  - docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
---

# 296x-415 Mimalloc Source-Level Owner Selection

## Purpose

Select the next single source-level target after the row414 refresh.

The direct-path lane stays closed. `object_lifecycle_facade` remains the top
source-level owner surface, so the next diagnostic should inventory its source
shape inside the active mimalloc working card instead of reopening any helper,
substrate, or inventory-only row.

## Contract

```text
output_contract=mimalloc-source-level-owner-selection-v0
input_contract=mimalloc-source-level-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0

source_level_owner_surface=object_lifecycle_facade
source_level_owner_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
source_level_owner_dominant=1
typed_object_legacy_field_helper_open=0
runtime_databox_consumer_surface_open=0
public_arraybox_runtime_surface_open=0
directarray_optional_member_open=0
result_capsule_value_aggregate_open=0
page_model_page_queue_open=0
new_fast_path_open=0
new_fast_path_owner=none
return_to_mimalloc_source_level=1
selected_boundary=object_lifecycle_facade_source_shape_inventory
next_diagnostic=object_lifecycle_facade_source_shape_inventory
selected_next=object_lifecycle_facade_source_shape_inventory
selected_reason=object_lifecycle_facade_remains_top_source_level_owner_surface_after_refresh

open_new_fast_path_only_if_positive_net_helper_delta=1
open_new_fast_path_only_if_perf_owner_pct_above_threshold=1
open_new_fast_path_only_if_selected_callsite_or_family=1
open_new_fast_path_only_if_no_recent_nonkeeper=1
open_new_fast_path_only_if_no_silent_fallback=1

optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

`object_lifecycle_facade` remains the source-level owner surface. The next
diagnostic should inventory its source shape inside the active mimalloc work
instead of creating another inventory-only row. The direct-path lane stays
closed.

## Current Workstream Note

HostAllocatorV0 provider-host refresh is recorded in the active workstream.
The provider-host normal route now uses claim-mainline mode when free/realloc/
usable-size claims are bound, leaving shim pointer tracking as a compatibility
fallback only. The next provider-boundary owner is threaded provider
claim-mainline evidence. That evidence keeps the TLS recursion guard as a
keeper but does not select host-backed provider C-shape work for thread
performance; the next threaded owner is thread-local or pure-provider allocator
shape. Provider activation, product replacement, hooks, global allocator, and
winner claims remain closed.

Route taxonomy is mandatory for this note:

```text
provider_ldpreload_measurement_route=provider_host_adapter_ldpreload
provider_ldpreload_hako_hot_path_claim=0
hako_mimalloc_thread_hot_path_claim=0
```

This is provider ABI / shim / host-backed adapter evidence, not `.hako`
mimalloc object-lifecycle thread hot-path evidence.

Next docs/report task:

```text
task_id=TYPEROUTE-001
ssot=docs/development/current/main/design/type-abi-route-descriptor-plane-ssot.md
behavior_change=0
selected_next_action=add_type_abi_route_descriptor_report_boundary_without_changing_allocator_execution
```

## Forbidden

- no new DirectArray member
- no helper micro-optimization
- no generic typed-field residence retry
- no RuntimeDataBox fallback widening
- no public ArrayBox handle reinterpretation
- no provider activation
- no allocator replacement
- no hook installation
- no `#[global_allocator]`

## Guard

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_source_level_owner_refresh_guard.sh
```
