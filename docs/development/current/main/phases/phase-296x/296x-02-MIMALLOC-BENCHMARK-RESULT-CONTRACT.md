---
Status: Landed
Date: 2026-05-27
Scope: define the stable benchmark result vocabulary before parsing external logs.
Blocker: MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-01-MIMALLOC-BENCHMARK-HAKMEM-ASSET-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-02 Mimalloc Benchmark Result Contract

## Decision

Close:

```text
MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001
```

Accept the shared benchmark result vocabulary for the `hakmem` corpus before
writing parsers or opening DLL/provider work.

The shared contract keeps the repeated-measurement fields stable across the
external corpus, the `.hako` exact-EXE runner, and the adapter outputs:

```text
benchmark_result_contract=hakmem-benchmark-result-v0
benchmark_profile=phase296x-repeated-v0
source_corpus=/home/tomoaki/git/hakmem_20260525_extracted/hakmem
workload_id
allocator_id
runner_kind
operation_family
operation_repeat
timing_repeat_kind=process-invocation-v0
sample_count
warmup_count
summary_statistic=min,median,max
canonical_rss_collector=external-time
internal_rss_evidence=preserved
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator_installed=0
winner_claim=0
```

The result contract is neutral about whether the input comes from the
historical `benchres.csv` catalog, a `hakozuna_compare` log, or the exact-EXE
comparison runner. The later rows will map adapter-specific fields into this
vocabulary.

## Selected Next

Select:

```text
MIMALLOC-BENCHMARK-HAKMEM-BENCHRES-ADAPTER-296X-001
```

The next row should import one selected `benchres.csv` family into the accepted
result contract without opening provider/DLL/replacement seams.

## Stop Line

This row does not write parsers yet, does not make winner claims, does not
activate provider/DLL/replacement/hook/global-allocator seams, and does not
change the default runtime behavior of the exact-EXE runner.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_benchmark_result_contract_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
