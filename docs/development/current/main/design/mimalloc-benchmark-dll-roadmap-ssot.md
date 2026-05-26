---
Status: SSOT
Date: 2026-05-27
Scope: benchmark and DLL/provider sequencing after the first mimalloc port pass.
Related:
  - docs/development/current/main/phases/phase-296x/README.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md
---

# Mimalloc Benchmark / DLL Roadmap

## Decision

After the first `.hako` mimalloc port pass, the next lane is benchmark
contract work, not provider activation.

Open first:

- benchmark asset inventory;
- workload identity and equivalence maps;
- result format adapters for external `hakmem` artifacts;
- exact-EXE benchmark harness rows that keep winner claims closed.

Keep closed until the benchmark contract is stable:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- speed or memory winner claims.

## External Corpus

The current external benchmark corpus root is:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

Initial inventory should classify these artifacts:

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

The corpus is read-only input for this repo. Do not move or rewrite files under
the external path from Hakorune rows.

## Benchmark Contract Order

1. Inventory the external corpus and select the first adapters.
2. Define the benchmark result contract:
   - `benchmark_result_contract`
   - `benchmark_profile`
   - `workload_id`
   - `allocator_id`
   - `runner_kind`
   - `operation_family`
   - `operation_repeat`
   - `timing_repeat_kind`
   - `sample_count`
   - `warmup_count`
   - `summary_statistic=min,median,max`
   - `canonical_rss_collector=external-time`
   - `internal_rss_evidence=preserved`
   - stop-line fields for provider/replacement/hook/global allocator.

Current row token:

```text
MIMALLOC-BENCHMARK-RESULT-CONTRACT-296X-001
```

3. Add adapter rows:
   - `benchres.csv` adapter;
   - `hakozuna_compare` log adapter;
   - selected `perf/strace` observation adapter only after the first two are
     stable.
4. Add exact-EXE benchmark harness rows that reuse already-landed workload
   ids before opening native replacement or DLL work.
5. Run one selected same workload with repeated process timing:
   - `sample_count=3`
   - `warmup_count=1`
   - `operation_repeat=128`
   - `winner_claim=0`
6. Only after the above is stable, select DLL/provider load-only work.

The accepted result contract is intentionally compatible with the existing
exact-EXE repeated-measurement runner and with the external `hakmem`
historical adapters. The first adapter rows are historical bridges, not winner
claims or provider activations.

## Provider Package Timing

Provider package load work starts only after all of the following are true:

```text
benchmark_result_contract=accepted
hakmem_benchres_adapter=accepted
hakozuna_compare_adapter=accepted
exact_exe_benchmark_harness=accepted
exact_exe_repeated_measurement=accepted
winner_claim=0
provider_active=0
replacement_active=0
```

The first provider package row must be load-only:

```text
dll_mode=load-only
provider_active=0
replacement_active=0
global_allocator=0
winner_claim=0
```

The first load-only step is metadata preflight, not shared-library loading:

```text
dll_mode=metadata-preflight
shared_library_load_executed=0
provider_active=0
replacement_active=0
global_allocator=0
winner_claim=0
```

The ladder is:

1. metadata preflight over manifest / descriptor / hash;
2. shared-library-load-only smoke with no export resolution;
3. descriptor-read smoke with descriptor export only;
4. provider API bind smoke;
5. explicit provider call smoke;
6. repeated benchmark through explicit provider;
7. replacement / hook / `#[global_allocator]` only after a separate decision
   row accepts the risk.

## Mini-Agent Rules

Small models should take exactly one slice:

- one row = one commit;
- no provider/DLL/replacement activation unless the row name explicitly says
  load-only or provider-call;
- no winner claim;
- no edits under `/home/tomoaki/git/hakmem_20260525_extracted/hakmem`;
- run the row guard, `bash tools/checks/current_state_pointer_guard.sh`, and
  `git diff --check`.
