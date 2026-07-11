---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-DEV-BIRTH-VERIFICATION-001
---

# MIRBUILDER-DEV-BIRTH-VERIFICATION-001

## Summary

`DevBirthVerification` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice fixes only the guarded
developer warning pass that scans `NewBox` instructions and checks nearby
`birth` calls. It does not claim module function insertion, condition_fn
injection, region cleanup, metadata publication, semantic refresh, full
finalize, generated Hako, backend routes, ABI changes, runtime fallback, or
source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-plan-v0.json`

## Guard Conditions

- `using_is_dev`
- `stageb_dev_verify_enabled`
- `cli_verbose_enabled`

## Verification Steps

- `IterateFunctionBlocks`
- `ScanNewBoxInstructions`
- `SkipStageBDriverBox`
- `SkipStringBox`
- `ExpectBirthTailByBoxTypeAndArity`
- `LookAheadThreeInstructions`
- `AcceptMethodBirthOnSameReceiver`
- `AcceptConstStringGlobalCompatibilityPath`
- `WarnOnMissingBirth`
- `WarnSummaryWhenAnyMissing`

## Artifacts

- `tools/rust_lifecycle/mirbuilder_dev_birth_verification.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-dev-birth-verification-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_dev_birth_verification_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.dev_birth_verification` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.module_function_insertion
callsite: MirBuilder::finalize_module -> module.add_function(function)
deny_reason: UnsupportedDirectShape
deny_detail: ModuleFunctionInsertionRequired
semantic_owner: MirBuilder::finalize_module module function insertion
next_slice_token: MIRBUILDER-MODULE-FUNCTION-INSERTION-001
```

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
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
```

## Acceptance

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_dev_birth_verification.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_dev_birth_verification_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
