---
Status: Landed
Date: 2026-05-30
Scope: pin the route-policy contract that binds the env policy and array hint predicates before implementation.
Blocker: RUNTIME-DATA-DISPATCH-ROUTE-POLICY-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-396-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-INVENTORY.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/mir_call/auto_specialize.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
---

# 296x-397 Runtime Data Dispatch Route Policy Contract

## Purpose

Row396 showed the env-backed route policy and the array hint predicates. This
row freezes the shared contract surface before any implementation claims the
route split.

## Contract

```text
output_contract=runtime-data-dispatch-route-policy-contract-v0
input_contract=runtime-data-dispatch-route-policy-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
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
selected_next=runtime_data_dispatch_route_policy_implementation
selected_reason=the_route_split_is_visible_but_the_contract_surface_needs_to_be_frozen_before_implementation
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

### RPC-001: Route Policy Contract Surface Inventory

Input:
- `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`
- `src/llvm_py/instructions/mir_call/auto_specialize.py`
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`

Output:
- short table of the contract-visible route policy pieces
- short list of likely miss points

Acceptance:
- the route-policy contract surface is pinned
- no direct-array fast path claim is made

### RPC-002: Fail-Fast Boundary Note

Input:
- RPC-001 output

Output:
- short note that explains whether the contract boundary is the env policy,
  the array hint predicates, or both together

Acceptance:
- the next owner is selected exactly once
- no implementation is proposed

### RPC-003: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row397 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_contract_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The env-backed route policy and the array hint predicates are both visible, so
the remaining ambiguity is the contract surface itself:

```text
selected_next=runtime_data_dispatch_route_policy_implementation
selected_reason=the_route_split_is_visible_but_the_contract_surface_needs_to_be_frozen_before_implementation
```

## Acceptance

- row396 real route-policy inventory is the input
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
bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_contract_guard.sh
```
