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

### 2026-03-27: array-local integer `value_class` profile

**Hypothesis**

- array-local `value_class=ScalarI64` proof を持てば、integer-heavy `ArrayBox.get/set/len` fast lane が trait/codec overhead を減らせる

**Touched owner area**

- [mod.rs](/home/tomoaki/git/hakorune-selfhost/src/boxes/array/mod.rs)
- [handle_cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/handle_cache.rs)

**Commands**

```bash
cargo test -q integer_value_class_ --lib
cargo test -q -p nyash_kernel array_ -- --nocapture
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- tests:
  - targeted root/unit and `nyash_kernel array_` tests were green
- micro:
  - `kilo_micro_array_getset = 47 ms`
- main:
  - `kilo_kernel_small_hk = 840 ms`
- note:
  - an earlier naive variant briefly showed `44 ms` micro, but mainline regressed much harder (`1114 ms`); the mixed-array short-circuit recheck removed the worst regression, but still failed to beat the accepted baseline

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- do not add per-store array-local proof bookkeeping unless it produces a clear whole-program win
- the next useful cut likely needs lower fixed cost in `array_slot_store_i64` / TLS path or a more explicit integer representation split

### 2026-03-27: integer shadow vector on `ArrayBox`

**Hypothesis**

- integer-only `ArrayBox` に `Vec<i64>` shadow を常時維持すれば、`get/set/len` の hot path を direct scalar 読み書きに寄せられる

**Touched owner area**

- [mod.rs](/home/tomoaki/git/hakorune-selfhost/src/boxes/array/mod.rs)
- [handle_cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/handle_cache.rs)

**Commands**

```bash
cargo test -q scalar_i64_shadow_ --lib
cargo test -q -p nyash_kernel array_ -- --nocapture
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- tests:
  - targeted shadow tests and `nyash_kernel array_` tests were green
- micro:
  - `kilo_micro_array_getset = 45 ms`
- main:
  - `kilo_kernel_small_hk = 837 ms`
- note:
  - accepted line before this attempt was `kilo_micro_array_getset = 47 ms`, `kilo_kernel_small_hk = 809 ms`
  - micro improved slightly, but shadow maintenance overhead still lost on whole-program cost

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- do not keep a second integer shadow structure unless the carrying seam can prove lower write maintenance cost
- next useful cut is still fixed-cost reduction in `array_slot_store_i64` / TLS path, not per-array proof bookkeeping

### 2026-03-27: authoritative `ArrayStorage::{Generic,I64}` split on `ArrayBox`

**Hypothesis**

- a single authoritative integer lane inside `ArrayBox` should beat shadow bookkeeping because `get/set/len` can stay scalar without maintaining a second sidecar structure

**Touched owner area**

- [mod.rs](/home/tomoaki/git/hakorune-selfhost/src/boxes/array/mod.rs)
- [handle_cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/handle_cache.rs)

**Commands**

