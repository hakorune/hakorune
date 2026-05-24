# Allocator Comparison Tools

This directory contains small phase-295x comparison helpers. They are local
evidence tools, not allocator-provider activation paths.

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
