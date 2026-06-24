---
Status: Landed
Date: 2026-05-27
Scope: close the external hakmem corpus adapter bring-up before real exact-EXE measurement.
Blocker: MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-03-MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER.md
  - docs/development/current/main/phases/phase-296x/296x-04-MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER.md
  - docs/development/current/main/phases/phase-296x/296x-05-MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-06 External Corpus Closeout

## Decision

Close:

```text
MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001
```

The phase-296x external corpus bring-up now has the minimum stable bridges
needed before real exact-EXE measurement:

```text
benchres_adapter=accepted
hakozuna_compare_adapter=accepted
exact_exe_harness_pilot=accepted
winner_claim=0
provider_active=0
replacement_active=0
global_allocator=0
```

The external `hakmem` corpus remains read-only input. Historical rows are
schema evidence only; current benchmark evidence comes from the exact-EXE
runner.

## Selected Next

Select:

```text
MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT-296X-001
```

The next row should run the same selected workload with process-repeat timing:

```text
workload=representative-small-block-v0
sample_count=3
warmup_count=1
operation_repeat=128
timing_repeat_kind=process-invocation-v0
```

## Stop Line

This row does not compute winners, does not run DLL/provider work, and does
not open replacement/hook/global-allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_benchmark_external_corpus_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