```bash
cargo check -q
cargo test -q -p nyash_kernel array_ -- --nocapture
cargo test -q core13_array_boxcall_set_get -- --nocapture
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- tests:
  - `cargo check -q` green
  - targeted `nyash_kernel array_` tests green
  - `core13_array_boxcall_set_get` green
- micro:
  - `kilo_micro_array_getset = 46 ms`
- main:
  - `kilo_kernel_small_hk = 858 ms`
- note:
  - accepted line before this attempt was `kilo_micro_array_getset = 46-47 ms`, `kilo_kernel_small_hk = 809 ms`
  - the authoritative split did not buy measurable micro improvement, while main paid extra read-path cost on non-integer arrays

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- do not reopen `ArrayStorage::{Generic,I64}` as a broad internal split until the carry seam can avoid extra read crossings on generic/string arrays
- next exact front stays fixed-cost reduction in `array_slot_store_i64` / TLS path, or an AOT-side reduction of redundant array crossings

### 2026-03-27: intra-block array store-load forwarding in pure-first lowering

**Hypothesis**

- if pure-first lowering forwards a plain `ArrayBox/RuntimeDataBox.set(idx, i64)` result to the immediately following `get(idx)` in the same block, `kilo_micro_array_getset` should lose one substrate crossing without broad runtime changes

**Touched owner area**

- [hako_llvmc_ffi_pure_compile.inc](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc)

**Commands**

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- micro:
  - `kilo_micro_array_getset = 46 ms`
- main:
  - `kilo_kernel_small_hk = 887 ms`
- note:
  - redundant crossing forwarding did not move the integer micro at all
  - whole-program `kilo` regressed harder than the accepted `809 ms` line, so the compile-time forwarding rule is not a valid current-wave win

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- do not reopen local store-load forwarding without a clearer proof/effect contract
- current next front stays native fixed-cost reduction in `array_slot_store_i64` / TLS path

### 2026-03-27: `ArrayBox.items` `parking_lot::RwLock -> std::sync::RwLock`

**Hypothesis**

- if the current `LocalKey::with` hot symbol is largely `parking_lot` lock machinery, replacing `ArrayBox.items` with `std::sync::RwLock` should cut fixed-cost overhead on integer-heavy `get/set`

**Touched owner area**

- [mod.rs](/home/tomoaki/git/hakorune-selfhost/src/boxes/array/mod.rs)

**Commands**

```bash
cargo check -q
cargo test -q -p nyash_kernel array_ -- --nocapture
cargo test -q core13_array_boxcall_set_get -- --nocapture
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- tests:
  - `cargo check -q` green
  - targeted `nyash_kernel array_` tests green
  - `core13_array_boxcall_set_get` green
- micro:
  - `kilo_micro_array_getset = 69 ms`
- main:
  - `kilo_kernel_small_hk = 872 ms`
- note:
  - accepted line before this attempt was `kilo_micro_array_getset = 46-47 ms`, `kilo_kernel_small_hk = 809 ms`
  - the lock implementation swap regressed both the integer micro and whole-program `kilo`, so the current hot `LocalKey::with` cost is not solved by replacing `parking_lot` with `std::sync`

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- do not reopen lock-implementation swaps on `ArrayBox.items` in the current wave
- next exact front remains fixed-cost reduction in `array_slot_store_i64` / TLS path, with owner confirmation below the lock implementation level

### 2026-03-27: `host_handles.table` `parking_lot::RwLock -> std::sync::RwLock`

**Hypothesis**

- if the remaining `LocalKey::with` hot symbol is mainly registry lock machinery under `with_array_box_borrowed(...)`, replacing the global host-handle table lock with `std::sync::RwLock` should reduce fixed-cost overhead on pure array substrate calls

**Touched owner area**

- [host_handles.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/host_handles.rs)

**Commands**

```bash
cargo check -q
cargo test -q -p nyash_kernel array_ -- --nocapture
cargo test -q core13_array_boxcall_set_get -- --nocapture
NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/run_kilo_micro_machine_ladder.sh 1 3
PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 3
```

**Observed result**

- tests:
  - `cargo check -q` green
  - targeted `nyash_kernel array_` tests green
  - `core13_array_boxcall_set_get` green
- micro:
  - `kilo_micro_array_getset = 68 ms`
- main:
  - `kilo_kernel_small_hk = 909 ms`
- note:
  - accepted line before this attempt was `kilo_micro_array_getset = 46-47 ms`, `kilo_kernel_small_hk = 809 ms`
  - the registry lock swap regressed even harder than the `ArrayBox.items` lock swap, so the current fixed cost is not explained by `parking_lot` on the host-handle table alone

**Verdict**

- rejected
- reverted immediately

**Next candidate**

- do not reopen lock-implementation swaps on `host_handles.table` in the current wave
- next exact front remains direct fixed-cost reduction inside `array_slot_store_i64` / `array_slot_load_hi`, or crossing-count reduction above the runtime substrate

## Historical Pre-Ledger Rejects

- array `len/push` borrowed follow-up
- direct-downcast store candidate

これらも rejected だったけれど、exact kept snapshot を ledger format で残していない。したがって current canonical rows には入れず、historical note としてだけ扱う。

## Current Next Step

1. lock staged `AOT-Core` proof vocabulary in docs
2. keep this ledger as the single reject log for the current array substrate wave
3. resume code only on the integer-heavy `ArrayBox.get/set/len` fast lane
