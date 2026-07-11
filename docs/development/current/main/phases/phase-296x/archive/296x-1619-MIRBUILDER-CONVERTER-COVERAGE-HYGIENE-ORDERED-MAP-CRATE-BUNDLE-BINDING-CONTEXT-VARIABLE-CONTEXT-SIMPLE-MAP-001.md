# 296x-1619 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-ORDERED-MAP-CRATE-BUNDLE-BINDING-CONTEXT-VARIABLE-CONTEXT-SIMPLE-MAP-001

Status: Landed
Date: 2026-06-22

## Purpose

Implement the first representative easy-tier crate-level bundle for the
selected ordered-map MirBuilder harness family. The bundle stays bounded to
`BindingContext` and `VariableContext simple-map` and composes the already
landed typed family slices into one crate-level executable bridge.

## Scope

```text
BoxCount: one bundle implementation contract
owner: MirBuilder converter coverage hygiene ordered-map crate bundle
input: landed BindingContext and VariableContext simple-map typed harness artifacts
output: representative crate-level bundle artifact
```

## Observed State

```text
selected_slice=BindingContext_and_VariableContext_simple_map
binding_context_typed_harness=landed
variable_context_simple_map_typed_harness=landed
crate_level_bundle_opened=0
crate_linker_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
new_family_selection_opened=0
new_route_selection_opened=0
```

## Bundle Contract

The first representative easy-tier crate-level bundle must stay minimal and
explicit:

- the bundle stays on the selected ordered-map family only
- the bundle composes the landed BindingContext and VariableContext simple-map
  harness slices
- the bundle uses the existing typed harness payload path
- the bundle does not open the crate linker
- the bundle does not open the nightly rustc adapter path
- the bundle does not add runtime fallback
- the bundle does not widen family selection beyond the ordered-map slice

## Deferred Work

Keep these work items deferred until a later implementation slice:

- crate linker
- crate surface facts
- family-wide typed harness migration
- CoreContext integration
- TypeContext integration
- MetadataContext integration
- CarrierInfo integration
- BoxCompilationContext integration
- full MirBuilder crate coverage

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the ordered-map crate-level bundle contract is explicit
the bundle remains bounded to BindingContext and VariableContext simple-map
route selection remains unopened
crate linker remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
no family selection beyond the ordered-map slice is opened
```

## Stop Line

```text
do_not_open_crate_linker=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_open_new_family_selection=1
do_not_open_new_route_selection=1
do_not_start_unbounded_crate_coverage=1
```

## Closeout

```text
output_contract=rust-lifecycle-mirbuilder-easy-v0-ordered-map-bundle-v4
family_id=hakorune_mir_builder::ordered_map_bundle
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
generated_hako_exe_aot=green
summary=ok
```
