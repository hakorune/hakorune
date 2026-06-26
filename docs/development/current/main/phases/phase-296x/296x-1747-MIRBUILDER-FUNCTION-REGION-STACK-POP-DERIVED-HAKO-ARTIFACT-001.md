---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` FunctionRegionStackPop is now checked in as a
derived Hako artifact under `lang/generated/rust_derived/hakorune_mir_builder/`.
The semantic-closure report has been regenerated so the first remaining
materialization gap now advances to `finalize_module.slot_registry_release`.

## Authority

Semantic source:

```text
MirBuilderFunctionRegionStackPopPlanV1
  -> FunctionRegionStackPopExecutionProjectionV1
  -> derived Hako artifact
```

Implemented surface:

```text
tools/rust_lifecycle/mirbuilder_function_region_stack_pop_artifacts.py
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.artifact.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-oracle-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-recipe-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-verifier-result-v0.json
```

The artifact stays a family-scoped derived landing, not a backend route or ABI
expansion.

## Acceptance

```text
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-function-region-stack-pop --check = green
python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_semantic_closure_report.py --check-reference --drift-probes = green
bash tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_derived_artifact_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
cargo check --release = green
git diff --check = green
```

## Non-Claims

```text
new_abi = 0
new_backend_route = 0
runtime_fallback = 0
source_selfhost_claim = 0
slot_registry_release = 0
host_env_lookup = 0
full_finalize_module = 0
```

## Next

```text
MIRBUILDER-SLOT-REGISTRY-RELEASE-DERIVED-HAKO-ARTIFACT-001
```
