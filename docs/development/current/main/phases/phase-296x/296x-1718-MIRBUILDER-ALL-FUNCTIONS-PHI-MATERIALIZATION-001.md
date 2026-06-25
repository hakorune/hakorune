---
Status: Landed
Date: 2026-06-25
Card: MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-001
---

# MIRBUILDER-ALL-FUNCTIONS-PHI-MATERIALIZATION-001

## Summary

`AllFunctionsPhiMaterialization` is now a source-derived PlanOnly capability
for the prepared-state minimal MirBuilder path. The slice owns only the
`finalize_module` sweep over `module.functions.values_mut()` and its
delegation to the existing `PhiInputMaterialization` provider with the
`finalize_module_all_functions` context. It does not re-own PHI
materializer internals, full finalize, generated Hako, backend routes, ABI
changes, runtime fallback, mainline selection, or source selfhost.

## Source Authority

- `src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module`
- Delegate plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-phi-input-materialization-plan-v0.json`
- Predecessor plan:
  `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-direct-state-plan-refresh-plan-v0.json`

## Sweep Contract

```text
iteration = for function in module.functions.values_mut()
delegate = phi_input_materializer::materialize_all_phi_inputs
delegate_context = finalize_module_all_functions
delegate_capability = PhiInputMaterialization
error_transport = ResultPropagatedByQuestionMark
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_all_functions_phi_materialization.py`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-all-functions-phi-materialization-plan-v0.json`
- `tools/checks/rust_lifecycle_mirbuilder_all_functions_phi_materialization_guard.sh`

## Derived Frontier Result

The minimal execution path analyzer now marks
`finalize_module.all_functions_phi_materialization` as `Available`.

The next derived unsupported edge is a design stop:

```text
edge_id: minimal_path.completion_design_stop
callsite: MinimalMirBuilderExecutionPath -> post-finalize completion design stop
deny_reason: UnsupportedDirectShape
deny_detail: MinimalExecutionPathCompletionDesignReviewRequired
semantic_owner: Minimal MirBuilder execution path completion review
next_slice_token: MIRBUILDER-MINIMAL-EXECUTION-PATH-COMPLETION-DESIGN-STOP-001
```

## Non-Claims

```text
full_finalize_module = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
source_selfhost_claim = 0
```

## Acceptance

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_all_functions_phi_materialization.py tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_all_functions_phi_materialization_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
