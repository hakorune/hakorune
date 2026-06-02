# Hakmem Mid/Large MT System Fixture

This directory vendors the minimal `hakmem` mid/large multi-thread benchmark
for Hakorune LD_PRELOAD allocator replacement checks.

## Scope

This is not the full extracted `hakmem` corpus. The original corpus is about
17GB and includes many unrelated historical artifacts. This fixture keeps only
the system-malloc benchmark source required to build:

```text
bench_mid_large_mt_system
```

Copied files:

```text
bench_mid_large_mt.c
core/bench_profile.h
LICENSE
NOTICE
THIRD_PARTY_NOTICES.md
```

The source was copied from:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```

## Build

```sh
make -C benchmarks/external/hakmem/mid-large-mt-system
```

The executable is written to:

```text
benchmarks/external/hakmem/mid-large-mt-system/build/bench_mid_large_mt_system
```

## Smoke

```sh
benchmarks/external/hakmem/mid-large-mt-system/build/bench_mid_large_mt_system 2 1000 128 42
```

Expected stdout contains:

```text
Throughput = <ops/s> ops/s [t=<n> iter=<n> ws=<n>] time=<s>s
```

## Contract

The fixture is for external-process allocator replacement only. It builds
without `USE_HAKMEM`, so the benchmark uses libc `malloc` / `free` and can be
driven by Hakorune-generated LD_PRELOAD shims.

Do not vendor the whole corpus unless a later paper/reproducibility decision
requires historical result archives or additional benchmark families.
