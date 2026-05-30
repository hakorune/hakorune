---
Status: Landed
Date: 2026-05-30
Scope: inventory the RuntimeDataBox field route policy after row394 pinned the field sink and dispatch map ABI.
Blocker: RUNTIME-DATA-DISPATCH-ROUTE-POLICY-CONTRACT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-394-RUNTIME-DATABOX-FIELD-DISPATCH-ROOT-CAUSE-INVENTORY.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/field_access.py
---

# 296x-395 Runtime Data Dispatch Field Route Inventory

## Purpose

Row394 pinned the RuntimeDataBox field sink and dispatch map ABI. The remaining
question is whether the field-route policy in `select_runtime_data_call_spec`
and `lower_runtime_data_field_call` is now a stable boundary or whether it
still needs its own policy split. Inventory the field-route boundary file by
file, keep the sink and map ABI split narrow, and choose exactly one next
durable owner.

## Contract

```text
output_contract=runtime-data-dispatch-field-route-inventory-v0
input_contract=runtime-databox-field-dispatch-root-cause-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
runtime_data_field_methods_present=1
runtime_data_field_route_policy_present=1
runtime_data_select_call_spec_present=1
field_access_sink_present=1
runtime_databox_field_dispatch_is_not_direct_array=1
selected_next=runtime_data_dispatch_route_policy_inventory
selected_reason=field_route_map_and_sink_are_pinned_but_the_route_policy_surface_still_needs_its_own_inventory
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

### RFI-001: Runtime Data Field Route Map Inventory

Input:
- `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`

Output:
- short table of the explicit field-route map entries and policy surface
- short list of likely miss points

Acceptance:
- the field route map is pinned
- no direct-array fast path claim is made

### RFI-002: RuntimeDataBox Field Sink Check

Input:
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the explicit `RuntimeDataBox.getField/setField` sinks
- short list of likely miss points

Acceptance:
- the RuntimeDataBox sink is pinned
- no new Array or DirectArray claim is made

### RFI-003: Route Policy Boundary Note

Input:
- RFI-001 through RFI-002 outputs

Output:
- short note that explains whether the field-route boundary is the map ABI,
  the sink, or the route policy split

Acceptance:
- the next owner is selected exactly once
- no implementation is proposed

### RFI-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row395 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_field_route_inventory_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The route map and the sink are both visible, so the remaining ambiguity is the
route-policy surface itself:

```text
selected_next=runtime_data_dispatch_route_policy_inventory
selected_reason=field_route_map_and_sink_are_pinned_but_the_route_policy_surface_still_needs_its_own_inventory
```

## Acceptance

- row394 real root-cause inventory is the input
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
bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_field_route_inventory_guard.sh
```
