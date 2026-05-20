# Hako Alloc Allocator Comparison Benchmark Execution Preflight Closeout

Status: accepted
Decision: accepted
Scope: MIMAP-438A allocator comparison benchmark execution preflight closeout.

## Purpose

MIMAP-438A closes the benchmark execution preflight inventory and diagnostics
pack before a controlled benchmark execution plan can be selected.

The closeout proves that:

```text
MIMAP-436A benchmark execution preflight inventory
  -> MIMAP-437A benchmark execution preflight diagnostics
  -> benchmark execution preflight package is observable
```

## Included Rows

```text
MIMAP-436A allocator comparison benchmark execution preflight inventory
MIMAP-437A allocator comparison benchmark execution preflight diagnostics
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_closeout_guard.sh
```

The closeout re-runs the MIMAP-436A and MIMAP-437A L2 guards. It does not add
L3 benchmark evidence; actual benchmark execution belongs to a later explicit
row.
