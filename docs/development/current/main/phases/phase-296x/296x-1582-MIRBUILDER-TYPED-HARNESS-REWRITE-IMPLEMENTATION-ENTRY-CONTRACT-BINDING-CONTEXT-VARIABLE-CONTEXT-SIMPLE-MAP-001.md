# 296x-1582 MIRBUILDER-TYPED-HARNESS-REWRITE-IMPLEMENTATION-ENTRY-CONTRACT-BINDING-CONTEXT-VARIABLE-CONTEXT-SIMPLE-MAP-001

Status: landed
Date: 2026-06-22

## Purpose

Define the implementation entry contract for the first typed harness rewrite
slice: BindingContext and VariableContext simple-map.

## Scope

```text
BoxCount: one consultation entry contract
owner: MirBuilder typed harness rewrite implementation entry
input: implementation boundary for the shared ordered-map harness family
output: durable entry contract for the shared ordered-map harness family
```

## Observed State

```text
selected_slice=BindingContext_and_VariableContext_simple_map
typed_rewrite_contract=present
typed_emission_contract=present
typed_boundary_contract=present
binding_context_main_lines=1
variable_context_simple_map_main_lines=1
typed_converter_core=present
shared_renderer_layer=present
shared_generator_layer=present
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
```

## Entry Contract

If implementation starts, the first touch set must be the shared ordered-map
slice only:

```text
tools/rust_lifecycle/mirbuilder_family_artifacts.py
tools/rust_lifecycle/family_artifact_spec.py
tools/rust_lifecycle/family_artifact_builders.py
tools/rust_lifecycle/shared_mirbuilder_emitter.py
```

The entry contract must keep these rules explicit:

- the first implementation step is limited to the shared ordered-map family
- the first implementation step keeps BindingContext and VariableContext
  simple-map as the only members
- the first implementation step does not start BoxCompilationContext
- the first implementation step does not start VariableContext snapshot/restore
- the first implementation step does not start carrier snapshot contracts
- the first implementation step does not start VariableContext immutable
  borrow ReturnSource
- the first implementation step does not widen route selection
- the first implementation step does not open nightly rustc adapter paths
- the first implementation step does not change the typed converter core
- the first implementation step does not add family-specific emitter branching

The entry contract must also keep the rollout order explicit:

- first, keep the selected slice within the shared emitter path
- second, keep any emitted result constrained to the current ordered-map
  harness family
- third, defer all remaining raw-string slices until a separate consultation
  row selects them

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the first implementation touch set is explicit
the first implementation touch set is bounded to the shared ordered-map family
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this entry contract
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_expand_scope_beyond_shared_ordered_map_family=1
do_not_add_family_specific_emitter_branching=1
