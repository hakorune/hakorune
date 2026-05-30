---
Status: Landed
Date: 2026-05-30
Scope: choose the next durable owner after the remaining RuntimeDataBox consumer surfaces were inventoried file by file.
Blocker: RUNTIME-DATA-DISPATCH-CONSUMER-CALLSITE-ATTRIBUTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md
  - docs/development/current/main/investigations/phase296x-400-rdo001-collection-method-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo002-boxcall-collection-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo003-field-sink-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo004-legacy-bridge-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo005-tests-and-route-assertions-inventory.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/instructions/boxcall_runtime_data.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/mir_call_legacy.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - src/llvm_py/tests/test_collection_method_call.py
---

# 296x-401 Runtime Data Dispatch Consumer Callsite Attribution Refresh

## Purpose

Row400 split the remaining RuntimeDataBox consumer surface into file-by-file
inventories. Refresh the owner now and choose exactly one next durable surface
before any new policy change. Keep the row docs-only.

## Contract

```text
output_contract=runtime-data-dispatch-consumer-callsite-attribution-refresh-v0
input_contract=runtime-data-dispatch-route-policy-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
consumer_surface_collection_method=collection_method_call.py
consumer_surface_boxcall=boxcall_runtime_data.py
consumer_surface_field_sink=field_access.py
consumer_surface_legacy_bridge=mir_call_legacy.py
consumer_surface_tests=test_runtime_data_dispatch_policy.py|test_collection_method_call.py
runtime_data_dispatch_thin_consumer=1
runtime_data_route_policy_source_stable=1
runtime_data_consumer_surface_attributed_file_by_file=1
runtime_data_direct_array_reinterpretation_allowed=0
selected_next=collection_method_call_surface_owner_selection
selected_reason=collection_method_call_py_owns_the_shared_collection_route_order_and_the_direct_array_lane_so_it_is_the_highest_leverage_remaining_consumer_surface_after_the_row400_inventory_split
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

### CCA-001: Collection Method Surface Compare

Input:
- `docs/development/current/main/investigations/phase296x-400-rdo001-collection-method-surface-inventory.md`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short note confirming whether this remains the primary consumer surface
- short list of likely miss points

Acceptance:
- the collection-method surface is the selected next owner or explicitly rejected
- no implementation is proposed

### CCA-002: BoxCall Surface Compare

Input:
- `docs/development/current/main/investigations/phase296x-400-rdo002-boxcall-collection-surface-inventory.md`
- `src/llvm_py/instructions/boxcall_runtime_data.py`

Output:
- short note confirming whether the boxcall surface stays secondary
- short list of likely miss points

Acceptance:
- the boxcall surface is pinned as secondary or explicitly rejected
- no implementation is proposed

### CCA-003: Field Sink And Legacy Compare

Input:
- `docs/development/current/main/investigations/phase296x-400-rdo003-field-sink-surface-inventory.md`
- `docs/development/current/main/investigations/phase296x-400-rdo004-legacy-bridge-surface-inventory.md`
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call_legacy.py`

Output:
- short note confirming whether the sink and bridge remain compatibility surfaces
- short list of likely miss points

Acceptance:
- the sink/bridge remain secondary or explicitly rejected
- no implementation is proposed

### CCA-004: Tests And Route Assertions Review

Input:
- `docs/development/current/main/investigations/phase296x-400-rdo005-tests-and-route-assertions-inventory.md`
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short note on the remaining assertions that still anchor the route split

Acceptance:
- the tests remain pinning the chosen owner split
- no implementation is proposed

### CCA-005: Next Owner Selection

Input:
- CCA-001 through CCA-004 outputs

Output:
- choose exactly one next owner

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### CCA-006: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row401 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_consumer_callsite_attribution_refresh_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The remaining RuntimeDataBox consumer surface is now attributed file by file.
The shared collection route order is the highest-leverage remaining owner:

```text
selected_next=collection_method_call_surface_owner_selection
selected_reason=collection_method_call_py_owns_the_shared_collection_route_order_and_the_direct_array_lane_so_it_is_the_highest_leverage_remaining_consumer_surface_after_the_row400_inventory_split
```

## Acceptance

- row400 remains landed and its source module remains intact
- `RuntimeDataBox` remains distinct from DirectArray
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
bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_consumer_callsite_attribution_refresh_guard.sh
```
