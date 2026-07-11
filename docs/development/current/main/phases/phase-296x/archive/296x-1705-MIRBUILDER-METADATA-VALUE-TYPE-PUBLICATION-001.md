# 296x-1705 MIRBUILDER-METADATA-VALUE-TYPE-PUBLICATION-001

Status: Landed
Date: 2026-06-25

## Summary

`MetadataValueTypePublication` is now a source-derived PlanOnly capability for
the prepared-state minimal MirBuilder path.

The slice records the existing publication:

```text
function.metadata.value_types = self.type_ctx.value_types.clone()
```

It does not claim the following `value_origin_callers` merge, PHI return-type
inference, PHI input materialization, module insertion, generated Hako, backend
routes, ABI, or runtime behavior.

## Source Authority

```text
src/mir/builder/module_lifecycle.rs
  MirBuilder::finalize_module
```

The source order is fixed as:

```text
type_hint_providers::annotate_missing_result_types_from_calls_and_await
  -> function.metadata.value_types publication
  -> value_origin_callers merge
```

## Artifact

```text
tools/rust_lifecycle/mirbuilder_metadata_value_type_publication.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-value-type-publication-plan-v0.json
tools/checks/rust_lifecycle_mirbuilder_metadata_value_type_publication_guard.sh
```

The plan exposes:

```text
available_capability = MetadataValueTypePublication
publication.operation = CloneOwnedMap
```

## Derived Frontier Result

The minimal execution path analyzer now marks:

```text
finalize_module.metadata_value_type_publication
  status = Available
```

The next first unsupported edge is:

```text
edge_id = finalize_module.metadata_origin_caller_merge
callsite = MirBuilder::finalize_module -> merge function.metadata.value_origin_callers
deny_reason = UnsupportedDirectShape
deny_detail = MetadataOriginCallerMergeRequired
next_slice_token = MIRBUILDER-METADATA-ORIGIN-CALLER-MERGE-001
```

## Non-Claims

```text
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
python3 tools/rust_lifecycle/mirbuilder_metadata_value_type_publication.py --check-reference --drift-probes
bash tools/checks/rust_lifecycle_mirbuilder_metadata_value_type_publication_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
cargo check --release
```
