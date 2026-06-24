---
Status: Landed
Date: 2026-05-27
Scope: classify the row 44 scout baseline gap before any optimization work starts.
Blocker: HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-44-HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
---

# 296x-45 Hako Mimalloc Performance Gap Taxonomy Adapter

## Purpose

Classify the row 44 scout baseline before optimization. This row does not prove
a winner, widen the benchmark contract unnecessarily, or optimize `.hako`
mimalloc.

## Input

Use a row 44 repeated measurement report with:

```text
output_contract=mimalloc-comparison-repeated-measurement-v0
workloads=representative-small-block-v0
sample_count=3
warmup_count=1
operation_repeat=128
winner_claim=0
```

## Output Contract

Add an adapter that emits:

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
workload_id
measurement_profile
hako_subject=hako_mimalloc_exact_exe
c_subject=c_mimalloc_explicit_runner
hako_elapsed_min_ms
hako_elapsed_median_ms
hako_elapsed_max_ms
c_elapsed_min_ms
c_elapsed_median_ms
c_elapsed_max_ms
elapsed_median_gap_ms
elapsed_median_ratio
hako_rss_median_bytes
c_rss_median_bytes
rss_median_gap_bytes
hako_max_to_median_ratio
c_max_to_median_ratio
outlier_observed=0|1
evidence_quality=stable|noisy
gap_owner=<one primary owner>
gap_confidence=low|medium|high
next_diagnostic
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Owner Rules

Allowed `gap_owner` values:

```text
allocator_algorithm
compiler_lowering
hako_runtime_baseline
c_abi_memory_bridge
osvm_page_source
provider_wrapper
benchmark_harness
```

Use `benchmark_harness` when max/median ratios or sample instability dominate
the evidence. Use `hako_runtime_baseline` for fixed exact-EXE/runtime cost.
Use `compiler_lowering` for generated code shape costs. Use
`allocator_algorithm` only when the gap scales with allocator operations.

## Evidence

Implemented:

```text
tools/allocator/hako_mimalloc_gap_taxonomy_adapter.py
```

The adapter reads:

```text
output_contract=mimalloc-comparison-repeated-measurement-v0
```

and emits:

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
```

The row guard verifies both:

```text
live row 44 baseline report:
  same workload / sample policy / stop line preserved

synthetic outlier report:
  outlier_observed=1
  evidence_quality=noisy
  gap_owner=benchmark_harness
  gap_confidence=medium
  next_diagnostic=measurement_hygiene_refresh
```

The synthetic report keeps C-side outlier handling stable without requiring
every guard run to reproduce an operating-system scheduling spike.

## First Classification Policy

Given the row 44 rerun observed a C-side max outlier, the first accepted result
may be:

```text
outlier_observed=1
evidence_quality=noisy
gap_owner=benchmark_harness
gap_confidence=medium
next_diagnostic=measurement_hygiene_refresh
```

This is a valid row 45 result. It should select a measurement hygiene row, not
an optimization row.

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-CONDITIONAL-DIAGNOSTIC-SELECTION-296X-001
```

The next row should choose measurement hygiene only if row 45 evidence is noisy
or harness-owned. Otherwise it should select the owner-specific narrow
diagnostic.

## Stop Line

Do not change sample policy globally, make `body_elapsed_ns` primary, add CPU
pinning, activate providers, replace the allocator, install hooks, or claim a
winner in this row.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_gap_taxonomy_adapter_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

The guard proves:

```text
same_workload=1
same_operation_count=1
same_sample_policy=1
gap_owner is one allowed value
gap_confidence is present
next_diagnostic is present
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
