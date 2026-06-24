---
Status: Landed
Date: 2026-05-30
Scope: inventory the RuntimeDataBox field route policy after row395 pinned the route boundary and before any contract row opens.
Blocker: RUNTIME-DATA-DISPATCH-ROUTE-POLICY-CONTRACT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-395-RUNTIME-DATA-DISPATCH-FIELD-ROUTE-INVENTORY.md
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/mir_call/auto_specialize.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
---

# 296x-396 Runtime Data Dispatch Route Policy Inventory

## Purpose

Row395 pinned the field-route boundary. The remaining question is whether the
route-policy surface itself is now a stable contract or whether it still needs
its own contract row. Inventory the policy sources file by file, keep the
field sink / map ABI / route policy split narrow, and choose exactly one next
durable owner.

## Contract

```text
output_contract=runtime-data-dispatch-route-policy-inventory-v0
input_contract=runtime-data-dispatch-field-route-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
runtime_data_array_route_policy_present=1
runtime_data_array_route_policy_env_present=1
runtime_data_select_call_spec_present=1
prefer_runtime_data_array_route_present=1
prefer_runtime_data_array_i64_key_route_present=1
prefer_runtime_data_array_i64_key_i64_value_route_present=1
route_policy_surface_is_split_between_runtime_data_dispatch_and_auto_specialize=1
selected_next=runtime_data_dispatch_route_policy_contract
selected_reason=env_policy_and_array_hint_predicates_are_visible_but_the_contract_row_is_still_needed_to_pin_the_split
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

### RPI-001: Runtime Data Dispatch Policy Source Inventory

Input:
- `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`

Output:
- short table of the env-backed route policy helpers
- short list of likely miss points

Acceptance:
- the route policy source is pinned
- no direct-array fast path claim is made

### RPI-002: AutoSpecialize Route Predicate Inventory

Input:
- `src/llvm_py/instructions/mir_call/auto_specialize.py`

Output:
- short table of the array-specialization predicate helpers
- short list of likely miss points

Acceptance:
- the route predicate surface is pinned
- no direct-array fast path claim is made

### RPI-003: Policy Boundary Note

Input:
- RPI-001 through RPI-002 outputs

Output:
- short note that explains whether the boundary is the env policy, the array
  predicates, or both together

Acceptance:
- the next owner is selected exactly once
- no implementation is proposed

### RPI-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row396 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_inventory_guard.sh` passes
- `bash tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

## Decision

The env-backed route policy and the array hint predicates are both visible, so
the remaining ambiguity is the contract surface itself:

```text
selected_next=runtime_data_dispatch_route_policy_contract
selected_reason=env_policy_and_array_hint_predicates_are_visible_but_the_contract_row_is_still_needed_to_pin_the_split
```

## Acceptance

- row395 real route inventory is the input
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
bash tools/checks/k2_wide_phase296x_runtime_data_dispatch_route_policy_inventory_guard.sh
```
