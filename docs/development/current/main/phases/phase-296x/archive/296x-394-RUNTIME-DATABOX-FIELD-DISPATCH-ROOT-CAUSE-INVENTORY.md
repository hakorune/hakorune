---
Status: Landed
Date: 2026-05-30
Scope: inventory the RuntimeDataBox field dispatch root cause after row393 located the producer / consumer / carrier / miss-point split and before any new implementation owner opens.
Blocker: RUNTIME-DATA-DISPATCH-FIELD-ROUTE-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-393-TYPED-OBJECT-LEGACY-FIELD-HELPER-CALLSITE-INVENTORY.md
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/mir_call/collection_method_call.py
---

# 296x-394 RuntimeDataBox Field Dispatch Root Cause Inventory

## Purpose

Row393 pinned the typed-object legacy field helper callsites and isolated the
remaining miss to the RuntimeDataBox field dispatch path. Inventory the field
dispatch root cause file by file, keep the field sink and dispatch map split
narrow, and choose exactly one next durable owner.

## Contract

```text
output_contract=runtime-databox-field-dispatch-root-cause-inventory-v0
input_contract=typed-object-legacy-field-helper-callsite-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
field_access_sink_present=1
runtime_data_dispatch_map_abi_present=1
runtime_databox_field_get_sink=RuntimeDataBox.getField
runtime_databox_field_set_sink=RuntimeDataBox.setField
runtime_databox_field_dispatch_is_not_direct_array=1
selected_next=runtime_data_dispatch_field_route_inventory
selected_reason=field_access_falls_back_to_runtime_databox_map_abi_and_the_dispatch_surface_still_needs_route_attribution
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
only. Do not open implementation.
Treat each task below as independently runnable. Do not bundle multiple files
into one worker pass.

### RDI-001: Field Access Sink Inventory

Input:
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the explicit `RuntimeDataBox.getField/setField` sinks
- short list of likely miss points

Acceptance:
- the RuntimeDataBox sink is pinned
- no DirectI64 consumer claim is made

### RDI-002: Runtime Data Dispatch Inventory

Input:
- `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`

Output:
- short table of the explicit map ABI dispatch entries
- short list of likely miss points

Acceptance:
- the field route map is pinned
- no direct-array fast path claim is made

### RDI-003: Route Boundary Note

Input:
- RDI-001 through RDI-002 outputs

Output:
- short note that explains whether the miss is the sink, the dispatch map, or
  both together

Acceptance:
- the next owner is selected exactly once
- no implementation is proposed

### RDI-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row394 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_databox_field_dispatch_root_cause_inventory_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The remaining miss point is now specific enough to treat as a RuntimeDataBox
field-dispatch route question rather than a typed-object helper attribution
question:

```text
selected_next=runtime_data_dispatch_field_route_inventory
selected_reason=field_access_falls_back_to_runtime_databox_map_abi_and_the_dispatch_surface_still_needs_route_attribution
```

## Acceptance

- row393 real callsite inventory is the input
- `RuntimeDataBox.getField/setField` stay distinct from DirectArray
- the dispatch map remains visible as the route carrier
- next selected row is docs-first
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
bash tools/checks/k2_wide_phase296x_runtime_databox_field_dispatch_root_cause_inventory_guard.sh
```
