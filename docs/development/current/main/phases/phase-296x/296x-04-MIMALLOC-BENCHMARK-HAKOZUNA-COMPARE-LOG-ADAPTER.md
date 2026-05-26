---
Status: Landed
Date: 2026-05-27
Scope: import one selected hakozuna_compare log family into the accepted result contract.
Blocker: MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-03-MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - tools/allocator/hakmem_hakozuna_compare_log_adapter.py
---

# 296x-04 Hakmem Hakozuna Compare Log Adapter

## Decision

Close:

```text
MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001
```

Import one selected `hakozuna_compare_*.log` family into the accepted
benchmark result contract without opening provider/DLL/replacement seams.

The adapter is the existing narrow bridge:

```text
tools/allocator/hakmem_hakozuna_compare_log_adapter.py
```

It reads historical `hakozuna_compare` logs and emits:

```text
output_contract=hakmem-external-hakozuna-compare-log-adapter-v0
dataset_role=external-historical-benchmark-corpus
winner_claim=0
```

The adapter extracts the run-level throughput / elapsed / peak RSS summary
fields and preserves the first run rows for schema inspection.

## Selected Next

Select:

```text
MIMALLOC-BENCHMARK-EXACT-EXE-HARNESS-PILOT-296X-001
```

The next row should run one already-landed `.hako` workload through the shared
benchmark harness and keep winner claims closed.

## Stop Line

This row does not import historical logs as current repeated-measurement
evidence, does not compute winners, and does not open provider/DLL/
replacement/hook/global-allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_benchmark_hakozuna_compare_log_adapter_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
