# 296x-1707 MIRBUILDER-PHI-RETURN-TYPE-INFERENCE-001

Status: Landed
Date: 2026-06-25

## Summary

`PhiReturnTypeInference` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path.

The slice records the existing `finalize_module` delegation:

```text
phi_type_inference::infer_return_type_from_phi(self, &mut function)
  -> if Some(inferred_type), function.signature.return_type = inferred_type
```

It does not claim PHI input materialization, module insertion, full finalize,
generated Hako, backend routes, ABI, or runtime behavior.

## Source Authority

```text
src/mir/builder/module_lifecycle.rs
  MirBuilder::finalize_module

src/mir/builder/phi_type_inference.rs
  infer_return_type_from_phi
```

The resolver chain is fixed as:

```text
SkipConcreteReturnType
TerminatorReturnOnly
DirectValueTypesLookup
TypeHintPolicyExtract
MethodReturnHintBox
PhiTypeResolver
GenericTypeResolver
UnknownFallbackOutsideDebug
```

## Artifact

```text
tools/rust_lifecycle/mirbuilder_phi_return_type_inference.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-plan-v0.json
tools/checks/rust_lifecycle_mirbuilder_phi_return_type_inference_guard.sh
```

The plan exposes:

```text
available_capability = PhiReturnTypeInference
entrypoint = phi_type_inference::infer_return_type_from_phi
```

## Derived Frontier Result

The minimal execution path analyzer now marks:

```text
finalize_module.phi_return_type_inference
  status = Available
```

The next first unsupported edge is:

```text
edge_id = finalize_module.phi_input_materialization
callsite = MirBuilder::finalize_module -> materialize all PHI inputs
deny_reason = UnsupportedDirectShape
deny_detail = PhiInputMaterializationRequired
next_slice_token = MIRBUILDER-PHI-INPUT-MATERIALIZATION-001
```

## Non-Claims

```text
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
python3 tools/rust_lifecycle/mirbuilder_phi_return_type_inference.py --check-reference --drift-probes
bash tools/checks/rust_lifecycle_mirbuilder_phi_return_type_inference_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
cargo check --release
```
