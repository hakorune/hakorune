---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-DEV-BIRTH-VERIFICATION-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-DEV-BIRTH-VERIFICATION-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::finalize_module` dev NewBox birth verification now has a focused
DerivedShadow Hako artifact for the prepared minimal profile. The artifact
materializes the source-derived guard conditions and warning-only verification
step frame:

```text
MirBuilder::finalize_module dev birth verification block
```

It records the guard condition count, verification step count, warning count,
and no-mutation frame. It does not claim module function insertion,
condition_fn injection, all-functions PHI materialization, region cleanup,
metadata publication, semantic refresh, or full `finalize_module`.

## Authority

Semantic source:

```text
MirBuilderDevBirthVerificationPlanV1
  -> DevBirthVerification DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.artifact.json
```

The artifact is an executable materialization of the existing dev birth
verification plan. It is not module insertion, condition_fn injection,
metadata publication, semantic refresh, or full finalize_module.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
DevBirthVerificationApi.run/1 direct same-module route = green
proof = typed_global_call_same_module_object_handle
definition_owner = uniform_mir
result box = DevBirthVerificationResultBox
guard_conditions = 3
verification_steps = 10
warnings = 0
mutates_function = 0
dev_birth_verification = 1
module_function_insertion = 0
condition_fn_injection = 0
full_finalize_module = 0
runtime_fallback = 0
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.module_function_insertion
next_slice = MIRBUILDER-MODULE-FUNCTION-INSERTION-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_dev_birth_verification_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_dev_birth_verification_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_dev_birth_verification.artifact.json`

## Non-Claims

```text
module_function_insertion = 0
condition_fn_injection = 0
all_functions_phi_materialization = 0
region_stack_pop = 0
slot_registry_release = 0
metadata_publication = 0
semantic_refresh = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_dev_birth_verification_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-dev-birth-verification --check
bash tools/checks/rust_lifecycle_mirbuilder_dev_birth_verification_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
