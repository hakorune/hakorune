---
Status: Landed
Date: 2026-05-30
Scope: extract the env-backed RuntimeDataBox route policy into a dedicated policy source module and keep runtime_data_dispatch.py as a thin consumer.
Blocker: RUNTIME-DATA-DISPATCH-ROUTE-POLICY-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-397-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-CONTRACT.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/mir_call/runtime_data_route_policy.py
  - src/llvm_py/instructions/mir_call/auto_specialize.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
---

# 296x-398 Runtime Data Dispatch Route Policy Implementation

## Purpose

Row397 froze the route-policy contract. This row moves the env-backed policy
source into a dedicated module and keeps the dispatch layer thin so the shared
policy surface has one owner and one import boundary.

## Contract

```text
output_contract=runtime-data-dispatch-route-policy-implementation-v0
input_contract=runtime-data-dispatch-route-policy-contract-v0
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
selected_next=runtime_data_dispatch_route_policy_keeper_measurement
selected_reason=the_policy_surface_now_has_a_dedicated_source_module_and_the_next_row_should_only_verify_the_extraction_is_stable
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

### RPI-001: Policy Source Module Extraction

Input:
- `src/llvm_py/instructions/mir_call/runtime_data_route_policy.py`
- `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`

Output:
- short table of the extracted policy source helpers
- short list of likely miss points

Acceptance:
- the env-backed route policy lives in the dedicated module
- no direct-array fast path claim is made

### RPI-002: Thin-Consumer Boundary Note

Input:
- RPI-001 output

Output:
- short note that explains whether the boundary is the policy module, the
  dispatch consumer, or both together

Acceptance:
- the next owner is selected exactly once
- no implementation is proposed

### RPI-003: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row398 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_implementation_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The route-policy source module is the stable owner, while the dispatch module
becomes the thin consumer:

```text
selected_next=runtime_data_dispatch_route_policy_keeper_measurement
selected_reason=the_policy_surface_now_has_a_dedicated_source_module_and_the_next_row_should_only_verify_the_extraction_is_stable
```

## Acceptance

- row397 real route-policy contract is the input
- `RuntimeDataBox` field routing stays distinct from DirectArray
- the route policy helpers remain visible
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
bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_implementation_guard.sh
```
