# Hakozuna Mixed Working-Set Fixture

This directory vendors the minimal `hakozuna` mixed working-set benchmark used
by the Ubuntu-side allocator comparison.

## Scope

This is not the full extracted `hakmem` / `hakozuna` corpus. The fixture keeps
only the source needed to build:

```text
bench_mixed_ws_crt
```

Copied files:

```text
bench_mixed_ws.c
LICENSE
NOTICE
```

Local fixture files:

```text
Makefile
hz3.h
README.md
THIRD_PARTY_NOTICES.md
```

The source was copied from:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem/hakozuna/hz3/bench/bench_mixed_ws.c
```

## Build

```sh
make -C benchmarks/external/hakozuna/mixed-ws
```

The executable is written to:

```text
benchmarks/external/hakozuna/mixed-ws/build/bench_mixed_ws_crt
```

## Smoke

```sh
benchmarks/external/hakozuna/mixed-ws/build/bench_mixed_ws_crt 1 1000 128 16 1024
```

Expected stdout contains:

```text
threads=<n> iters=<n> ws=<n> size=<min>..<max> time=<s> ops/s=<value>
```

## Allocator Compare

Use the repo-local compare tool to run this fixture under system malloc, C
mimalloc, and the benchmark-only Hakorune replacement front:

```sh
python3 tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py \
  --allow-ldconfig-discovery \
  --replacement-front-native-slot-mode \
  --replacement-front-thread-local-mode \
  --replacement-front-tls-counter-mode \
  --replacement-front-cross-thread-smoke \
  --replacement-front-match-workload-realloc-size \
  --out target/hakozuna-mixed-ws-replacement-smoke/report.out \
  --out-dir target/hakozuna-mixed-ws-replacement-smoke/artifacts \
  --sample-count 5
```

The Hakorune replacement front here is a benchmark-only fixed-slot subject.
It is useful for allocator-front development evidence, but it is not a product
allocator activation path and must not be used for a winner claim.

## Contract

The fixture is for external-process allocator replacement checks. It is built
with `HZ3_BENCH_USE_CRT=1`, so the benchmark calls libc
`malloc` / `realloc` / `free` and can be driven by Hakorune-generated
LD_PRELOAD shims.

Do not vendor the whole hakozuna corpus unless a later paper/reproducibility
decision explicitly selects additional benchmark families.
