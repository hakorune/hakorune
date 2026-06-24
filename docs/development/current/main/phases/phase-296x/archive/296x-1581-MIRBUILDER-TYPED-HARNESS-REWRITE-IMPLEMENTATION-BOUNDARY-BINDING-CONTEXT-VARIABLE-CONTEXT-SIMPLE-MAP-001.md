# 296x-1581 MIRBUILDER-TYPED-HARNESS-REWRITE-IMPLEMENTATION-BOUNDARY-BINDING-CONTEXT-VARIABLE-CONTEXT-SIMPLE-MAP-001

Status: landed
Date: 2026-06-22

## Purpose

Define the implementation boundary for the first typed harness rewrite slice:
BindingContext and VariableContext simple-map.

## Scope

```text
BoxCount: one consultation boundary
owner: MirBuilder typed harness rewrite implementation boundary
input: typed harness rewrite emission contract for the shared ordered-map harness family
output: durable implementation boundary for the shared ordered-map harness family
```

## Observed State

```text
selected_slice=BindingContext_and_VariableContext_simple_map
typed_rewrite_contract=present
typed_emission_contract=present
binding_context_main_lines=1
variable_context_simple_map_main_lines=1
typed_converter_core=present
shared_renderer_layer=present
shared_generator_layer=present
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
```

## Boundary

The implementation boundary must be narrow and explicit:

- the rewrite slice stays limited to the shared ordered-map family
- the shared ordered-map family currently means only BindingContext and
  VariableContext simple-map
- implementation work, when it starts, must flow through the shared emitter
  path
- implementation work, when it starts, must not widen route selection
- implementation work, when it starts, must not open nightly rustc adapter
  paths
- implementation work, when it starts, must not change the typed converter
  core

The implementation boundary must keep these file responsibilities explicit:

- `tools/rust_lifecycle/family_artifact_spec.py` remains the shared data
  carrier for spec-level metadata and is not the place to widen family scope
- `tools/rust_lifecycle/family_artifact_builders.py` remains the shared
  renderer and is not the place to add family-specific routing
- `tools/rust_lifecycle/shared_mirbuilder_emitter.py` remains the shared
  emission path for the slice
- `tools/rust_lifecycle/mirbuilder_family_artifacts.py` remains the current
  family-spec host for the selected slice until a later rewrite task replaces
  the raw harness text
- `tools/rust_lifecycle/mirbuilder_carrier_snapshot_artifacts.py` stays
  outside this boundary

The implementation boundary must keep these exclusions explicit:

- do not include BoxCompilationContext
- do not include VariableContext snapshot/restore
- do not include carrier snapshot contracts
- do not include VariableContext immutable borrow ReturnSource
- do not include route selection changes
- do not include nightly rustc adapter opening
- do not include runtime fallback
- do not add family-specific emitter branching

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the implementation boundary is explicit and narrow
the first rewrite slice remains BindingContext and VariableContext simple-map
the shared emitter remains the emission path
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_expand_scope_beyond_shared_ordered_map_family=1
do_not_add_family_specific_emitter_branching=1
```
