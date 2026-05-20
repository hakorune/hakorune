# Hako Alloc Allocator Comparison Workload Matrix Inventory

Status: accepted
Decision: accepted
Scope: MIMAP-430A allocator comparison workload matrix inventory.

## Purpose

MIMAP-430A inventories the workload families required before comparing
`.hako` / `hako_alloc` against C mimalloc for throughput and memory usage.
It does not run benchmarks and does not replace the process allocator.

## Input Contract

Accepted inventory requires:

```text
small_allocation_workload_present == 1
small_free_workload_present == 1
realloc_workload_present == 1
huge_allocation_workload_present == 1
throughput_workload_present == 1
memory_usage_workload_present == 1
workload_family_count >= 1
```

## Reject Reasons

```text
1 missing small allocation workload
2 missing small free workload
3 missing realloc workload
4 missing huge allocation workload
5 missing throughput workload
6 missing memory-usage workload
7 invalid workload family count
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
