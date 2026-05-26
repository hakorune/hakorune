---
Status: Landed
Date: 2026-05-27
Scope: inventory external hakmem benchmark assets and select the first adapter row.
Blocker: MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-01 Hakmem Asset Inventory

## Decision

Close:

```text
MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001
```

This row stays inventory-only. It classifies the external `hakmem` corpus,
selects the first adapter row, and keeps DLL/provider work closed.

External corpus root:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

## Inventory Result

### Selected adapter family

```text
MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001
```

### Read-only input paths used

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/mimalloc_bench_full_20260117_064626/benchres.csv
/home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/hakozuna_compare_20260118_034554/hakozuna_compare_20260118_034554_mimalloc_e165faccc.log
/home/tomoaki/git/hakmem_20260525_extracted/hakmem/PERF_INDEX.md
/home/tomoaki/git/hakmem_20260525_extracted/hakmem/HAKMEM_ARCHITECTURE_OVERVIEW.md
```

### Classified families

```text
source:
  bench_tiny_hot.c
  bench_random_mixed_hakmem
  system_bench_random_mixed
  run_benchmarks.sh

benchres_csv:
  bench_results/mimalloc_bench_full_20260105_182735/benchres.csv
  bench_results/mimalloc_bench_full_20260105_182746/benchres.csv
  bench_results/mimalloc_bench_full_20260105_183446/benchres.csv
  bench_results/mimalloc_bench_full_20260117_064153/benchres.csv
  bench_results/mimalloc_bench_full_20260117_064211/benchres.csv
  bench_results/mimalloc_bench_full_20260117_064626/benchres.csv

hakozuna_compare_log:
  bench_results/hakozuna_compare_20260105_182355/*.log
  bench_results/hakozuna_compare_20260105_182514/*.log
  bench_results/hakozuna_compare_20260105_182646/*.log
  bench_results/hakozuna_compare_20260118_034554/*.log

perf_strace:
  bench_results/s51_malloc_large_b40795031_20260105_105541/large_*.perf
  bench_results/s51_malloc_large_b40795031_20260105_105541/large_*.strace
  bench_results/s51_malloc_large_b40795031_20260105_110157/large_hz3_s51.strace

historical_report:
  PERF_INDEX.md
  HAKMEM_ARCHITECTURE_OVERVIEW.md
  WORKLOAD_COMPARISON_20251205.md
  PERF_ANALYSIS_RANDOM_MIXED_VS_TINY_HOT.md

parked_unknown:
  .git/**
  .claude/**
  allocators/*.so
  so/*.so
  out/**
```

## Selected Next

Select:

```text
MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001
```

The result contract should land before writing a `benchres.csv` or
`hakozuna_compare` parser.

## Mini-Agent Checklist

1. Run:

```bash
find /home/tomoaki/git/hakmem_20260525_extracted/hakmem -maxdepth 3 -type f
```

2. Do not edit the external corpus.
3. Update this card only.
4. Add or update one focused guard if the inventory becomes machine-checked.

## Stop Line

This row does not open provider/DLL/replacement/global allocator seams and does
not make winner claims.
