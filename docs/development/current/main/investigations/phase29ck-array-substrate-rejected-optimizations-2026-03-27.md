# Phase29ck Array Substrate Rejected Optimizations (2026-03-27)

## Scope

- current `phase-29ck` array substrate perf wave の rejected attempts を 1 本の rolling ledger に残す
- short summary だけを `phase-29ck/README.md` / `CURRENT_TASK.md` / `10-Now.md` に残し、exact evidence はここへ集約する
- current main judge は `WSL warmup=1 repeat=3` だよ

## Current Reading

- `kilo_kernel_small_hk` は `pure-first + compat_replay=none` で green
- string family はかなり前進した
- current main residual is array substrate hot path
- ただし blind lock/cache tweaks は mainline regressions を出したので、次は proof-vocabulary first に戻る

## Rejected Attempts

### 2026-03-27: `ArrayBox.items` `RwLock -> Mutex`

**Hypothesis**

- `ArrayBox` write hot path の lock overhead を減らせば `kilo_micro_array_getset` と main `kilo` の両方に効く

**Touched owner area**

- [mod.rs](/home/tomoaki/git/hakorune-selfhost/src/boxes/array/mod.rs)

**Commands**

```bash
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- micro:
  - `kilo_micro_array_getset = 46 ms`
- main:
  - `kilo_kernel_small_hk = 842 ms`

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- do not reopen lock swap alone
- next cut must carry stronger representation/proof information than a bare synchronization swap

### 2026-03-27: raw borrowed cache inside `with_array_box_borrowed(...)`

**Hypothesis**

- borrowed/raw cache hit を強くすれば `LocalKey::with` / handle-cache fixed cost を減らせる

**Touched owner area**

- [handle_cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/handle_cache.rs)

**Commands**

```bash
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- micro:
  - `kilo_micro_array_getset = 46 ms`
- main:
  - `kilo_kernel_small_hk = 841 ms`

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- keep `handle_cache` tweaks subordinate to a clearer proof/representation cut
- next exact front is integer-heavy array fast lane with staged proof vocabulary

## Historical Pre-Ledger Rejects

- array `len/push` borrowed follow-up
- direct-downcast store candidate

これらも rejected だったけれど、exact kept snapshot を ledger format で残していない。したがって current canonical rows には入れず、historical note としてだけ扱う。

## Current Next Step

1. lock staged `AOT-Core` proof vocabulary in docs
2. keep this ledger as the single reject log for the current array substrate wave
3. resume code only on the integer-heavy `ArrayBox.get/set/len` fast lane
