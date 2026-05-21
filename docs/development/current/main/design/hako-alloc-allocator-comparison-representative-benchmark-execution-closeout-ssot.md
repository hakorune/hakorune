# Hako Alloc Allocator Comparison Representative Benchmark Execution Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Scope: MIMAP-446A allocator comparison representative benchmark execution closeout.

## Purpose

MIMAP-446A closes the representative benchmark execution pilot and diagnostics
pack before opening any C mimalloc comparison execution row.

The closeout proves that:

```text
MIMAP-444A representative benchmark execution pilot
  -> MIMAP-445A representative benchmark execution diagnostics
  -> representative benchmark execution package is observable
```

## Included Rows

```text
MIMAP-444A allocator comparison representative benchmark execution pilot
MIMAP-445A allocator comparison representative benchmark execution diagnostics
```

## Still Closed

```text
C mimalloc execution
hook installation
backend matcher additions
process allocator replacement
#[global_allocator]
worker/TLS or thread execution
hidden env / implicit discovery / process-global activation config
```

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_representative_benchmark_execution_closeout_guard.sh
```

The closeout re-runs the MIMAP-444A and MIMAP-445A L2 guards. It does not add
C mimalloc execution evidence; that belongs to a later explicit row.
