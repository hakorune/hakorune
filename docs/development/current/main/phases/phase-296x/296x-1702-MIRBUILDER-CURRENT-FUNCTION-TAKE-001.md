# 296x-1702 MIRBUILDER-CURRENT-FUNCTION-TAKE-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-CURRENT-FUNCTION-TAKE-001

## Purpose

Close the current-function take frontier edge for the prepared-state minimal
MirBuilder path. This slice makes
`self.scope_ctx.current_function.take().unwrap()` an explicit source-derived
capability provider, without implementing type propagation, type-hint
provision, PHI inference, full finalize, generated Hako, backend routes, ABI
changes, or runtime behavior.

## Source Authority

```text
source:
  src/mir/builder/module_lifecycle.rs::MirBuilder::prepare_module
  src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module

predecessor:
  MirBuilderTypedValueVerificationPlanV1
    non_claims.current_function_take = 0

function transport owner:
  MirFunctionConstructorCompositionPlanV1
```

The selected edge is limited to:

```text
install:
  self.scope_ctx.current_function = Some(main_function)

take:
  let mut function = self.scope_ctx.current_function.take().unwrap()
```

## Capability

```text
provider:
  MirBuilderCurrentFunctionTakePlanV1

capability:
  CurrentFunctionTake

result contract:
  taken_value = MirFunctionPreparedMain
  source_state = self.scope_ctx.current_function
  post_take_state = None
  local_binding = function
```

## Derived Frontier Result

After registering `CurrentFunctionTake` as a `PlanOnly` provider, the frontier
analyzer advances to the next live edge:

```text
edge:
  finalize_module.type_propagation

callsite:
  MirBuilder::finalize_module -> TypePropagationPipeline::run

detail:
  TypePropagationPipelineRequired

next slice:
  MIRBUILDER-TYPE-PROPAGATION-PIPELINE-001
```

## Non-Claims

```text
type_propagation = 0
type_hint_provision = 0
metadata_value_type_publication = 0
phi_return_type_inference = 0
phi_input_materialization = 0
module_function_insertion = 0
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
  tools/rust_lifecycle/mirbuilder_current_function_take.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_current_function_take_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
