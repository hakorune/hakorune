# 296x-1701 MIRBUILDER-TYPED-VALUE-VERIFICATION-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-TYPED-VALUE-VERIFICATION-001

## Purpose

Close the typed-value verification frontier edge for the prepared-state minimal
MirBuilder path. This slice makes
`verify_typed_values_are_defined(self, "finalize_module")` an explicit
source-derived capability provider, without implementing current-function
take, type propagation, PHI inference, full finalize, generated Hako, backend
routes, ABI changes, or runtime behavior.

## Source Authority

```text
source:
  src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module
  src/mir/builder/emission/value_lifecycle.rs::verify_typed_values_are_defined

predecessor:
  MirBuilderCurrentModuleTakePlanV1
    non_claims.verify_typed_values = 0

input type owner:
  MirBuilderLiteralIntegerLoweringPlanV1
    result_contract.published_type = MirType::Integer
```

The selected edge is limited to:

```text
call:
  verify_typed_values_are_defined(self, "finalize_module")

contract:
  every typed ValueId is defined by the current function or is a function
  parameter before finalization proceeds
```

## Capability

```text
provider:
  MirBuilderTypedValueVerificationPlanV1

capability:
  TypedValueDefinitionVerification

definition sources:
  compute_def_blocks(func)
  func.params

fail-fast:
  [freeze:contract][value_lifecycle/typed_without_def]
```

## Derived Frontier Result

After registering `TypedValueDefinitionVerification` as a `PlanOnly` provider,
the frontier analyzer advances to the next live edge:

```text
edge:
  finalize_module.take_current_function

callsite:
  MirBuilder::finalize_module -> take current_function

detail:
  CurrentFunctionTakeRequired

next slice:
  MIRBUILDER-CURRENT-FUNCTION-TAKE-001
```

## Non-Claims

```text
current_function_take = 0
type_propagation = 0
type_hint_provision = 0
phi_return_type_inference = 0
phi_input_materialization = 0
module_metadata_publication = 0
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
python3 -m py_compile \
  tools/rust_lifecycle/mirbuilder_typed_value_verification.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_typed_value_verification_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
