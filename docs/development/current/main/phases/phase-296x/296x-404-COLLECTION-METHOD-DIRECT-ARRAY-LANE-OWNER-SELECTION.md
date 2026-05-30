---
Status: Landed
Date: 2026-05-30
Scope: select the direct-array lane owner after row403 pinned the shared collection route order and exact-only direct-array branch.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-403-COLLECTION-METHOD-ROUTE-ORDER-INVENTORY.md
  - docs/development/current/main/investigations/phase296x-403-ror001-runtime-data-preemption-compare.md
  - docs/development/current/main/investigations/phase296x-403-ror002-direct-array-lane-compare.md
  - docs/development/current/main/investigations/phase296x-403-ror003-array-and-map-fallback-compare.md
  - docs/development/current/main/investigations/phase296x-403-ror004-compatibility-surfaces-compare.md
  - docs/development/current/main/investigations/phase296x-403-ror005-tests-and-route-assertions-review.md
  - docs/development/current/main/investigations/phase296x-403-ror006-next-owner-selection.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/utils/resolver_helpers.py
  - src/llvm_py/instructions/boxcall_runtime_data.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/mir_call_legacy.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - src/llvm_py/tests/test_collection_method_call.py
---

# 296x-404 Collection Method Direct Array Lane Owner Selection

## Purpose

Row403 pinned the shared collection route order and showed the direct-array lane
is exact-only. Select the next durable owner for that lane before any
implementation opens. Keep the row docs-only.

## Contract

```text
output_contract=collection-method-direct-array-lane-owner-selection-v0
input_contract=collection-method-route-order-inventory-v0
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
selected_next=collection_method_call_direct_array_lane_guard_surface
selected_reason=the_exact_only_direct_array_lane_is_the_remaining_highest_leverage_owner_and_should_freeze_a_guard_surface_before_any_implementation
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

### DALO-001: Direct Array Lane Compare

Input:
- `docs/development/current/main/investigations/phase296x-403-ror002-direct-array-lane-compare.md`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the exact-only direct-array lane
- short list of likely miss points

Acceptance:
- the direct-array lane remains exact-only
- no implementation is proposed

### DALO-002: Array Fallback Compare

Input:
- `docs/development/current/main/investigations/phase296x-403-ror003-array-and-map-fallback-compare.md`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the ArrayBox and MapBox fallback branches
- short list of likely miss points

Acceptance:
- the fallback branches stay fallback-only
- no implementation is proposed

### DALO-003: Compatibility Surfaces Compare

Input:
- `docs/development/current/main/investigations/phase296x-403-ror004-compatibility-surfaces-compare.md`
- `src/llvm_py/instructions/boxcall_runtime_data.py`
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call_legacy.py`

Output:
- short table of the compatibility surfaces
- short list of likely miss points

Acceptance:
- the compatibility surfaces stay secondary
- no implementation is proposed

### DALO-004: Tests And Route Assertions Review

Input:
- `docs/development/current/main/investigations/phase296x-403-ror005-tests-and-route-assertions-review.md`
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short note on the remaining assertions that still anchor the split

Acceptance:
- the tests remain pinning the chosen owner split
- no implementation is proposed

### DALO-005: Next Owner Selection

Input:
- DALO-001 through DALO-004 outputs

Output:
- choose exactly one next owner

Acceptance:
- one owner selected
- rejected owners have short reasons
- implementation remains closed

### DALO-006: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row404 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_owner_selection_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Investigation Notes

The row404 direct-array lane owner selection is being captured as separate,
mini-worker-sized evidence notes so that each file can be reviewed
independently:

- [DALO-001 direct array lane compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo001-direct-array-lane-compare.md>)
- [DALO-002 array fallback compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo002-array-fallback-compare.md>)
- [DALO-003 compatibility surfaces compare](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo003-compatibility-surfaces-compare.md>)
- [DALO-004 tests and route assertions review](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo004-tests-and-route-assertions-review.md>)
- [DALO-005 next owner selection](</home/tomoaki/git/hakorune-selfhost/docs/development/current/main/investigations/phase296x-404-dalo005-next-owner-selection.md>)

## Decision

The exact-only direct-array lane is the remaining highest-leverage owner after
the shared route order inventory:

```text
selected_next=collection_method_call_direct_array_lane_guard_surface
selected_reason=the_exact_only_direct_array_lane_is_the_remaining_highest_leverage_owner_and_should_freeze_a_guard_surface_before_any_implementation
```

## Acceptance

- row403 is landed and its route-order inventory remains intact
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
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_owner_selection_guard.sh
```
