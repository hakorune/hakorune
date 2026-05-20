# Hako Alloc Allocator Comparison Workload Matrix Closeout

Status: accepted
Decision: accepted
Scope: MIMAP-432A allocator comparison workload matrix closeout.

## Purpose

MIMAP-432A closes the allocator comparison workload matrix inventory and
diagnostics pack before benchmark execution or host allocator replacement can
be considered.

The closeout proves that:

```text
MIMAP-430A workload matrix inventory
  -> MIMAP-431A workload matrix diagnostics
  -> comparison workload matrix package is observable
```

## Included Rows

```text
MIMAP-430A allocator comparison workload matrix inventory
MIMAP-431A allocator comparison workload matrix diagnostics
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_closeout_guard.sh
```

The closeout re-runs the MIMAP-430A and MIMAP-431A L2 guards. It does not add
L3 benchmark evidence; benchmark execution belongs to a later explicit row.
