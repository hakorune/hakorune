# Hako Alloc Allocator Comparison Controlled Benchmark Execution Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Scope: MIMAP-442A allocator comparison controlled benchmark execution closeout.

## Purpose

MIMAP-442A closes the controlled benchmark execution inventory and diagnostics
pack before any representative benchmark execution row is opened.

The closeout proves that:

```text
MIMAP-440A controlled benchmark execution inventory
  -> MIMAP-441A controlled benchmark execution diagnostics
  -> controlled benchmark execution package is observable
```

## Included Rows

```text
MIMAP-440A allocator comparison controlled benchmark execution inventory
MIMAP-441A allocator comparison controlled benchmark execution diagnostics
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_controlled_benchmark_execution_closeout_guard.sh
```

The closeout re-runs the MIMAP-440A and MIMAP-441A L2 guards. It does not add
L3 benchmark evidence; actual benchmark execution belongs to a later explicit
row.
