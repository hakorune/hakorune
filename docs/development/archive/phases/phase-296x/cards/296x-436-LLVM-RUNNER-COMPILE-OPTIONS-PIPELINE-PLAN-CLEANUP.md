---
Status: Done
Date: 2026-06-06
Scope: route current LLVM runner env/stage decisions through named plan boxes.
Blocker: LLVM-PIPE-003
Related:
  - docs/development/current/main/phases/phase-296x/296x-434-LLVM-RUNNER-PIPELINE-DEBT-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-435-LLVM-RUNNER-PIPELINE-REPORT-FIELDS.md
  - src/runner/product/llvm/compile_options.rs
  - src/runner/product/llvm/pipeline_plan.rs
  - src/runner/product/llvm/pipeline_report.rs
---

# 296x-436 LLVM Runner CompileOptions / PipelinePlan Cleanup

## Purpose

`LLVM-PIPE-001` made pipeline debt visible. `LLVM-PIPE-002` added opt-in
runtime evidence. This row performs the first cleanup step: the current LLVM
runner defaults now flow through named `LlvmCompileOptions` and
`LlvmPipelinePlan` boxes instead of being hardcoded directly in the runner
body.

This row keeps behavior unchanged.

## Decision

```text
compile_options_v0=1
pipeline_plan_v0=1
runner_behavior_change=0
backend_selection_change=0
fallback_policy_change=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

Current plan shape:

```text
LlvmCompileOptions:
  future_rewrite_route=EnvFutureExterns

LlvmPipelinePlan:
  compile_options=current_default
  method_id_injector_enabled=1
  joinir_experiment_hook_enabled=1
```

`NYASH_REWRITE_FUTURE` is still used internally by the MIR normalization
contract, but it is selected by `LlvmCompileOptions` rather than being an
unnamed runner side effect.

## Report Fields

Runtime report now includes:

```text
pipeline_plan_v0=1
compile_options_v0=1
mir_future_rewrite_option=env_future_externs
mir_future_rewrite_route=env_forced_llvm_future_externs
method_id_injector_plan_enabled=1
joinir_experiment_hook_plan_enabled=1
```

## Acceptance

```bash
cargo check -q
bash tools/smokes/v2/profiles/integration/llvm/llvm_pipeline_runtime_report.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

- do not change executor order in this row
- do not change fallback policy in this row
- do not remove PyVM diagnostic reachability in this row
- do not change Future rewrite semantics in this row
- do not implement FastMem/MemOp lowering in this row
- do not open product allocator activation

## Follow-Up

```text
MIR-FMEM-001:
  consult and document the MIRBuilder FastMemRegion/MemOp representation
  boundary before adding MIR contracts or lowering.
```

## Landed Evidence

```text
pipeline_plan_v0=1
compile_options_v0=1
mir_future_rewrite_option=env_future_externs
mir_future_rewrite_route=env_forced_llvm_future_externs
method_id_injector_plan_enabled=1
joinir_experiment_hook_plan_enabled=1
execution_backend=mock
llvm_fallback_used=1
runner_behavior_change=0
```

Next row:

```text
MIR-FMEM-001 MIRBuilder FastMemRegion/MemOp design consultation
```
