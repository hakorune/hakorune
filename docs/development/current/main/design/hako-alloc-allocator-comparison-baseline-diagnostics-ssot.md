# Hako Alloc Allocator Comparison Baseline Diagnostics

Status: accepted
Decision: accepted
Scope: MIMAP-428A allocator comparison baseline diagnostics.

## Purpose

MIMAP-428A consumes MIMAP-427A allocator comparison baseline inventory
reports and classifies why allocator comparison is still blocked. The row is
diagnostic-only: it does not run benchmarks, install hooks, add backend
matchers, replace the process allocator, or install a global allocator.

## Input Contract

The owner consumes:

```text
HakoAllocAllocatorComparisonBaselineInventoryReport
```

The diagnostic maps MIMAP-427A reasons directly:

```text
1 missing C mimalloc baseline
2 missing hako_alloc baseline
3 missing throughput target
4 missing memory-usage target
5 missing workload matrix
6 invalid repeat count
7 closed seam leak
```

## Report Contract

Accepted-ready input produces:

```text
diagnostic_present = 1
baseline_inventory_present = 1
comparison_ready = 1
blocked_input_present = 0
reason = 0
```

Blocked input produces exactly one blocked-input flag for the reason family.

Execution fields remain closed:

```text
process_replacement_executed = 0
hook_installed = 0
backend_matcher_added = 0
global_allocator_installed = 0
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
exe = deferred-to-comparison-baseline-closeout
```
