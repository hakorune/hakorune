# Hako Alloc Allocator Comparison Baseline Closeout

Status: accepted
Decision: accepted
Scope: MIMAP-429A allocator comparison baseline closeout.

## Purpose

MIMAP-429A closes the allocator comparison baseline inventory and diagnostics
pack before benchmark execution or host allocator replacement can be considered.

The closeout proves that:

```text
MIMAP-427A inventory
  -> MIMAP-428A diagnostics
  -> comparison baseline package is observable
```

## Included Rows

```text
MIMAP-427A allocator comparison baseline inventory
MIMAP-428A allocator comparison baseline diagnostics
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_baseline_closeout_guard.sh
```

The closeout re-runs the MIMAP-427A and MIMAP-428A L2 guards. It does not add
L3 benchmark evidence; benchmark execution belongs to a later explicit row.
