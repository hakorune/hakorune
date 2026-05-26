---
Status: Landed
Date: 2026-05-27
Scope: run a real repeated exact-EXE comparison for the selected same workload.
Blocker: MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-05-MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-06-MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
---

# 296x-07 Exact-EXE Repeated Measurement

## Decision

Close:

```text
MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001
```

Run the selected same workload through the exact-EXE repeated-measurement
harness with process-repeat timing:

```text
workload=representative-small-block-v0
operation_family=small-block
sample_count=3
warmup_count=1
operation_repeat=128
timing_repeat_kind=process-invocation-v0
summary_statistic=min,median,max
winner_claim=0
```

## Evidence

Representative guard evidence:

```text
representative-small-block-v0:
  hako_external_elapsed_min_ms=70
  hako_external_elapsed_median_ms=70
  hako_external_elapsed_max_ms=80
  c_external_elapsed_min_ms=70
  c_external_elapsed_median_ms=70
  c_external_elapsed_max_ms=70
  hako_external_rss_median_bytes=3641344
  c_external_rss_median_bytes=3985408
```

The row records measurement evidence only. It does not compute a speed or RSS
winner.

## Selected Next

Select:

```text
MIMALLOC-DLL-LOAD-ONLY-SELECTION-296X-001
```

The next row may select a load-only DLL metadata smoke. Provider activation,
allocator replacement, hooks, and global allocator work remain closed until a
separate later row explicitly opens them.

## Stop Line

This row does not compute winners, require timing/RSS parity, open body
timing, or activate provider/DLL/replacement/hook/global-allocator behavior.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_benchmark_exact_exe_repeated_measurement_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
