---
Status: Landed
Date: 2026-05-30
Scope: inventory the shared collection route order after row402 pinned the collection-method surface owner and before any implementation opens.
Blocker: COLLECTION-METHOD-ROUTE-ORDER-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-402-COLLECTION-METHOD-SURFACE-OWNER-SELECTION.md
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

# 296x-403 Collection Method Route Order Inventory

## Purpose

Row402 pinned `collection_method_call.py` as the highest-leverage remaining
collection-method surface. Inventory the shared route order itself so the next
diagnostic can narrow to exactly one durable owner before any implementation.
Keep the row docs-only.

## Contract

```text
output_contract=collection-method-route-order-inventory-v0
input_contract=collection-method-surface-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
shared_route_order_surface=collection_method_call.py
collection_method_surface_primary=collection_method_call.py
runtime_data_preemption_surface=lower_runtime_data_method_call
direct_array_lane_surface=_lower_direct_array_nativedirect_call
array_fallback_surface=_lower_array_collection_method_call
map_fallback_surface=_lower_map_collection_method_call
compatibility_surface_boxcall=boxcall_runtime_data.py
compatibility_surface_field_sink=field_access.py
compatibility_surface_legacy=mir_call_legacy.py
tests_surface_anchor=test_runtime_data_dispatch_policy.py|test_collection_method_call.py
runtime_data_dispatch_thin_consumer=1
shared_collection_route_order_is_highest_leverage=1
direct_array_lane_exact_only=1
public_arraybox_runtime_surface_secondary=1
compatibility_surfaces_secondary=1
selected_next=collection_method_call_direct_array_lane_owner_selection
selected_reason=the_shared_collection_route_order_is_now_pinned_enough_to_narrow_into_the_direct_array_lane_exact_only_owner_selection_before_any_implementation
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

### ROR-001: RuntimeData Preemption Compare

Input:
- `docs/development/current/main/phases/phase-296x/296x-402-COLLECTION-METHOD-SURFACE-OWNER-SELECTION.md`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the runtime-data preemption branch and likely miss points

Acceptance:
- the runtime-data preemption stays thin and exact-only
- no implementation is proposed

### ROR-002: Direct Array Lane Compare

Input:
- `src/llvm_py/instructions/mir_call/collection_method_call.py`
- `src/llvm_py/utils/resolver_helpers.py`

Output:
- short table of the direct-array lane and likely miss points

Acceptance:
- the direct-array lane remains exact-only
- no implementation is proposed

### ROR-003: Array And Map Fallback Compare

Input:
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the ArrayBox and MapBox fallback branches
- short list of likely miss points

Acceptance:
- the fallback branches stay fallback-only
- no implementation is proposed

### ROR-004: Compatibility Surfaces Compare

Input:
- `src/llvm_py/instructions/boxcall_runtime_data.py`
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call_legacy.py`

Output:
- short table of the compatibility surfaces
- short list of likely miss points

Acceptance:
- the compatibility surfaces stay secondary
- no implementation is proposed

### ROR-005: Tests And Route Assertions Review

Input:
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short note on the remaining assertions that still anchor the split

Acceptance:
- the tests remain pinning the chosen route order split
- no implementation is proposed

### ROR-006: Next Owner Selection

Input:
- ROR-001 through ROR-005 outputs

Output:
- choose exactly one next owner

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### ROR-007: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row403 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_route_order_inventory_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Investigation Notes

The row403 route-order inventory is being captured as separate, mini-worker-sized
evidence notes so that each file can be reviewed independently:

- [ROR-001 runtime-data preemption compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-403-ror001-runtime-data-preemption-compare.md>)
- [ROR-002 direct-array lane compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-403-ror002-direct-array-lane-compare.md>)
- [ROR-003 array and map fallback compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-403-ror003-array-and-map-fallback-compare.md>)
- [ROR-004 compatibility surfaces compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-403-ror004-compatibility-surfaces-compare.md>)
- [ROR-005 tests and route assertions review](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-403-ror005-tests-and-route-assertions-review.md>)
- [ROR-006 next owner selection](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-403-ror006-next-owner-selection.md>)

## Decision

The shared collection route order is now pinned enough to narrow into the
direct-array lane exact-only owner selection before any implementation:

```text
selected_next=collection_method_call_direct_array_lane_owner_selection
selected_reason=the_shared_collection_route_order_is_now_pinned_enough_to_narrow_into_the_direct_array_lane_exact_only_owner_selection_before_any_implementation
```

## Acceptance

- row402 is landed and its surface remains intact
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
bash tools/checks/k2_wide_phase296x_collection_method_route_order_inventory_guard.sh
```
