# Hakmem Tiny Hot System Fixture

This directory vendors the minimal `hakmem` tiny hot-path benchmark for
Hakorune LD_PRELOAD allocator replacement checks.

## Scope

This is not the full extracted `hakmem` corpus. The original corpus is about
17GB and includes many unrelated historical artifacts. This fixture keeps only
the system-malloc benchmark source required to build:

```text
bench_tiny_hot_system
```

Copied files:

```text
bench_tiny_hot.c
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
make -C benchmarks/external/hakmem/tiny-hot-system
```

The executable is written to:

```text
benchmarks/external/hakmem/tiny-hot-system/build/bench_tiny_hot_system
```

## Smoke

```sh
benchmarks/external/hakmem/tiny-hot-system/build/bench_tiny_hot_system 64 100 1000
```

Expected stdout contains:

```text
Throughput = <ops/s> ops/s [size=<n> batch=<n> cycles=<n>] time=<s>s
```

## Contract

The fixture is for external-process allocator replacement only. It builds
without `USE_HAKMEM`, so the benchmark uses libc `malloc` / `free` and can be
driven by Hakorune-generated LD_PRELOAD shims.

Do not vendor the whole corpus unless a later paper/reproducibility decision
requires historical result archives or additional benchmark families.
