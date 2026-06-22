# 296x-1579 MIRBUILDER-TYPED-HARNESS-REWRITE-CONTRACT-BINDING-CONTEXT-VARIABLE-CONTEXT-SIMPLE-MAP-001

Status: landed
Date: 2026-06-22

## Purpose

Define the typed harness rewrite contract for the first selected slice:
BindingContext and VariableContext simple-map.

## Scope

```text
BoxCount: one consultation contract
owner: MirBuilder typed harness rewrite contract
input: first typed harness rewrite slice selection
output: durable rewrite contract for the shared ordered-map harness family
```

## Observed State

```text
selected_slice=BindingContext_and_VariableContext_simple_map
binding_context_main_lines=1
variable_context_simple_map_main_lines=1
typed_converter_core=present
shared_renderer_layer=present
shared_generator_layer=present
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
```

## Contract

The first typed harness rewrite contract must treat the shared ordered-map
family as a single rewrite surface with two current members:

```text
BindingContext
VariableContext simple-map
```

The contract must keep these boundaries explicit:

- harness input comes from the current family artifact specs and their current
  acceptance harness intent
- harness output is a typed rewrite contract, not implementation code
- the typed rewrite contract must not widen route selection
- the typed rewrite contract must not open nightly rustc adapter paths
- the typed rewrite contract must not change the typed converter core

The contract must keep these rewrite rules explicit:

- preserve ordered-map operations already accepted by the lightweight
  converter path
- keep the shared ordered-map harness family as the first rewrite unit
- do not include BoxCompilationContext in this contract
- do not include snapshot/restore or carrier snapshot contracts in this
  contract
- do not include VariableContext immutable borrow ReturnSource in this
  contract

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the typed harness rewrite contract is explicit and bounded
the first rewrite slice remains BindingContext and VariableContext simple-map
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_harness_implementation=1
do_not_expand_scope_beyond_shared_ordered_map_family=1
```
