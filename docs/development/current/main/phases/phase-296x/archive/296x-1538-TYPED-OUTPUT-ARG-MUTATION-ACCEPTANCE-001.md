# 296x-1538 TYPED-OUTPUT-ARG-MUTATION-ACCEPTANCE-001

Status: closed
Date: 2026-06-21

## Purpose

Accept typed object output-argument mutation so generated bridge APIs can be
executed directly instead of duplicating their operation body in `Main`.

Current workaround:

```text
generated CarrierInfo artifact keeps CarrierInfoApi.from_snapshot(...)
but Main inlines the operation body for EXE smoke
```

Desired accepted shape:

```hako
local carrier_data = OrderedMap.create()
CarrierInfoApi.from_snapshot(carrier_data, "i", snapshot)
```

and `carrier_data` contains the fields written by the static API after return.

## Scope

```text
BoxCount: one parameter-mutation visibility shape
owner: backend route / object parameter materialization
input: typed OrderedMapBox parameter passed to static API
output: caller observes set/remove/clear mutations after call
```

## Acceptance

```text
variable_context_carrier_snapshot generated Main calls
  CarrierInfoApi.from_snapshot(carrier_data, "i", snapshot)

variable_context_explicit_carrier_snapshot generated Main calls
  CarrierInfoApi.with_explicit_carriers_from_snapshot(...)

bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_derived_artifact_guard.sh
  generated_hako_exe=green
  no inlined duplicate carrier projection in Main
```

Result:

```text
CarrierInfoApi.from_snapshot(info, "i", snapshot) runs in generated Main.
CarrierInfoApi.with_explicit_carriers_from_snapshot(...) runs in generated Main.
typed_output_arg_mutation=green
main_inlined_duplicate_carrier_projection=0
```

## Stop Line

```text
do_not_reintroduce_return_ctx_variable_map=1
do_not_use_runtime_try_hako_then_rust_fallback=1
do_not_change_ordered_map_semantics=1
do_not_add_static_method_by_name_special_case=1
```
