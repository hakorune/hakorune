---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-TYPED-VALUE-VERIFICATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-TYPED-VALUE-VERIFICATION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` typed-value definition verification now has a
focused DerivedShadow Hako artifact for the prepared minimal profile. The
artifact materializes only:

```text
typed result value
  -> defined value shell
  -> TypedValueVerificationResultBox
```

It does not take the current function, run type propagation, infer PHI return
types, materialize PHI inputs, publish module metadata, or claim full finalize.

## Authority

Semantic source:

```text
MirBuilderTypedValueVerificationPlanV1
  -> TypedValueDefinitionVerification DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.artifact.json
```

The artifact is an executable materialization of the existing typed-value
verification plan. It is not a full `finalize_module` artifact and does not
reinterpret current-function ownership or type propagation.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
TypedValueVerificationApi.verify/4 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = TypedValueVerificationResultBox
typed_value_verification = 1
current_function_take = 0
type_propagation = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.take_current_function
next_slice = MIRBUILDER-CURRENT-FUNCTION-TAKE-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_typed_value_verification_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_typed_value_verification_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-value-verification-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-value-verification-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-typed-value-verification-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_typed_value_verification.artifact.json`

## Non-Claims

```text
current_function_take = 0
type_propagation = 0
type_hint_provision = 0
phi_return_type_inference = 0
phi_input_materialization = 0
module_metadata_publication = 0
full_finalize_module = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_typed_value_verification_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-typed-value-verification --check
bash tools/checks/rust_lifecycle_mirbuilder_typed_value_verification_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
