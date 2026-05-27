---
Status: Landed
Date: 2026-05-27
Scope: run baseline repeated measurements for the first parity workload with winner claims closed.
Blocker: HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-43-HAKO-MIMALLOC-PERF-PARITY-WORKLOAD-MATRIX.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
---

# 296x-44 Hako Mimalloc Performance Parity Baseline Pack

## Decision

Close:

```text
HAKO-MIMALLOC-PERF-PARITY-BASELINE-PACK-296X-001
```

Run the first same-workload baseline pack with the accepted repeated
measurement policy:

```text
same_workload=1
same_operation_count=1
sample_count=3
warmup_count=1
operation_repeat=128
winner_claim=0
```

The active baseline pair is:

```text
hako_mimalloc_exact_exe
c_mimalloc_explicit_runner
```

The `hakozuna_reference` and `provider_package_hako_mimalloc_explicit`
subjects remain parked as reference-only definitions for later rows.

## Evidence

The baseline pack runner produced:

```text
output_contract=mimalloc-comparison-repeated-measurement-v0
measurement_profile=phase295x-repeated-v0
workload_count=1
workloads=representative-small-block-v0
sample_count=3
warmup_count=1
operation_repeat=128
timing_repeat_kind=process-invocation-v0
hako_runtime_config_profile=empty
hako_selected_loadset=empty
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
summary=ok
```

Sample medians from the run:

```text
workload_0_hako_external_elapsed_min_ms=80
workload_0_hako_external_elapsed_median_ms=90
workload_0_hako_external_elapsed_max_ms=90
workload_0_c_external_elapsed_min_ms=70
workload_0_c_external_elapsed_median_ms=70
workload_0_c_external_elapsed_max_ms=70
workload_0_hako_external_rss_median_bytes=3584000
workload_0_c_external_rss_median_bytes=3985408
```

The row records baseline evidence only. It does not compute a winner or open
provider/replacement/hook/global-allocator seams.

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-GAP-TAXONOMY-ADAPTER-296X-001
```

The next row should classify the measured gap by primary owner before any
optimization work starts.

## Stop Line

This row does not compare winners, activate providers, replace the process
allocator, install hooks, or claim the benchmark result is already parity.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_perf_parity_baseline_pack_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
