# Hako Alloc Allocator Comparison Measurement Plan Diagnostics

Status: accepted
Decision: accepted
Scope: MIMAP-434A allocator comparison measurement plan diagnostics.

## Purpose

MIMAP-434A consumes the MIMAP-433A measurement-plan inventory report and
publishes observer-only diagnostics for missing or invalid measurement inputs.

The row keeps benchmark execution closed. It only classifies whether the
measurement plan is ready for a later benchmark-execution row.

## Diagnostic Reasons

```text
0 accepted
1 missing run-count plan
2 missing warmup plan
3 missing output contract
4 missing throughput measurement plan
5 missing memory-usage measurement plan
6 invalid run count
7 invalid warmup count
8 closed execution seam was observed
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_diagnostics_guard.sh --level L2
```

L3 benchmark evidence belongs to a later explicit benchmark-execution row.
