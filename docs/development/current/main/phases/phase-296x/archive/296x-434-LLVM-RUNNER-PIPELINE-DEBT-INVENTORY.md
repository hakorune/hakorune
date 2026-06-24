---
Status: Done
Date: 2026-06-06
Scope: inventory the current LLVM runner pipeline seams before cleanup.
Blocker: LLVM-PIPE-001
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - tools/hako_check/llvm_pipeline_inventory.py
  - tools/hako_check/llvm_pipeline_inventory_smoke.sh
---

# 296x-434 LLVM Runner Pipeline Debt Inventory

## Purpose

`MIM-FMEM-017D` named the replacement-front producer taxonomy. This row keeps
LLVM runner cleanup separate from that taxonomy by making the current runner
seams visible as a static `hako_check` report.

This row is report/check-only.

## Decision

```text
llvm_pipeline_inventory_v0=1
runner_behavior_change=0
llvm_execution_run=0
pyvm_execution_run=0
harness_execution_run=0
mock_fallback_execution_run=0
source_rewrite_executed=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

The current LLVM runner debt is classified as pipeline structure debt, not
replacement-front producer semantics:

```text
NYASH_REWRITE_FUTURE:
  forced by LLVM MirCompilerBox with an env restore guard
  consumed by MIR normalization as FutureNew/FutureSet/Await -> env.future

method_id_injector:
  runner stage is present
  pass is currently a retired compatibility stub
  mutation count is zero

joinir_experiment:
  runner hook is present
  real mutation is feature/env gated
  disabled or non-applicable paths return original MIR

PyVM:
  withdrawn from daily/product ownership
  still reachable only through SMOKES_USE_PYVM=1 diagnostic route

harness/mock:
  object output, ny_llvmc harness, and mock fallback remain visible executor
  seams
```

## Fields

```text
output_contract=hako-check-llvm-pipeline-inventory-v0
tool_surface=hako_check_llvm_pipeline_inventory
observation_only=1
rewrite_executed=0
source_rewrite_executed=0
benchmark_run_executed=0
behavior_change=0

mir_future_rewrite_forced=1
mir_future_rewrite_env_key=NYASH_REWRITE_FUTURE
mir_future_rewrite_env_restore_guard=1
mir_future_rewrite_consumed_by_normalize=1
mir_future_rewrite_route=FutureNew/FutureSet/Await->env.future

method_id_injector_stage_present=1
method_id_injector_called=1
method_id_injector_noop_stub=1
method_id_injector_mutation_count=0

joinir_experiment_hook_called=1
joinir_experiment_feature_gate=llvm-harness
joinir_experiment_env_gate=NYASH_JOINIR_EXPERIMENT+NYASH_JOINIR_LLVM_EXPERIMENT+NYASH_LLVM_USE_HARNESS
joinir_experiment_fallback_policy=original_mir
joinir_experiment_original_mir_fallback=1

pyvm_executor_stage_present=1
pyvm_reachable=1
pyvm_gate=SMOKES_USE_PYVM
pyvm_daily_route=0
pyvm_withdrawn_policy=diagnostic_only

llvm_obj_out_stage_present=1
llvm_harness_stage_present=1
llvm_harness_feature_gate=llvm-harness
llvm_harness_default_enabled=1
llvmlite_daily_owner=0

mock_fallback_stage_present=1
mock_fallback_reachable=1
mock_fallback_blocked_when_harness_explicit=1

execution_backend_order=pyvm,obj_out,ny_llvmc_exe,mock
execution_backend_runtime_sample=0
llvm_fallback_used=0
llvm_fallback_reason=static_inventory_only
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
summary=ok
```

## Acceptance

```bash
python3 -m py_compile tools/hako_check/llvm_pipeline_inventory.py
bash tools/hako_check/llvm_pipeline_inventory_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

- do not change LLVM runner behavior in this row
- do not replace `NYASH_REWRITE_FUTURE` with `CompileOptions` in this row
- do not remove `method_id_injector` in this row
- do not change `joinir_experiment` fallback behavior in this row
- do not remove PyVM diagnostic reachability in this row
- do not run PyVM, harness, object emit, or mock fallback from the inventory
- do not open product allocator activation

## Follow-Up

```text
LLVM-PIPE-002:
  add/normalize dynamic pipeline report fields for future rewrite route,
  JoinIR experiment, method-id mutation count, execution backend, fallback use,
  and fallback reason.

LLVM-PIPE-003:
  move env side effects and runner ad-hoc stages toward explicit
  CompileOptions / PipelinePlan / LoweringPlan objects.
```

## Landed Evidence

```text
mir_future_rewrite_forced=1
method_id_injector_mutation_count=0
joinir_experiment_fallback_policy=original_mir
pyvm_reachable=1
pyvm_daily_route=0
execution_backend_order=pyvm,obj_out,ny_llvmc_exe,mock
llvm_fallback_used=0
llvm_fallback_reason=static_inventory_only
runner_behavior_change=0
```

Next row:

```text
LLVM-PIPE-002 LLVM runner pipeline report fields
```
