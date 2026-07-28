---
Status: Done
Date: 2026-06-06
Scope: add opt-in dynamic LLVM runner pipeline report fields.
Blocker: LLVM-PIPE-002
Related:
  - docs/development/current/main/phases/phase-296x/296x-434-LLVM-RUNNER-PIPELINE-DEBT-INVENTORY.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - tools/smokes/v2/profiles/integration/llvm/llvm_pipeline_runtime_report.sh
  - src/runner/product/llvm/pipeline_report.rs
---

# 296x-435 LLVM Runner Pipeline Report Fields

## Purpose

`LLVM-PIPE-001` made the LLVM runner seams visible as static inventory. This
row adds an opt-in runtime report so actual executor selection and fallback use
can be observed before cleanup moves env side effects and ad-hoc stages into
explicit plan objects.

This row is diagnostic/report-only.

## Decision

```text
llvm_pipeline_runtime_report_v0=1
report_env=NYASH_LLVM_PIPELINE_REPORT_OUT
report_default=unset
runner_behavior_change=0
backend_selection_change=0
fallback_policy_change=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

`NYASH_LLVM_PIPELINE_REPORT_OUT` writes a kv report only when explicitly set.
It is not a route selector and does not authorize fallback.

## Fields

```text
output_contract=hako-llvm-pipeline-runtime-report-v0
tool_surface=llvm_runner_pipeline_report
observation_only=1
behavior_change=0
mir_future_rewrite_route=env_forced_llvm_future_externs
pipeline_joinir_experiment_enabled=0|1
method_id_injector_mutation_count=<n>
execution_backend=pyvm|obj_out|ny_llvmc_exe|mock|pyvm_error|not_selected
llvm_fallback_used=0|1
llvm_fallback_reason=none|harness_unavailable_or_not_requested|...
pyvm_requested=0|1
harness_requested=0|1
mock_fallback_used=0|1
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
summary=ok
```

## Acceptance

```bash
cargo check -q
bash tools/smokes/v2/profiles/integration/llvm/llvm_pipeline_runtime_report.sh
bash tools/checks/current_state_pointer_guard.sh
```

The smoke exercises the current no-LLVM-feature mock route and expects:

```text
execution_backend=mock
llvm_fallback_used=1
llvm_fallback_reason=harness_unavailable_or_not_requested
mock_fallback_used=1
pyvm_requested=0
harness_requested=0
```

## Stop Line

- do not change executor order in this row
- do not change harness/mock fallback policy in this row
- do not remove PyVM diagnostic reachability in this row
- do not replace `NYASH_REWRITE_FUTURE` with `CompileOptions` in this row
- do not convert `method_id_injector` / `joinir_experiment` into plan stages in
  this row
- do not open product allocator activation

## Follow-Up

```text
LLVM-PIPE-003:
  move env side effects and runner ad-hoc stages toward explicit
  CompileOptions / PipelinePlan / LoweringPlan.
```

## Landed Evidence

```text
output_contract=hako-llvm-pipeline-runtime-report-v0
mir_future_rewrite_route=env_forced_llvm_future_externs
pipeline_plan_v0=1
compile_options_v0=1
mir_future_rewrite_option=env_future_externs
pipeline_joinir_experiment_enabled=0
method_id_injector_plan_enabled=1
method_id_injector_mutation_count=0
joinir_experiment_hook_plan_enabled=1
execution_backend=mock
llvm_fallback_used=1
llvm_fallback_reason=harness_unavailable_or_not_requested
mock_fallback_used=1
runner_behavior_change=0
```

Next row:

```text
LLVM-PIPE-003 CompileOptions / PipelinePlan cleanup
```
