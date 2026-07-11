---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-TYPE-HINT-PROVISION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-TYPE-HINT-PROVISION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` type-hint provision now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes only the existing source-derived provider cases:

```text
Await
Call(Global)
Call(Constructor)
Call(OtherOrMissingCallee)
```

It records the prepared function/module/type-context mutation frame and returns
a `TypeHintProvisionResultBox`. It does not publish metadata value types, merge
origin callers, infer PHI return types, materialize PHI inputs, insert module
functions, or claim full finalize.

## Authority

Semantic source:

```text
MirBuilderTypeHintProvisionPlanV1
  -> TypeHintProvision DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.artifact.json
```

The artifact is an executable materialization of the existing type-hint
provision plan. It is not metadata publication and does not reinterpret later
finalize stages.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
TypeHintProvisionApi.run/3 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = TypeHintProvisionResultBox
type_hint_provision = 1
metadata_value_type_publication = 0
metadata_origin_caller_merge = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.metadata_value_type_publication
next_slice = MIRBUILDER-METADATA-VALUE-TYPE-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_type_hint_provision_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_type_hint_provision_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-type-hint-provision-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_type_hint_provision.artifact.json`

## Non-Claims

```text
metadata_value_type_publication = 0
metadata_origin_caller_merge = 0
phi_return_type_inference = 0
phi_input_materialization = 0
module_function_insertion = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_type_hint_provision_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-type-hint-provision --check
bash tools/checks/rust_lifecycle_mirbuilder_type_hint_provision_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
