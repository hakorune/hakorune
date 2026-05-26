---
Status: Landed
Date: 2026-05-27
Scope: parse one selected benchres.csv corpus family into the accepted result contract.
Blocker: MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-02-MIMALLOC-BENCHMARK-RESULT-CONTRACT.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - tools/allocator/hakmem_benchres_adapter.py
---

# 296x-03 Hakmem Benchres Adapter

## Decision

Close:

```text
MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001
```

Import one selected `benchres.csv` family into the accepted benchmark result
contract without opening provider/DLL/replacement seams.

The adapter is the existing narrow bridge:

```text
tools/allocator/hakmem_benchres_adapter.py
```

It reads whitespace-delimited `mimalloc-bench` `benchres.csv` rows and emits
historical benchmark evidence with:

```text
output_contract=hakmem-external-benchres-adapter-v0
dataset_role=external-historical-benchmark-corpus
winner_claim=0
```

Allocator aliases are normalized as:

```text
mi -> mimalloc
tc -> tcmalloc
sys -> system
```

and the adapter publishes:

```text
elapsed_ms
peak_rss_bytes
user_sec
sys_sec
major_faults
minor_faults
```

## Selected Next

Select:

```text
MIMALLOC-BENCHMARK-HAKOZUNA-COMPARE-LOG-ADAPTER-296X-001
```

The next row should import one `hakozuna_compare` family into the same result
contract without opening provider/DLL/replacement seams.

## Stop Line

This row does not import historical rows as repeated measurement evidence,
does not compute winners, and does not open provider/DLL/replacement/hook/
global-allocator seams.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_benchmark_hakmem_benchres_adapter_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
