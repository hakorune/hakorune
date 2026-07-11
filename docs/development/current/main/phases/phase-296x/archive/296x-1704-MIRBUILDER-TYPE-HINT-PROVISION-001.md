# 296x-1704 MIRBUILDER-TYPE-HINT-PROVISION-001

Status: Landed
Date: 2026-06-25

## Summary

`TypeHintProvision` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path.

The slice records the existing
`type_hint_providers::annotate_missing_result_types_from_calls_and_await`
delegation in `MirBuilder::finalize_module` without changing generated Hako,
backend routes, ABI, or runtime behavior.

## Source Authority

```text
src/mir/builder/module_lifecycle.rs
  MirBuilder::finalize_module

src/mir/builder/type_hint_providers.rs
  annotate_missing_result_types_from_calls_and_await
```

The provider scans the prepared function and annotates missing result types for:

```text
Await
Call(Global)
Call(Constructor)
Call(OtherOrMissingCallee)
```

## Artifact

```text
tools/rust_lifecycle/mirbuilder_type_hint_provision.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-plan-v0.json
tools/checks/rust_lifecycle_mirbuilder_type_hint_provision_guard.sh
```

The plan exposes:

```text
available_capability = TypeHintProvision
entrypoint = type_hint_providers::annotate_missing_result_types_from_calls_and_await
```

## Derived Frontier Result

The minimal execution path analyzer now marks:

```text
finalize_module.type_hint_provision
  status = Available
```

The next first unsupported edge is:

```text
edge_id = finalize_module.metadata_value_type_publication
callsite = MirBuilder::finalize_module -> publish function.metadata.value_types
deny_reason = UnsupportedDirectShape
deny_detail = MetadataValueTypePublicationRequired
next_slice_token = MIRBUILDER-METADATA-VALUE-TYPE-PUBLICATION-001
```

## Non-Claims

```text
metadata_value_type_publication = 0
metadata_origin_caller_merge = 0
phi_return_type_inference = 0
phi_input_materialization = 0
module_function_insertion = 0
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
```

## Acceptance

```text
python3 tools/rust_lifecycle/mirbuilder_type_hint_provision.py --check-reference --drift-probes
bash tools/checks/rust_lifecycle_mirbuilder_type_hint_provision_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
cargo check --release
```
