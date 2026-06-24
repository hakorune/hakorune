---
Status: Landed
Date: 2026-05-27
Scope: run one already-landed .hako workload through the shared benchmark harness using the accepted result contract.
Blocker: MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-04-MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
---

# 296x-05 Exact-EXE Harness Pilot

## Decision

Close:

```text
MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001
```

Run one already-landed `.hako` workload through the shared repeated
measurement harness with the accepted benchmark result contract. This keeps
the exact-EXE comparison lane anchored on the same workload identity while the
historical `hakmem` adapters stay read-only.

The shared harness is the existing bridge:

```text
tools/allocator/mimalloc_repeated_measurement_runner.py
```

The pilot keeps the repeated comparison output contract intact:

```text
output_contract=mimalloc-comparison-repeated-measurement-v0
measurement_profile=phase295x-repeated-v0
sample_count
warmup_count
operation_repeat
timing_repeat_kind=process-invocation-v0
summary_statistic=min,median,max
canonical_rss_collector=external-time
internal_rss_evidence=preserved
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
```

Representative pilot workload:

```text
representative-small-block-v0
```

## Selected Next

Select:

```text
MIMALLOC-BENCHMARK-EXTERNAL-CORPUS-CLOSEOUT-296X-001
```

The next row should close the external corpus adapter bring-up before any
load-only DLL/provider selection work opens.

## Stop Line

This row does not compute winners, does not open provider/DLL/replacement/
hook/global-allocator seams, and does not introduce new workload identities.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_benchmark_exact_exe_harness_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
