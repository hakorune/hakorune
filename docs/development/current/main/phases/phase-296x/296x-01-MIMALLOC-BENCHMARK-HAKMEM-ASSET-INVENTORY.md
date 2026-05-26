---
Status: Current
Date: 2026-05-27
Scope: inventory external hakmem benchmark assets and select the first adapter row.
Blocker: MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-01 Hakmem Asset Inventory

## Decision

Keep this row as an inventory row. It should not parse all logs and should not
open DLL/provider work.

External corpus root:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

## Inventory Targets

Classify these asset families:

```text
bench_tiny_hot.c
bench_random_mixed_hakmem
system_bench_random_mixed
run_benchmarks.sh
bench_results/**/benchres.csv
bench_results/**/hakozuna_compare_*.log
bench_results/**/large_*.perf
bench_results/**/large_*.strace
PERF_INDEX.md
HAKMEM_ARCHITECTURE_OVERVIEW.md
```

## Expected Output

The completed row should record:

- selected first adapter family;
- rejected or parked artifact families;
- exact corpus paths used as read-only input;
- one next row token.

Recommended first adapter:

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

2. Group files into:

```text
source
binary
script
benchres_csv
hakozuna_compare_log
perf_strace
historical_report
parked_unknown
```

3. Do not edit the external corpus.
4. Update this card only.
5. Add or update one focused guard if the inventory becomes machine-checked.

## Stop Line

This row does not open provider/DLL/replacement/global allocator seams and does
not make winner claims.
