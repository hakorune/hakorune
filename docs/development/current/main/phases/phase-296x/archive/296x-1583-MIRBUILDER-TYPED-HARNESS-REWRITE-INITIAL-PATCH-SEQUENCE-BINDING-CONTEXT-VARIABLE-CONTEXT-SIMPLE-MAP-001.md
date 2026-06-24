# 296x-1583 MIRBUILDER-TYPED-HARNESS-REWRITE-INITIAL-PATCH-SEQUENCE-BINDING-CONTEXT-VARIABLE-CONTEXT-SIMPLE-MAP-001

Status: landed
Date: 2026-06-22

## Purpose

Define the initial patch sequence for the first typed harness rewrite slice:
BindingContext and VariableContext simple-map.

## Scope

```text
BoxCount: one consultation sequence
owner: MirBuilder typed harness rewrite initial patch sequence
input: implementation entry contract for the shared ordered-map harness family
output: durable first-patch sequence for the shared ordered-map harness family
```

## Observed State

```text
selected_slice=BindingContext_and_VariableContext_simple_map
typed_rewrite_contract=present
typed_emission_contract=present
typed_boundary_contract=present
typed_entry_contract=present
binding_context_main_lines=1
variable_context_simple_map_main_lines=1
typed_converter_core=present
shared_renderer_layer=present
shared_generator_layer=present
route_selection_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
```

## Patch Sequence

If implementation starts, the first patch sequence must be:

1. Extend the shared spec carrier to represent a typed harness rewrite payload
   for the shared ordered-map family, while keeping the current raw harness
   rows intact until the slice is fully switched.
2. Teach the shared builder layer to render that typed harness payload for the
   selected family only.
3. Teach the shared emitter to consume the typed harness payload for the
   selected family only.
4. Switch `mirbuilder_family_artifacts.py` for BindingContext and VariableContext
   simple-map to the typed harness payload, leaving every other family
   untouched.

The sequence must keep these boundaries explicit:

- the first patch sequence is limited to the shared ordered-map family
- the first patch sequence keeps BindingContext and VariableContext simple-map
  as the only members
- the first patch sequence does not start BoxCompilationContext
- the first patch sequence does not start VariableContext snapshot/restore
- the first patch sequence does not start carrier snapshot contracts
- the first patch sequence does not start VariableContext immutable borrow
  ReturnSource
- the first patch sequence does not widen route selection
- the first patch sequence does not open nightly rustc adapter paths
- the first patch sequence does not change the typed converter core
- the first patch sequence does not add family-specific emitter branching

The sequence must also keep the deferred work explicit:

- leave `mirbuilder_carrier_snapshot_artifacts.py` untouched for this sequence
- leave `box_compilation_context_spec()` untouched for this sequence
- leave snapshot/restore and carrier snapshot slices for a later consultation
  row

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the first patch sequence is explicit and ordered
the first patch sequence is bounded to the shared ordered-map family
route selection remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no code change is started by this sequence
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_start_implementation_changes=1
do_not_expand_scope_beyond_shared_ordered_map_family=1
do_not_add_family_specific_emitter_branching=1
