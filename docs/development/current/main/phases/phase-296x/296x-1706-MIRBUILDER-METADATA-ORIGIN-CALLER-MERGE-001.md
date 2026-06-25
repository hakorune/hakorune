# 296x-1706 MIRBUILDER-METADATA-ORIGIN-CALLER-MERGE-001

Status: Landed
Date: 2026-06-25

## Summary

`MetadataOriginCallerMerge` is now a source-derived PlanOnly capability for
the prepared-state minimal MirBuilder path.

The slice records the existing `finalize_module` merge:

```text
let mut origin_callers = function.metadata.value_origin_callers.clone()
for (k, v) in self.metadata_ctx.value_origin_callers().iter()
  origin_callers.insert(*k, v.clone())
function.metadata.value_origin_callers = origin_callers
```

It does not claim PHI return-type inference, PHI input materialization, module
insertion, generated Hako, backend routes, ABI, or runtime behavior.

## Source Authority

```text
src/mir/builder/module_lifecycle.rs
  MirBuilder::finalize_module
```

The source order is fixed as:

```text
metadata value-type publication
  -> clone existing function value_origin_callers
  -> iterate builder MetadataContext value_origin_callers
  -> insert cloned values with SourceWins collision policy
  -> assign back to function metadata
  -> PHI return type inference
```

## Artifact

```text
tools/rust_lifecycle/mirbuilder_metadata_origin_caller_merge.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-metadata-origin-caller-merge-plan-v0.json
tools/checks/rust_lifecycle_mirbuilder_metadata_origin_caller_merge_guard.sh
```

The plan exposes:

```text
available_capability = MetadataOriginCallerMerge
merge.collision_policy = SourceWins
```

## Derived Frontier Result

The minimal execution path analyzer now marks:

```text
finalize_module.metadata_origin_caller_merge
  status = Available
```

The next first unsupported edge is:

```text
edge_id = finalize_module.phi_return_type_inference
callsite = MirBuilder::finalize_module -> infer return type from PHI
deny_reason = UnsupportedDirectShape
deny_detail = PhiReturnTypeInferenceRequired
next_slice_token = MIRBUILDER-PHI-RETURN-TYPE-INFERENCE-001
```

## Non-Claims

```text
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
python3 tools/rust_lifecycle/mirbuilder_metadata_origin_caller_merge.py --check-reference --drift-probes
bash tools/checks/rust_lifecycle_mirbuilder_metadata_origin_caller_merge_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
cargo check --release
```
