# 296x-1580 MIRBUILDER-TYPED-HARNESS-REWRITE-EMISSION-CONTRACT-BINDING-CONTEXT-VARIABLE-CONTEXT-SIMPLE-MAP-001

Status: landed
Date: 2026-06-22

## Purpose

Define the typed harness rewrite emission contract for the first selected
slice: BindingContext and VariableContext simple-map.

## Scope

```text
BoxCount: one consultation emission contract
owner: MirBuilder typed harness rewrite emission contract
input: typed harness rewrite contract for the shared ordered-map harness family
output: durable emission contract for the shared ordered-map harness family
```

## Observed State

```text
selected_slice=BindingContext_and_VariableContext_simple_map
typed_rewrite_contract=present
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

The emission contract must treat the shared ordered-map harness family as a
typed output surface, not a place to make new slice selection decisions.

The contract must keep these boundaries explicit:

- emitter input comes from the typed harness rewrite contract, not from raw
  route decisions
- emitter output is the Hako emission plan for the shared ordered-map family
- the emitter must not widen route selection
- the emitter must not open nightly rustc adapter paths
- the emitter must not change the typed converter core
- the emitter must not reintroduce raw `main_lines` ownership into the
  selection path

The contract must keep these emission rules explicit:

- preserve ordered-map operations already accepted by the lightweight
  converter path
- emit the BindingContext and VariableContext simple-map family through the
  shared emitter path only
- keep `BindingContext` and `VariableContext simple-map` as the only members
  of this emission contract
- do not include BoxCompilationContext in this contract
- do not include snapshot/restore or carrier snapshot contracts in this
  contract
- do not include VariableContext immutable borrow ReturnSource in this
  contract
- do not add family-specific emitter branching beyond the shared ordered-map
  family path

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the typed harness rewrite emission contract is explicit and bounded
the first rewrite slice remains BindingContext and VariableContext simple-map
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
the shared emitter stays the only emission path for this slice
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_emission_implementation=1
do_not_expand_scope_beyond_shared_ordered_map_family=1
do_not_add_family_specific_emitter_branching=1
```
