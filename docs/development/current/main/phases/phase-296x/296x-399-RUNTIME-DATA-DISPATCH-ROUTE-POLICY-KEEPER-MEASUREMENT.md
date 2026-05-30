---
Status: Landed
Date: 2026-05-30
Scope: measure that the extracted RuntimeDataBox route-policy source module remains stable and that runtime_data_dispatch.py stays a thin consumer.
Blocker: RUNTIME-DATA-DISPATCH-ROUTE-POLICY-KEEPER-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-398-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-IMPLEMENTATION.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/mir_call/runtime_data_route_policy.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
---

# 296x-399 Runtime Data Dispatch Route Policy Keeper Measurement

## Purpose

Row398 extracted the env-backed route-policy source into a dedicated module.
This row measures that the split is stable: the policy module remains the
source of truth, the dispatch layer stays thin, and the RuntimeDataBox route
policy continues to fail fast on unsupported env values without reintroducing a
DirectArray fast path.

## Measurement Contract

```text
output_contract=runtime-data-dispatch-route-policy-keeper-measurement-v0
input_contract=runtime-data-dispatch-route-policy-implementation-v0
workload_id=representative-object-lifecycle-small-block-v0
policy_source_module=src/llvm_py/instructions/mir_call/runtime_data_route_policy.py
runtime_data_dispatch_thin_consumer=1
runtime_data_array_route_policy_present=1
runtime_data_array_route_policy_env_present=1
runtime_data_select_call_spec_present=1
prefer_runtime_data_array_route_present=1
prefer_runtime_data_array_i64_key_route_present=1
prefer_runtime_data_array_i64_key_i64_value_route_present=1
route_policy_surface_is_split_between_runtime_data_dispatch_and_auto_specialize=1
route_policy_invalid_env_fail_fast=1
route_policy_default_uses_array_hint_predicates=1
runtime_databox_field_dispatch_is_not_direct_array=1
direct_array_handle_reinterpretation_allowed=0
keeper_measurement_stability_smoke=ok
keeper_measurement_thin_consumer_smoke=ok
keeper_measurement_default_policy_smoke=ok
keeper_measurement_runtime_data_only_smoke=ok
selected_next=runtime_data_dispatch_route_policy_owner_refresh
selected_reason=the_policy_source_module_is_stable_and_the_next_row_should_refresh_the_owner_before_any_new_policy_change
measurement_open=0
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

### RKM-001: Stability Smoke

Input:
- row398 implementation
- current state
- route-policy keeper measurement guard

Output:
- short stability note that the extracted route-policy source module remains
  the stable owner

Acceptance:
- the stability smoke stays green
- no direct-array fast path claim is made

### RKM-002: Thin-Consumer Boundary Smoke

Input:
- RKM-001 output

Output:
- short note that explains whether the boundary is the policy module, the
  dispatch consumer, or both together

Acceptance:
- the next owner is selected exactly once
- no implementation is proposed

### RKM-003: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row399 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_keeper_measurement_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Acceptance

- row398 is landed and its source-module extraction stays intact
- the route policy helpers remain visible in the dedicated module
- the dispatch layer remains a thin consumer
- no DirectArray reinterpretation is opened
- no implementation is opened in this row

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
bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_keeper_measurement_guard.sh
```
