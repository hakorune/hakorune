---
Status: Landed
Date: 2026-05-30
Scope: choose the next durable collection-method owner after the consumer callsite attribution row split the remaining RuntimeDataBox surfaces file by file.
Blocker: COLLECTION-METHOD-SURFACE-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-401-RUNTIME-DATA-DISPATCH-CONSUMER-CALLSITE-ATTRIBUTION-REFRESH.md
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

# 296x-402 Collection Method Surface Owner Selection

## Purpose

Row401 pinned the collection-method surface as the highest-leverage remaining
RuntimeDataBox consumer. Choose exactly one next durable diagnostic owner
before any implementation opens. Keep the row docs-only.

## Contract

```text
output_contract=collection-method-surface-owner-selection-v0
input_contract=runtime-data-dispatch-consumer-callsite-attribution-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
collection_method_surface_primary=collection_method_call.py
boxcall_surface_secondary=boxcall_runtime_data.py
field_sink_surface_secondary=field_access.py
legacy_bridge_surface_secondary=mir_call_legacy.py
tests_surface_anchor=test_runtime_data_dispatch_policy.py|test_collection_method_call.py
shared_collection_route_order_is_highest_leverage=1
direct_array_lane_exact_only=1
public_arraybox_runtime_surface_secondary=1
compatibility_surfaces_secondary=1
selected_next=collection_method_call_route_order_inventory
selected_reason=collection_method_call_py_owns_the_shared_collection_route_order_and_the_direct_array_lane_so_the_next_durable_diagnostic_should_inventory_the_route_order_before_any_implementation
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

### CMO-001: Shared Route Order Compare

Input:
- `docs/development/current/main/phases/phase-296x/296x-401-RUNTIME-DATA-DISPATCH-CONSUMER-CALLSITE-ATTRIBUTION-REFRESH.md`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the shared collection route order
- short list of likely miss points

Acceptance:
- the shared route order remains the selected next owner or is explicitly rejected
- no implementation is proposed

### CMO-002: Direct Array Lane Compare

Input:
- `src/llvm_py/instructions/mir_call/collection_method_call.py`
- `src/llvm_py/utils/resolver_helpers.py`

Output:
- short table of the exact-lane direct-array branch
- short list of likely miss points

Acceptance:
- the direct-array lane remains exact-only
- no implementation is proposed

### CMO-003: Secondary Surfaces Compare

Input:
- `src/llvm_py/instructions/boxcall_runtime_data.py`
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call_legacy.py`

Output:
- short table of the boxcall, field sink, and legacy bridge surfaces
- short list of likely miss points

Acceptance:
- the secondary / compatibility surfaces stay secondary
- no implementation is proposed

### CMO-004: Tests And Route Assertions Review

Input:
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short note on the remaining assertions that still anchor the split

Acceptance:
- the tests remain pinning the chosen owner split
- no implementation is proposed

### CMO-005: Next Owner Selection

Input:
- CMO-001 through CMO-004 outputs

Output:
- choose exactly one next owner

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### CMO-006: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row402 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_surface_owner_selection_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The shared collection route order is the highest-leverage remaining owner:

```text
selected_next=collection_method_call_route_order_inventory
selected_reason=collection_method_call_py_owns_the_shared_collection_route_order_and_the_direct_array_lane_so_the_next_durable_diagnostic_should_inventory_the_route_order_before_any_implementation
```

## Acceptance

- row401 is landed and its consumer attribution remains intact
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
bash tools/checks/k2_wide_phase296x_collection_method_surface_owner_selection_guard.sh
```
