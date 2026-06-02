# Allocator Comparison Tools

This directory contains small phase-295x comparison helpers. They are local
evidence tools, not allocator-provider activation paths.

## Mimalloc Direct-Exact Evidence

Use the direct-exact wrappers when investigating current `.hako` mimalloc
parity. They source `tools/allocator/mimalloc_direct_exact_env.sh` so worker
runs do not accidentally measure the default/safe front.

```bash
tools/allocator/hako_mimalloc_direct_exact_app_perf_stat.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-public.stat.txt \
  --runs 5
```

For owner-first assembly evidence, use the perf/asm wrapper. It keeps the built
EXE, `perf.data`, annotate output, and objdump next to the report.

```bash
tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh \
  --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako \
  --out target/mimalloc-public.asm.txt \
  --symbol ny_main
```

## Hakmem External Bench Bridge

Use `hakmem_external_bench.py` to run selected benchmarks from the extracted
`hakmem_20260525` corpus while keeping copied binaries and mutable output under
`target/`.

Default source:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

Default target:

```text
target/hakmem-bench/
```

List the supported local bridge inputs:

```bash
tools/allocator/hakmem_external_bench.py --list
```

Prepare the target-local executable copy without running benchmarks:

```bash
tools/allocator/hakmem_external_bench.py --prepare-only
```

Run a small smoke benchmark:

```bash
tools/allocator/hakmem_external_bench.py \
  --bench cfrac \
  --allocator sys \
  --allocator mimalloc \
  --out target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

Mutable output:

```text
target/hakmem-bench/out/bench/benchres.csv
```

Snapshot output:

```text
target/hakmem-bench/results/*.benchres.csv
```

### Minimal LD_PRELOAD Fixture

For daily LD_PRELOAD allocator replacement checks, use the repo-local minimal
random-mixed fixture instead of the full extracted corpus:

```bash
make -C benchmarks/external/hakmem/random-mixed-system
```

The LD_PRELOAD pilot tools default to:

```text
benchmarks/external/hakmem/random-mixed-system/build/bench_random_mixed_system
```

Pass `--hakmem-root /path/to/hakmem` only when intentionally running against the
full extracted corpus.

Run the current no-product-default provider replacement decision ladder:

```bash
tools/allocator/hako_mimalloc_provider_replacement_decision_ladder.sh \
  --out target/provider-replacement-decision/report.out \
  --skip-build-release
```

This consumes Hako/C repeated evidence, provider explicit evidence, repeated
repo-local hakmem LD_PRELOAD evidence, and the generated Rust global allocator
smoke. It records readiness only; product allocator replacement, production
hooks, production `#[global_allocator]`, and winner claims stay closed.

To intentionally compare against a full extracted corpus build, pass:

```bash
--hakmem-root /home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

Compare two decision reports without changing product defaults:

```bash
python3 tools/allocator/provider_replacement_decision_pair_compare.py \
  --left target/provider-replacement-decision-s5/report.out \
  --right target/provider-replacement-decision-external-s5/report.out \
  --out target/provider-replacement-decision-pair/report.out
```

## Stop Lines

- Do not commit copied benchmark executables or generated `benchres.csv`.
- Do not import historical `hakmem` CSV/log rows as current phase repeated
  measurement evidence without a schema-adapter row.
- Do not claim speed or RSS winners from this bridge.
- Do not use this bridge to open provider activation, process replacement,
  hooks, backend matchers, or `#[global_allocator]`.

The bridge emits `winner_claim=0` and the provider/replacement stop-line fields
so downstream scripts can keep the boundary explicit.

## Hakmem Result Adapters

Convert a `mimalloc-bench` `benchres.csv` file into key-value evidence:

```bash
tools/allocator/hakmem_benchres_adapter.py \
  --in target/hakmem-bench/results/cfrac_sys_mimalloc.benchres.csv
```

Convert a `hakozuna_compare_*.log` file into key-value evidence:

```bash
tools/allocator/hakmem_hakozuna_compare_log_adapter.py \
  --in /home/tomoaki/git/hakmem_20260525_extracted/hakmem/bench_results/hakozuna_compare_20260118_034633/hakozuna_compare_20260118_034633_mimalloc_e165faccc.log
```

Both adapters emit external historical corpus evidence only. They are useful for
schema alignment and workload selection, not for phase-295x winner claims.
