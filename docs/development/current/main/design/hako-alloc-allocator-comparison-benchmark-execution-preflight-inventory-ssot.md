# Hako Alloc Allocator Comparison Benchmark Execution Preflight Inventory

Status: accepted
Decision: accepted
Scope: MIMAP-436A allocator comparison benchmark execution preflight inventory.

## Purpose

MIMAP-436A records the explicit preflight inputs that must be true before an
allocator comparison benchmark execution row can be opened.

This row does not run a benchmark. It proves that benchmark execution can be
made conditional on a visible preflight ledger instead of implicit environment
or process-global state.

## Inventory Inputs

```text
benchmark_runner_selected
output_capture_present
measurement_storage_present
workload_matrix_ready
measurement_plan_ready
process_replacement_closed
hook_install_closed
backend_matcher_closed
global_allocator_closed
hidden_env_closed
```

## Reject Reasons

```text
0 accepted
1 missing benchmark runner
2 missing output capture
3 missing measurement storage
4 workload matrix not ready
5 measurement plan not ready
6 process replacement is open
7 hook installation is open
8 backend matcher additions are open
9 global allocator installation is open
10 hidden env or implicit discovery is open
```

## Still Closed

```text
benchmark execution
hook installation
backend matcher additions
process allocator replacement
#[global_allocator]
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_inventory_guard.sh --level L2
```

L3 benchmark evidence belongs to a later explicit benchmark-execution row.
