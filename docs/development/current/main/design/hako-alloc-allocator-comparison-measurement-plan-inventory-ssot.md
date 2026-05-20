# Hako Alloc Allocator Comparison Measurement Plan Inventory

Status: accepted
Decision: accepted
Scope: MIMAP-433A allocator comparison measurement plan inventory.

## Purpose

MIMAP-433A records the explicit measurement plan inputs required before
`.hako` / `hako_alloc` can be compared against C mimalloc for throughput and
memory usage.

This row is still model-only. It records readiness for benchmark measurement,
but does not run a benchmark and does not open process allocator replacement.

## Inventory Inputs

```text
run_count_present
warmup_plan_present
output_contract_present
throughput_measurement_present
memory_usage_measurement_present
run_count
warmup_count
```

## Reject Reasons

```text
0 accepted
1 missing run-count plan
2 missing warmup plan
3 missing output contract
4 missing throughput measurement plan
5 missing memory-usage measurement plan
6 invalid run count
7 invalid warmup count
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_measurement_plan_inventory_guard.sh --level L2
```

L3 benchmark evidence belongs to a later explicit benchmark-execution row.
