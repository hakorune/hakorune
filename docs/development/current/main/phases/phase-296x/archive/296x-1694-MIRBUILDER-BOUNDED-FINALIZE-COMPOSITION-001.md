# 296x-1694 MIRBUILDER-BOUNDED-FINALIZE-COMPOSITION-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-BOUNDED-FINALIZE-COMPOSITION-001

## Purpose

Close the next minimal-execution frontier edge:

```text
MirBuilder::finalize_module
```

The slice records the live finalize sequence needed by the prepared-state
`build_module(AST Literal Integer(0))` profile. It is a PlanOnly capability and
does not claim full finalize behavior or generated Hako execution.

## Source Authority

```text
src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module
src/mir/function/module_impl.rs::MirModule::add_function
src/mir/type_propagation/pipeline.rs::TypePropagationPipeline
src/mir/semantic_refresh.rs::refresh_module_record_and_packed_layout_plans
```

## Plan

```text
FinalizeModuleComposition:
  append Return(result_value) if current block is open
  update main signature return type from type_ctx.value_types[result_value]
  take current module and function
  verify typed values
  run TypePropagationPipeline
  publish value_types and value_origin_callers metadata
  run PHI return inference and PHI input materialization
  add main function to module
  inject condition_fn if missing
  pop region and clear slot registry
  publish module declaration metadata
  refresh record/typed-object/direct-state plan subsets
  materialize PHI inputs for all functions
  return module
```

## Frontier Result

After this plan is available, the minimal execution path analyzer advances to:

```text
callsite:
  PreparedMirBuilderStateV1 build_module(ASTNode::Literal(Integer(0))) smoke

reason:
  UnsupportedDirectShape

detail:
  MinimalExecutionPathSmokeRequired

next slice:
  MIRBUILDER-MINIMAL-EXECUTION-PATH-SMOKE-001
```

## Non-Claims

```text
full finalize_module = 0
other root shapes = 0
condition_fn policy generalization = 0
semantic refresh full claim = 0
generated Hako artifact = 0
backend route = 0
ABI = 0
runtime fallback = 0
mainline selected = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_bounded_finalize_composition_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_bounded_finalize_composition.py
bash tools/checks/current_state_pointer_guard.sh
```
