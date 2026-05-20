# Hako Alloc Allocator Comparison Baseline Inventory

Status: accepted
Decision: accepted
Scope: MIMAP-427A allocator comparison baseline inventory.

## Purpose

MIMAP-427A inventories the explicit inputs required before judging
`.hako` / `hako_alloc` against C mimalloc for throughput and memory usage.
It does not run benchmarks and does not replace the process allocator.

## Input Contract

Accepted inventory requires:

```text
c_mimalloc_baseline_present == 1
hako_alloc_baseline_present == 1
throughput_target_present == 1
memory_usage_target_present == 1
workload_matrix_present == 1
repeat_count >= 1
```

## Reject Reasons

```text
1 missing C mimalloc baseline
2 missing hako_alloc baseline
3 missing throughput target
4 missing memory-usage target
5 missing workload matrix
6 invalid repeat count
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
