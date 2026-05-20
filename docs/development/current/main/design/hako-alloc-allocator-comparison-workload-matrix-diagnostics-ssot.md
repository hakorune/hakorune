# Hako Alloc Allocator Comparison Workload Matrix Diagnostics

Status: accepted
Decision: accepted
Scope: MIMAP-431A allocator comparison workload matrix diagnostics.

## Purpose

MIMAP-431A consumes MIMAP-430A allocator comparison workload matrix inventory
reports and classifies why the comparison workload matrix is still blocked.
The row is diagnostic-only: it does not run benchmarks, install hooks, add
backend matchers, replace the process allocator, or install a global allocator.

## Input Contract

The owner consumes:

```text
HakoAllocAllocatorComparisonWorkloadMatrixInventoryReport
```

The diagnostic maps MIMAP-430A reasons directly:

```text
1 missing small allocation workload
2 missing small free workload
3 missing realloc workload
4 missing huge allocation workload
5 missing throughput workload
6 missing memory-usage workload
7 invalid workload family count
8 closed seam leak
```

## Report Contract

Accepted-ready input produces:

```text
diagnostic_present = 1
workload_matrix_inventory_present = 1
workload_matrix_ready = 1
blocked_workload_present = 0
reason = 0
```

Blocked input produces exactly one blocked-workload flag for the reason
family.

Execution fields remain closed:

```text
benchmark_executed = 0
process_replacement_executed = 0
hook_installed = 0
backend_matcher_added = 0
global_allocator_installed = 0
would_run_benchmark = 0
would_replace_host_allocator = 0
would_install_hook = 0
would_add_backend_matcher = 0
would_run_thread = 0
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
validation_profile = scalar-mir
exe = deferred-to-comparison-workload-matrix-closeout
```
