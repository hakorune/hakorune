---
Status: Landed
Date: 2026-05-30
Scope: refresh the owner after the keeper measurement and classify the remaining RuntimeDataBox consumer callsite surface before any new policy change.
Blocker: RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-399-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-KEEPER-MEASUREMENT.md
  - docs/development/current/main/investigations/phase296x-400-rdo001-collection-method-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo002-boxcall-collection-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo003-field-sink-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo004-legacy-bridge-surface-inventory.md
  - docs/development/current/main/investigations/phase296x-400-rdo005-tests-and-route-assertions-inventory.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/mir_call/runtime_data_route_policy.py
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/instructions/boxcall_runtime_data.py
  - src/llvm_py/instructions/mir_call/method_call.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/mir_call_legacy.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - src/llvm_py/tests/test_collection_method_call.py
---

# 296x-400 Runtime Data Dispatch Route Policy Owner Refresh

## Purpose

Row399 confirmed that the extracted route-policy source module is stable and
that `runtime_data_dispatch.py` stays thin. The remaining question is which
RuntimeDataBox consumer surface should own the next diagnostic row. Refresh the
owner before any new policy change.

## Contract

```text
output_contract=runtime-data-dispatch-route-policy-owner-refresh-v0
input_contract=runtime-data-dispatch-route-policy-keeper-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
runtime_data_dispatch_thin_consumer=1
runtime_data_route_policy_source_stable=1
runtime_data_collection_method_surface_present=1
runtime_data_boxcall_collection_surface_present=1
runtime_data_field_sink_surface_present=1
runtime_data_legacy_bridge_surface_present=1
runtime_data_tests_surface_present=1
direct_array_handle_reinterpretation_allowed=0
selected_next=runtime_data_dispatch_consumer_callsite_attribution_refresh
selected_reason=the_route_policy_module_is_stable_and_the_remaining_consumer_surface_needs_callsite_attribution_before_any_new_policy_change
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

### RDO-001: Collection-Method Surface Inventory

Input:
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

Output:
- short table of the `RuntimeDataBox` / `ArrayBox` / `MapBox` collection-route
  surface
- short list of likely miss points

Acceptance:
- the collection-method surface is pinned
- no direct-array fast path claim is made

### RDO-002: BoxCall Collection Surface Inventory

Input:
- `src/llvm_py/instructions/boxcall_runtime_data.py`

Output:
- short table of the `RuntimeDataBox`-adjacent collection boxcall surface
- short list of likely miss points

Acceptance:
- the boxcall collection surface is pinned
- no direct-array fast path claim is made

### RDO-003: Field Sink Surface Inventory

Input:
- `src/llvm_py/instructions/mir_call/method_call.py`
- `src/llvm_py/instructions/field_access.py`

Output:
- short table of the `RuntimeDataBox.getField/setField` sink surface
- short list of likely miss points

Acceptance:
- the field sink surface is pinned
- no new Array or DirectArray claim is made

### RDO-004: Legacy Bridge Surface Inventory

Input:
- `src/llvm_py/instructions/mir_call_legacy.py`

Output:
- short table of the legacy RuntimeDataBox bridge surface
- short list of likely miss points

Acceptance:
- the legacy bridge surface is pinned
- no direct-array fast path claim is made

### RDO-005: Tests And Route Assertions Inventory

Input:
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short table of the policy and route assertions that still matter
- short list of likely miss points

Acceptance:
- the test assertions are pinned
- no implementation is proposed

### RDO-006: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row400 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_owner_refresh_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The route-policy source module is stable, but the remaining consumer surface
still needs callsite attribution before any new policy change:

```text
selected_next=runtime_data_dispatch_consumer_callsite_attribution_refresh
selected_reason=policy_source_module_is_stable_but_the_remaining_consumer_surface_needs_callsite_attribution_before_any_new_policy_change
```

## Acceptance

- row399 is landed and its source module remains intact
- `RuntimeDataBox` remains distinct from DirectArray
- the dispatch layer stays thin
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
bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_owner_refresh_guard.sh
```
