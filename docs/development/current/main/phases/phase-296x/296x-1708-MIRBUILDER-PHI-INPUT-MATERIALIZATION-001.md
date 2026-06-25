---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-PHI-INPUT-MATERIALIZATION-001
---

# MIRBUILDER-PHI-INPUT-MATERIALIZATION-001

## Summary

`PhiInputMaterialization` is now a source-derived PlanOnly capability for the
prepared-state minimal MirBuilder path. The slice fixes the existing
`phi_input_materializer::materialize_all_phi_inputs` delegation and helper
shape without claiming dev birth verification, module insertion, semantic
refresh, full finalize, generated Hako, backend routes, ABI changes, runtime
fallback, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- `src/mir/builder/ssa/phi_input_materializer.rs::materialize_all_phi_inputs`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-return-type-inference-plan-v0.json`

## Materialization Steps

- `PruneUnusedPhiInstructions`
- `CompleteMissingSelfCarriedPhiInputs`
- `CollectPhiInputWorklist`
- `BuildDefBlocksAndDominators`
- `RematerializeIncomingPerPredWithMemo`
- `RewritePhiInputSlots`
- `ReturnChangedCount`

## Artifacts

- `tools/rust_lifecycle/mirbuilder_phi_input_materialization.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_phi_input_materialization_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.phi_input_materialization` as `Available`.

The next derived unsupported edge is:

```text
edge_id: finalize_module.dev_birth_verification
callsite: MirBuilder::finalize_module -> dev NewBox birth verification
deny_reason: UnsupportedDirectShape
deny_detail: DevBirthVerificationRequired
semantic_owner: MirBuilder::finalize_module dev birth verification
next_slice_token: MIRBUILDER-DEV-BIRTH-VERIFICATION-001
```

## Non-Claims

```text
dev_birth_verification = 0
module_function_insertion = 0
condition_fn_injection = 0
all_functions_phi_materialization = 0
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
python3 -m py_compile tools/rust_lifecycle/mirbuilder_phi_input_materialization.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_phi_input_materialization_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
