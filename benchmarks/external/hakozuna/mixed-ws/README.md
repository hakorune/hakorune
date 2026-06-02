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

## Contract

The fixture is for external-process allocator replacement checks. It is built
with `HZ3_BENCH_USE_CRT=1`, so the benchmark calls libc
`malloc` / `realloc` / `free` and can be driven by Hakorune-generated
LD_PRELOAD shims.

Do not vendor the whole hakozuna corpus unless a later paper/reproducibility
decision explicitly selects additional benchmark families.
