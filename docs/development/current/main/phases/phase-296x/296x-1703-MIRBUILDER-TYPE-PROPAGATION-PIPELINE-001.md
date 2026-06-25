# 296x-1703 MIRBUILDER-TYPE-PROPAGATION-PIPELINE-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-TYPE-PROPAGATION-PIPELINE-001

## Purpose

Close the TypePropagationPipeline frontier edge for the prepared-state minimal
MirBuilder path. This slice makes the existing
`TypePropagationPipeline::run(&mut function, &mut self.type_ctx.value_types)`
SSOT entry an explicit source-derived capability provider, without
implementing type-hint provision, metadata publication, PHI return inference,
full finalize, generated Hako, backend routes, ABI changes, or runtime
behavior.

## Source Authority

```text
source:
  src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module
  src/mir/type_propagation/pipeline.rs::TypePropagationPipeline::run

predecessor:
  MirBuilderCurrentFunctionTakePlanV1
    non_claims.type_propagation = 0
```

The selected edge is limited to:

```text
call:
  TypePropagationPipeline::run(&mut function, &mut self.type_ctx.value_types)

pipeline order:
  seed_declared_field_types
  copy_propagation_initial
  binop_repropagation
  copy_propagation_after_binop
  phi_type_inference
```

## Capability

```text
provider:
  MirBuilderTypePropagationPipelinePlanV1

capability:
  TypePropagationPipelineExecution

entrypoint:
  TypePropagationPipeline::run

mutates:
  function
  self.type_ctx.value_types
```

## Derived Frontier Result

After registering `TypePropagationPipelineExecution` as a `PlanOnly` provider,
the frontier analyzer advances to the next live edge:

```text
edge:
  finalize_module.type_hint_provision

callsite:
  MirBuilder::finalize_module -> annotate missing call/await result types

detail:
  TypeHintProvisionRequired

next slice:
  MIRBUILDER-TYPE-HINT-PROVISION-001
```

## Non-Claims

```text
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
  tools/rust_lifecycle/mirbuilder_type_propagation_pipeline.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_type_propagation_pipeline_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
